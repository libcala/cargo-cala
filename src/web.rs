use super::run;

use cala_web::WebServer;
use pasts::prelude::*;

use std::io::Read;
use std::io::Write;

use inotify::{Inotify, WatchMask};

fn build_loop() {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    inotify
        .add_watch(
            "src",
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        )
        .expect("Failed to add inotify watch");

    let mut buffer = [0u8; 4096];
    loop {
        run(
            "cargo",
            &[
                "rustc",
                "--target",
                "wasm32-unknown-unknown",
                "--release",
                "--",
                "-C",
                "link-arg=-o",
                "-C",
                "link-arg=target/cargo-cala/web/out/app.wasm",
                "-C",
                "lto=fat",
                "-C",
                "opt-level=s",
                "-C",
                "debuginfo=0",
                "-C",
                "panic=abort",
            ],
        );

        let mut config = walrus::ModuleConfig::new();
        config.generate_name_section(false);
        config.generate_producers_section(false);
        let buf = std::fs::read("target/cargo-cala/web/out/app.wasm").unwrap();
        let mut module = config.parse(&buf).unwrap();

        let mut customs = vec![];
        for custom in module.customs.iter() {
            customs.push(custom.0);
        }
        for custom in customs.drain(..) {
            module.customs.delete(custom);
        }

        walrus::passes::gc::run(&mut module);

        module.name = None;
        module
            .producers
            .add_processed_by(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        module
            .emit_wasm_file("target/cargo-cala/web/out/app.wasm")
            .unwrap();

        let mut file = std::fs::File::create("target/cargo-cala/web/out/index.html").unwrap();

        let in_data = if let Ok(mut in_file) = std::fs::File::open("res/name.txt") {
            let mut in_data = String::new();
            in_file.read_to_string(&mut in_data).unwrap();
            in_data
        } else {
            "Cala Project".to_string()
        };

        write!(file, include!("res/html.rs"), in_data).unwrap();

        let mut file = std::fs::File::create("target/cargo-cala/web/out/icon.svg").unwrap();

        let in_data = if let Ok(mut in_file) = std::fs::File::open("res/icon.svg") {
            let mut in_data = vec![];
            in_file.read_to_end(&mut in_data).unwrap();
            in_data
        } else {
            include_bytes!("res/icon.svg").to_vec()
        };

        file.write_all(&in_data).unwrap();

        inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");
    }
}

pub(super) fn web() {
    println!("Building for Web…");

    std::fs::create_dir_all("target/cargo-cala/web/ffi").unwrap();
    std::fs::create_dir_all("target/cargo-cala/web/out").unwrap();

    {
        let _file = std::fs::File::create("target/cargo-cala/web/out/index.html").unwrap();
    }

    std::thread::spawn(build_loop);

    let ip_port = "0.0.0.0:8080";
    println!("Running on {}…", ip_port);

    pasts::ThreadInterrupt::block_on(WebServer::with_resources("target/cargo-cala/web/out/"));
}
