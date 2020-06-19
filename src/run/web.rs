use std::{io::Write, path::Path, process::{Command, Stdio}, str::FromStr, sync::mpsc::channel, time::Duration};
use tide::{Response, Error as TideError, StatusCode, http::Mime};
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use devout::out;

// Tags for prints.
const TAG: &str = "cargo-cala/web";
// Linux FlatPak target directory.
const PATH: &str = "./target/.cala/web/";

fn build_loop(url: &str) {
    let cala = super::Cala::new().expect("Couldn't parse `cala.muon`!");
    let packagename = super::url_to_packagename(&cala.webpage);
    let crate_name = packagename.get(..packagename.find('.').expect("bad packagename")).unwrap();
    // Paths
    let cargo_out = Path::new("./target").join("wasm32-unknown-unknown").join("debug").join(&format!("{}.wasm", crate_name));
    let path = Path::new(PATH);
    let app = path.join("app");
    let app_bin = app.join(&format!("{}.wasm", crate_name));

    out!(TAG, "Initialize File Watching…");
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(250)).unwrap();
    let curdir = std::env::current_dir().unwrap();
    watcher.watch(&curdir, RecursiveMode::Recursive).unwrap();
    let recompile = |string: &str| {
        let path = string.trim_start_matches(curdir.to_str().unwrap());
        !(path.starts_with("/target") || path == "/README.md")
    };

    out!(TAG, "Generating Cargo.toml…");
    std::fs::create_dir_all(path).expect("Failed to make wasm32 directory");
    let cargo_toml_lib = format!("[lib]\n\
    crate-type = [\"cdylib\"]\n\
    path = \"../../../src/{crate_name}.rs\"\n\
    ", crate_name = crate_name);
    let cargo_toml_path = super::generate_cargo_toml(&cala, crate_name, &path, &cargo_toml_lib);

    loop {
        // Ignore errors, maybe the file doesn't exist and we don't care.
        let _ = std::fs::remove_file(&cargo_out);

        out!(TAG, "Building cargo package \"{}\"…", crate_name);
        Command::new("cargo")
            .arg("rustc")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--target-dir")
            .arg("./target/")
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
            .spawn()
            .expect("Failed to build with Cargo")
            .wait()
            .unwrap();
        if let Ok(buf) = std::fs::read(&cargo_out) {
        out!(TAG, "Copying wasm binary into static hosting…");
        let mut config = walrus::ModuleConfig::new();
        config.generate_name_section(false);
        config.generate_producers_section(false);
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

        module.emit_wasm_file(&app_bin).expect("Failed to emit WASM");

        // Write index.html
        let mut file = std::fs::File::create(app.join("index.html")).unwrap();
        let in_data = if let Some(ref label) = cala.label {
            let label_translations = std::fs::File::open(label).expect("label is an invalid path");
            let label: super::super::translator::Translations = muon_rs::from_reader(label_translations).expect("Failed read on label");
            label.english().unwrap_or_else(|| "Cala Project").to_string()
        } else {
            "Cala Project".to_string()
        };
        write!(file, include_str!("../../res/index.html"), in_data, name = crate_name).unwrap();

        // Write favicon.svg
        let mut file = std::fs::File::create(app.join("favicon.svg")).unwrap();
        let in_data = if let Some(ref icon) = cala.icon {
            std::fs::read(icon).expect("icon is an invalid path")
        } else {
            include_bytes!("../../res/icon.svg").to_vec()
        };
        file.write_all(&in_data).unwrap();
        out!(TAG, "Ready and updated at http://{}!", url);
        }
        
        loop {
            use DebouncedEvent::*;
            match rx.recv().expect("Watch error, restart to continue") {
                NoticeWrite(_path) => {},
                NoticeRemove(_path) => {},
                Create(path) => if recompile(path.to_str().unwrap()) { break },
                Write(path) => if recompile(path.to_str().unwrap()) { break },
                Chmod(_path) => {},
                Remove(path) => if recompile(path.to_str().unwrap()) { break },
                Rename(path_a, path_b) => {
                    if recompile(path_a.to_str().unwrap()) { break }
                    if recompile(path_b.to_str().unwrap()) { break }
                },
                Rescan => {},
                Error(_, _) => {},
            }
        }
    }
}

async fn read_file(url: &str) -> (Result<Response, TideError>, bool) {
    let mut text = false;
    let path = Path::new(PATH).join(&format!("app{}", url));
    let out = async_std::fs::read(&path)
        .await
        .map_err(|e| {
            eprintln!("Failed Path: {:?}", path);
            TideError::new(StatusCode::NotFound, e)
        });
    let mut response = Response::new(StatusCode::Ok);
    match path.to_str().unwrap() {
        a if a.ends_with(".html") => {
            text = true;
            response.set_content_type(Mime::from_str("text/html;charset=utf-8").unwrap());
        }
        a if a.ends_with(".wasm") => {
            response.set_content_type(Mime::from_str("application/wasm").unwrap());
        }
        a if a.ends_with(".svg") => {
            response.set_content_type(Mime::from_str("image/svg+xml").unwrap());
        }
        e => {
            eprintln!("Unknown file ext: {}", e);
        }// Mime::from_str("text/plain;charset=utf-8").unwrap() }
    }
    match out {
        Ok(out) => {
            response.set_body(out);
            (Ok(response), text)
        }
        Err(err) => {
            (Err(err), text)
        }
    }
    
}

async fn index(_req: tide::Request<()>) -> Result<Response, TideError> {
    let (res, text) = read_file("/index.html").await;
    if res.is_err() && text {
        read_file("/404.html").await.0
    } else {
        res
    }
}

async fn handle_event(req: tide::Request<()>) -> Result<Response, TideError> {
    let (res, text) = read_file(req.url().path()).await;
    if res.is_err() && text {
        read_file("/404.html").await.0
    } else {
        res
    }
}

pub(super) fn web() {
    out!(TAG, "Beginning Debugging for Web…");

    std::fs::create_dir_all(Path::new(PATH).join("app")).unwrap();

    let ip_port = {
        let mut ip = whoami::hostname();
        ip.push_str(":8000");
        ip
    };

    let ip_port_copy = ip_port.clone();
    std::thread::spawn(move || build_loop(&ip_port_copy));

    let future = async {
        // tide::log::start();
        let mut app = tide::new();
        app.at("/").get(index);
        app.at("/*path").get(handle_event);
        app.listen(ip_port).await.unwrap();
    };

    async_std::task::block_on(future);
}
