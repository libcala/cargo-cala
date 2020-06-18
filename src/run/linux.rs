// FIXME: Move stuff out of this module to be shared.

use std::{path::Path, process::{Command, Stdio}};
use devout::out;

// Tags for prints.
const TAG: &str = "cargo-cala/linux";
// Linux FlatPak target directory.
const PATH: &str = "./target/.cala/linux/";

struct Arch {
    /// The cala name for the arch
    cala: &'static str,
    /// The Rust target name
    rust: &'static str,
    /// The FlatPak target name
    flatpak: &'static str,
}

static ARM_32: Arch = Arch { cala: "arm_32", rust: "thumbv7neon-unknown-linux-gnueabihf", flatpak: "arm" };
static ARM_64: Arch = Arch { cala: "arm_64", rust: "aarch64-unknown-linux-gnu", flatpak: "aarch64" };
static X86_32: Arch = Arch { cala: "x86_32", rust: "i586-unknown-linux-gnu", flatpak: "i386" };
static X86_64: Arch = Arch { cala: "x86_64", rust: "x86_64-unknown-linux-gnu", flatpak: "x86_64" };

// Install the freedesktop Platform.
fn install_freedesktop(arch: &Arch) {
    Command::new("flatpak")
        .arg("remote-add")
        .arg("--if-not-exists")
        .arg("flathub")
        .arg("https://flathub.org/repo/flathub.flatpakrepo")
        .stdout(Stdio::inherit())
        .stdin(Stdio::null())
        .output()
        .expect("Failed to connect to FlatHub");
        
    Command::new("flatpak")
        .arg("install")
        .arg("-y")
        .arg("flathub")
        .arg(&format!("org.freedesktop.Platform/{}/18.08", arch.flatpak))
        .stdout(Stdio::inherit())
        .stdin(Stdio::null())
        .output()
        .expect("Failed to install org.freedesktop.Platform");
}

// Build a FlatPak for an architecture
fn build_for_arch(root: &Path, cargo_toml_path: &Path, arch: &Arch, crate_name: &str, packagename: &str) {
    // Install dependant runtime
    install_freedesktop(arch);
    // Create folders
    let repo = root.join("repo");
    let root = root.join(arch.cala);
    let app = root.join("app");
    let bin = app.join("files/bin");
    let icon = app.join("export/share/icons/hicolor/scalable/apps");
    let export = app.join("export/share/applications");
    std::fs::create_dir_all(&bin).expect("Failed to make bin directory");
    std::fs::create_dir_all(&icon).expect("Failed to make icon directory");
    std::fs::create_dir_all(&export).expect("Failed to make export directory");
    std::fs::create_dir_all(&repo).expect("Failed to make repo directory");
    // Other paths
    let cargo_out = Path::new("./target").join(arch.rust).join("release").join(crate_name);
    let app_bin = bin.join(crate_name);
    let metadata = bin.join("app/metadata");
    let desktop = export.join(&format!("{}.desktop", packagename));

    out!(TAG, "Building cargo package \"{}\"…", crate_name);
    Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg(arch.rust)
        .arg("--release")
        .arg("--bin")
        .arg(crate_name)
        .arg("--manifest-path")
        .arg(cargo_toml_path)
        .stdout(Stdio::inherit())
        .stdin(Stdio::null())
        .output()
        .expect("Failed to build with Cargo");

    out!(TAG, "Copying binary into FlatPak…");
    std::fs::copy(cargo_out, app_bin).expect("Failed to copy app binary");

    out!(TAG, "Generating flatpak metadata…");
    std::fs::write(
        metadata,
        format!("\
            [Application]\n\
            name={packagename}/{arch}//\n\
            runtime=org.freedesktop.Platform/{arch}/18.08\n\
            command={crate_name}\n\
            \n\
            [Context]\n\
            shared=ipc;network;\n\
            sockets=x11;wayland;pulseaudio;\n\
            devices=dri;\n\
            filesystems=host;\n\
            ",
            crate_name = crate_name,
            packagename = packagename,
            arch = arch.flatpak,
        ),
    ).expect("Failed to write metadata");

    out!(TAG, "Generating .desktop launcher and translations…");
    let mut desktop_data: String = format!(
        "\
            [Desktop Entry]\n\
            Type=Application\n\
            Name={crate_name}\n\
            Exec={crate_name}\n\
            Icon={packagename}\n\
            Terminal=false\n\
        ",
        crate_name = crate_name,
        packagename = packagename,
    );
    // FIXME: Translations
    desktop_data.push_str("");
    /*if let Some(description_en) = app_description_en {
        desktop.push_str(&format!(
            "Name={app_name_en}\n\
             Comment[en]={app_description_en}\n",
            app_description_en = description_en
        ));
    }*/
    std::fs::write(desktop, desktop_data).unwrap();
    
    out!(TAG, "Building FlatPak…");
    Command::new("flatpak")
        .arg("build-export")
        .arg(repo)
        .arg("target/cargo_cala/run/app/")
        .stdout(Stdio::inherit())
        .stdin(Stdio::null())
        .output()
        .expect("Failed to run flatpak");
}

pub(super) fn run() {
    out!(TAG, "Beginning FlatPak Build…");
    let cala = super::Cala::new().expect("Couldn't parse `cala.muon`!");
    let packagename = super::url_to_packagename(&cala.webpage);
    let crate_name = packagename.get(..packagename.find('.').expect("bad packagename")).unwrap();

    // Create Cargo.toml
    out!(TAG, "Generating Cargo.toml…");
    let path = Path::new(PATH);
    std::fs::create_dir_all(path).expect("Failed to make flatpak directory");
    let cargo_toml_bin = format!("[[bin]]\n\
    name = \"{crate_name}\"\n\
    path = \"src/{crate_name}.rs\"\n\
    ", crate_name = crate_name);
    let cargo_toml_path = super::generate_cargo_toml(&cala, crate_name, &path, &cargo_toml_bin);

    // Build flatpak for 4 architectures
    build_for_arch(&path, &cargo_toml_path, &ARM_32, crate_name, &packagename);
    build_for_arch(&path, &cargo_toml_path, &ARM_64, crate_name, &packagename);
    build_for_arch(&path, &cargo_toml_path, &X86_32, crate_name, &packagename);
    build_for_arch(&path, &cargo_toml_path, &X86_64, crate_name, &packagename);

    out!(TAG, "Adding FlatPak repository…");
    Command::new("flatpak")
        .arg("--user")
        .arg("remote-add")
        .arg("--no-gpg-verify")
        .arg("--if-not-exists")
        .arg(&packagename)
        .arg("target/cargo_cala/run/repo/")
        .stdout(Stdio::inherit())
        .stdin(Stdio::null())
        .output()
        .expect("Failed to run flatpak");
        
    out!(TAG, "Installing FlatPak…");
    Command::new("flatpak")
        .arg("--user")
        .arg("install")
        .arg("--reinstall")
        .arg("--noninteractive")
        .arg(&packagename)
        .arg(&packagename)
        .arg("-y")
        .stdout(Stdio::inherit())
        .stdin(Stdio::null())
        .output()
        .expect("Failed to run flatpak");
}
