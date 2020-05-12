// FIXME: Move stuff out of this module to be shared.

use std::path::Path;
use std::cell::RefCell;
use std::process::Command;
use cargo::util::config::Config;
use cargo::ops::CompileOptions;
use cargo::core::compiler::BuildConfig;
use cargo::core::InternedString;
use cargo::ops::Packages;
use cargo::ops::CompileFilter;
use cargo::util::process_builder::ProcessBuilder;
use cargo::core::package_id::PackageId;
use cargo::core::manifest::Target;
use cargo::core::compiler::CompileMode;
use cargo::util::errors::CargoResult;
use cargo::core::compiler::CompileKind;
use cargo::core::compiler::MessageFormat;
use cargo::core::Workspace;
use cargo::core::shell::Verbosity;
use cargo::core::manifest::EitherManifest;

struct Executor;

impl cargo::core::compiler::Executor for Executor {
    fn exec(
        &self, 
        cmd: ProcessBuilder, 
        _id: PackageId, 
        _target: &Target, 
        _mode: CompileMode, 
        on_stdout_line: &mut dyn FnMut(&str) -> CargoResult<()>, 
        on_stderr_line: &mut dyn FnMut(&str) -> CargoResult<()>
    ) -> CargoResult<()>
    {
        cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
            .map(drop)
    }
}

pub(super) fn run() {
    let ncpus = num_cpus::get() as u32;
    let executor: Box<dyn cargo::core::compiler::Executor> = Box::new(Executor);

    let mut path = std::env::current_dir().unwrap();
    path.push("Cargo.toml");
    
    let config = Config::default().unwrap();
    config.shell().set_verbosity(Verbosity::Normal);

    // Load package information
    let manifest = match cargo::util::toml::read_manifest(&path, cargo::core::SourceId::for_path(&std::env::current_dir().unwrap()).unwrap(), &config).unwrap().0 {
        EitherManifest::Real(manifest) => manifest,
        EitherManifest::Virtual(_) => panic!("Virtual manifest not supported!"),
    };
    let metadata = manifest.metadata();

    let app_name = manifest.name().as_str();
    let app_description = &metadata.description;
    let app_name_en = app_name; // FIXME
    let app_description_en = app_description; // FIXME
//    let app_company = custom_metadata();
    let app_company = "company"; // FIXME
    let app_domain = format!("cala.{}.{}", app_company, app_name);

    // Create directory structure
    std::fs::create_dir_all("./target/cargo_cala/run/app/files/bin/").unwrap();
    std::fs::create_dir_all("./target/cargo_cala/run/app/export/share/icons/hicolor/scalable/apps/").unwrap();
    std::fs::create_dir_all("./target/cargo_cala/run/app/export/share/applications/").unwrap();
    std::fs::create_dir_all("./target/cargo_cala/run/repo/").unwrap();

    // Invoke cargo
    cargo::ops::compile_with_exec(
        &Workspace::new(Path::new(&path), &config).unwrap(),
        &CompileOptions {
            config: &config,
            build_config: BuildConfig {
                requested_kind: CompileKind::Host /*Host target*/,
                jobs: ncpus,
                requested_profile: InternedString::new(""), // FIXME?
                mode: CompileMode::Build,
                message_format: MessageFormat::Human,
                force_rebuild: false,
                build_plan: false,
                primary_unit_rustc: None,
                rustfix_diagnostic_server: RefCell::new(None),
            },
            features: Vec::new(),
            all_features: false,
            no_default_features: false,
            spec: Packages::Default,
            filter: CompileFilter::Default { required_features_filterable: false },
            target_rustdoc_args: None,
            target_rustc_args: None,
            local_rustdoc_args: None,
            rustdoc_document_private_items: false,
            export_dir: Some("./target/cargo_cala/run/app/files/bin/".into()),
        },
        &executor.into()
    ).unwrap();
    
    println!("[cargo-cala] Building flatpak…");

    // Create Flatpak Files
    std::fs::write(
        format!("./target/cargo_cala/run/app/metadata"),
        format!(
            "[Application]
name={app_domain}
runtime=org.gnome.Platform/x86_64/3.32
command={app_name}

[Context]
shared=ipc;network;
sockets=x11;wayland;pulseaudio;
devices=dri;
filesystems=host;",
            app_name = app_name, app_domain = app_domain,
        )
    ).unwrap();
    
    let mut desktop: String = format!(
        "[Desktop Entry]
Type=Application
Name={app_name}
Exec={app_name}
Icon={app_domain}
Terminal=false

Name[en]={app_name_en}",
        app_name = app_name, app_name_en = app_name_en,
        app_domain = app_domain,
    );
    if let Some(description_en) = app_description_en {
        desktop.push_str(&format!("\nComment={app_description_en}
Comment[en]={app_description_en}", app_description_en = description_en));
    }
    
    std::fs::write(
        format!("./target/cargo_cala/run/app/export/share/applications/{app_domain}.desktop", app_domain = app_domain), desktop
    ).unwrap();

    // Invoke flatpak build
    Command::new("flatpak")
        .arg("build-export")
        .arg("target/cargo_cala/run/repo/")
        .arg("target/cargo_cala/run/app/")
        .spawn()
        .expect("Failed to run flatpak")
        .wait()
        .expect("Failed to terminate flatpak");
        
    // Install locally using project-specific repo
    println!("[cargo-cala] Installing…");
    Command::new("flatpak")
        .arg("--user")
        .arg("remote-add")
        .arg("--no-gpg-verify")
        .arg("--if-not-exists")
        .arg(&app_domain)
        .arg("target/cargo_cala/run/repo/")
        .spawn()
        .expect("Failed to run flatpak")
        .wait()
        .expect("Failed to terminate flatpak");
    Command::new("flatpak")
        .arg("--user")
        .arg("install")
        .arg("--reinstall")
        .arg("--noninteractive")
        .arg(&app_domain)
        .arg(&app_domain)
        .arg("-y")
        .spawn()
        .expect("Failed to run flatpak")
        .wait()
        .expect("Failed to terminate flatpak");
    
    // Run program
    println!("[cargo-cala] Running…");
    Command::new("flatpak")
        .arg("run")
        .arg(&app_domain)
        .spawn()
        .expect("Failed to run flatpak")
        .wait()
        .expect("Failed to terminate flatpak");
}
