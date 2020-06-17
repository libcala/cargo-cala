use cargo::core::compiler::BuildConfig;
use cargo::core::compiler::CompileKind;
use cargo::core::compiler::CompileMode;
use cargo::core::compiler::MessageFormat;
use cargo::core::manifest::EitherManifest;
use cargo::core::manifest::Target;
use cargo::core::package_id::PackageId;
use cargo::core::shell::Verbosity;
use cargo::core::InternedString;
use cargo::core::Workspace;
use cargo::ops::CompileFilter;
use cargo::ops::CompileOptions;
use cargo::ops::Packages;
use cargo::util::config::Config;
use cargo::util::errors::CargoResult;
use cargo::util::process_builder::ProcessBuilder;
use std::cell::RefCell;
use cargo::core::compiler::CompileTarget;

use std::io::Read;
use std::io::Write;
use std::path::{Path};
use std::str::FromStr;

use tide::Response;
use tide::Error as TideError;
use tide::StatusCode;
use tide::http::Mime;

use inotify::{Inotify, WatchMask};

struct Executor;

impl cargo::core::compiler::Executor for Executor {
    fn exec(
        &self,
        cmd: ProcessBuilder,
        _id: PackageId,
        _target: &Target,
        _mode: CompileMode,
        on_stdout_line: &mut dyn FnMut(&str) -> CargoResult<()>,
        on_stderr_line: &mut dyn FnMut(&str) -> CargoResult<()>,
    ) -> CargoResult<()> {
        cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
            .map(drop)
    }
}

fn build_loop() {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    inotify
        .add_watch(
            "src",
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        )
        .expect("Failed to add inotify watch");

    let mut buffer = [0u8; 4096];

    // Setup Cargo
    let ncpus = num_cpus::get() as u32;
    let mut path = std::env::current_dir().unwrap();
    path.push("Cargo.toml");
    let config = Config::default().unwrap();
    config.shell().set_verbosity(Verbosity::Normal);
    let manifest = match cargo::util::toml::read_manifest(
        &path,
        cargo::core::SourceId::for_path(&std::env::current_dir().unwrap()).unwrap(),
        &config,
    )
    .unwrap()
    .0
    {
        EitherManifest::Real(manifest) => manifest,
        EitherManifest::Virtual(_) => panic!("Virtual manifest not supported!"),
    };
    // let metadata = manifest.metadata();
    let app_name = manifest.name().as_str();

    loop {
        let executor: Box<dyn cargo::core::compiler::Executor> = Box::new(Executor);
        let result = cargo::ops::compile_with_exec(
            &Workspace::new(Path::new(&path), &config).unwrap(),
            &CompileOptions {
                build_config: BuildConfig {
                    requested_kind: CompileKind::Target(CompileTarget::new("wasm32-unknown-unknown").unwrap()),
                    jobs: ncpus,
                    requested_profile: InternedString::new(""), // FIXME?
                    mode: CompileMode::Build,
                    message_format: MessageFormat::Human,
                    force_rebuild: false,
                    build_plan: false,
                    primary_unit_rustc: None,
                    rustfix_diagnostic_server: RefCell::new(None),
                    unit_graph: false,
                },
                features: Vec::new(),
                all_features: false,
                no_default_features: false,
                spec: Packages::Default,
                filter: CompileFilter::Default {
                    required_features_filterable: false,
                },
                target_rustdoc_args: None,
                target_rustc_args: Some(vec![
                    "-C".to_string(),
                    "lto=fat".to_string(),
                    "-C".to_string(),
                    "opt-level=s".to_string(),
                    "-C".to_string(),
                    "debuginfo=0".to_string(),
                    "-C".to_string(),
                    "panic=abort".to_string(),
                ]),
                local_rustdoc_args: None,
                rustdoc_document_private_items: false,
                export_dir: Some("./target/cargo-cala/web/out/".into()),
            },
            &executor.into(),
        );

        if result.is_ok() {
        let mut config = walrus::ModuleConfig::new();
        config.generate_name_section(false);
        config.generate_producers_section(false);
        let buf = std::fs::read(&format!("target/cargo-cala/web/out/{}.wasm", app_name)).unwrap();
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
            .emit_wasm_file(&format!("target/cargo-cala/web/out/{}.wasm", app_name))
            .unwrap();

        let mut file = std::fs::File::create("target/cargo-cala/web/out/index.html").unwrap();

        let in_data = if let Ok(mut in_file) = std::fs::File::open("res/name.txt") {
            let mut in_data = String::new();
            in_file.read_to_string(&mut in_data).unwrap();
            in_data
        } else {
            "Cala Project".to_string()
        };

        write!(file, include_str!("res/index.html"), in_data, name = app_name).unwrap();

        let mut file = std::fs::File::create("target/cargo-cala/web/out/logo.svg").unwrap();

        let in_data = if let Ok(mut in_file) = std::fs::File::open("res/logo.svg") {
            let mut in_data = vec![];
            in_file.read_to_end(&mut in_data).unwrap();
            in_data
        } else {
            include_bytes!("res/logo.svg").to_vec()
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
