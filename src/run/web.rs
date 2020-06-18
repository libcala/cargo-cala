use std::{io::{Read, Write}, path::Path, process::{Command, Stdio}, str::FromStr};
use tide::{Response, Error as TideError, StatusCode, http::Mime};
use inotify::{Inotify, WatchMask};
use devout::out;

// Tags for prints.
const TAG: &str = "cargo-cala/web";
// Linux FlatPak target directory.
const PATH: &str = "./target/.cala/web/";

fn build_loop() {
    out!(TAG, "Beginning Debugging For Web…");
    let cala = super::Cala::new().expect("Couldn't parse `cala.muon`!");
    let packagename = super::url_to_packagename(&cala.webpage);
    let crate_name = packagename.get(..packagename.find('.').expect("bad packagename")).unwrap();
    // Paths
    let cargo_out = Path::new("./target").join("wasm32-unknown-unknown").join("release").join(crate_name);
    let path = Path::new(PATH);
    let app = path.join("app");
    let app_bin = app.join(crate_name);

    out!(TAG, "Initialize File Watching…");
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");
    inotify
        .add_watch(
            "src",
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        )
        .expect("Failed to add inotify watch");
    let mut buffer = [0u8; 4096];

    out!(TAG, "Generating Cargo.toml…");
    std::fs::create_dir_all(path).expect("Failed to make wasm32 directory");
    let cargo_toml_lib = format!("[lib]\n\
    crate-type = [\"cdylib\"]\n\
    path = \"src/{crate_name}.rs\"\n\
    ", crate_name = crate_name);
    let cargo_toml_path = super::generate_cargo_toml(&cala, crate_name, &path, &cargo_toml_lib);

    loop {
        out!(TAG, "Building cargo package \"{}\"…", crate_name);
        Command::new("cargo")
            .arg("rustc")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--debug")
            .arg("--lib")
            .arg("--manifest-path")
            .arg(&cargo_toml_path)
            .arg("--")
            .arg("-C")
            .arg("lto=fat")
            .arg("-C")
            .arg("opt-level=s")
            .arg("-C")
            .arg("debuginfo=0")
            .arg("-C")
            .arg("panic=abort")
            .stdout(Stdio::inherit())
            .stdin(Stdio::null())
            .output()
            .expect("Failed to build with Cargo");
        out!(TAG, "Copying wasm binary into static hosting…");
        if std::fs::copy(&cargo_out, &app_bin).is_ok() {
        let mut config = walrus::ModuleConfig::new();
        config.generate_name_section(false);
        config.generate_producers_section(false);
        let buf = std::fs::read(&format!("target/cargo-cala/web/out/{}.wasm", crate_name)).unwrap();
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
            .emit_wasm_file(&format!("target/cargo-cala/web/out/{}.wasm", crate_name))
            .unwrap();

        let mut file = std::fs::File::create("target/cargo-cala/web/out/index.html").unwrap();

        let in_data = if let Ok(mut in_file) = std::fs::File::open("res/name.txt") {
            let mut in_data = String::new();
            in_file.read_to_string(&mut in_data).unwrap();
            in_data
        } else {
            "Cala Project".to_string()
        };

        write!(file, include_str!("../res/index.html"), in_data, name = crate_name).unwrap();

        let mut file = std::fs::File::create("target/cargo-cala/web/out/logo.svg").unwrap();

        let in_data = if let Ok(mut in_file) = std::fs::File::open("res/logo.svg") {
            let mut in_data = vec![];
            in_file.read_to_end(&mut in_data).unwrap();
            in_data
        } else {
            include_bytes!("../res/logo.svg").to_vec()
        };

        file.write_all(&in_data).unwrap();

        }

        println!("Waiting for events...");
        inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");
        println!("Got an event!");
    }
}

async fn read_file(url: &str) -> Result<Response, TideError> {
    let path = format!("./target/cargo-cala/web/out{}", url);
    dbg!(&path);
    let out = async_std::fs::read(&path)
        .await
        .map_err(|e| TideError::new(StatusCode::NotFound, e))?;
    let mut response = Response::new(StatusCode::Ok);
    match path {
        a if a.ends_with(".html") => {
            response.set_content_type(Mime::from_str("text/html;charset=utf-8").unwrap());
        }
        a if a.ends_with(".wasm") => {
            response.set_content_type(Mime::from_str("application/wasm").unwrap());
        }
        /*a if a.ends_with(".svg") => {
            "image/svg+xml"
        }*/
        _ => {}// Mime::from_str("text/plain;charset=utf-8").unwrap() }
    }
    response.set_body(out);
    Ok(response)
}

async fn index(_req: tide::Request<()>) -> Result<Response, TideError> {
    dbg!(_req.content_type());
    let res = read_file("/index.html").await;
    if res.is_err() {
        read_file("/404.html").await
    } else {
        res
    }
}

async fn handle_event(req: tide::Request<()>) -> Result<Response, TideError> {
    dbg!(req.content_type());
    let res = read_file(req.url().path()).await;
    if res.is_err() {
        read_file("/404.html").await
    } else {
        res
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

    let ip_port = {
        let mut ip = whoami::hostname();
        ip.push_str(":8000");
        ip
    };
    println!("Running on http://{}…", ip_port);

    let future = async {
        // tide::log::start();
        let mut app = tide::new();
        app.at("/").get(index);
        app.at("/*path").get(handle_event);
        app.listen(ip_port).await.unwrap();
    };

    async_std::task::block_on(future);
}
