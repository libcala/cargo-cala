// FIXME: Move stuff out of this module to be shared.

use std::path::Path;
use std::cell::RefCell;
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

    // Create directory structure
    std::fs::create_dir_all("./target/cargo_cala/run/app/files/bin/").unwrap();

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

    println!("Building flatpak…");
}
