use super::run;
use actix_web;
use actix_files;
use walrus;

use actix_web::{HttpServer, App as ActixApp, HttpRequest, web};
use actix_files as fs;

use std::io::Write;

use inotify::{
    WatchMask,
    Inotify,
};

fn build_loop() {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    inotify.add_watch("src", WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE).expect("Failed to add inotify watch");

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
                "-C", "link-arg=-o",
                "-C", "link-arg=target/cargo-cala/web/out/app.wasm",
                "-C", "lto=fat",
                "-C", "opt-level=s",
                "-C", "debuginfo=0",
                "-C", "panic=abort",
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
        module.producers.add_processed_by(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        module.emit_wasm_file("target/cargo-cala/web/out/app.wasm").unwrap();

        let mut file = std::fs::File::create("target/cargo-cala/web/out/index.html").unwrap();

        write!(file, include!("res/html.rs"), "Name").unwrap();

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

    let server = HttpServer::new(move ||
        ActixApp::new()
            .service(web::resource("/").route(
                web::get().to(|_req: HttpRequest| {
                    fs::NamedFile::open("./target/cargo-cala/web/out/index.html")
                }
            )))
            .service(web::resource("/{page}").route(
                web::get().to(|_req: HttpRequest, path: web::Path<(String,)>| {
                    fs::NamedFile::open(format!("./target/cargo-cala/web/out/{}", path.0))
                }
            )))
    );

    server.bind(ip_port).unwrap().run().unwrap();
}
