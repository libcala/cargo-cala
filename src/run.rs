use std::{fs::File, collections::HashMap, path::{Path, PathBuf}};

// Debug targets
mod web;

// Release targets
mod linux;

/// A dependency
#[derive(Serialize, Deserialize)]
struct Dependency {
    /// Name of dependency
    name: String,
    /// Semver version
    version: Option<String>,
    /// Local path (to not use crates.io)
    path: Option<String>,
    /// Features to enable (all are off by default)
    feature: Vec<String>,
}

/// Cala.muon configuration file.
#[derive(Serialize, Deserialize)]
struct Cala {
    /// Edition (Example: 2018)
    edition: String,
    /// Semver version: "0.1.0-alpha.1" => "0.1.0-beta.1" => "0.1.0-pre.1" =>
    /// "0.1.0-rc.1" => "0.1.0"
    version: String,
    /// A webpage describing the project (Example:
    /// https://cargo-cala.libcala.github.io/).
    webpage: String,
    /// Path to a file of translations for the name of the app `res/Label.muon`.
    label: Option<String>,
    /// Path to the application's icon (Example: res/icon.svg).
    icon: Option<String>,
    /// List of authors (Example: Jeron Aldaron Lau <jeronlau@plopgrizzly.com>).
    author: Vec<String>,
    /// List of keywords associated with the distribution package.
    keyword: Vec<String>,
    /// Path to a file of translations for a sentence describing the app.
    comment: Option<String>,
    /// List of mime types that this application can open.
    mime: Vec<String>,
    /// List of categories from platform-agnostic list:
    /// - `edit`: Apps for creating/editing art or other multimedia
    /// - `edit::graphics`: Apps for creating/editing graphics.
    /// - `edit::video`: Apps for creating/editing video.
    /// - `edit::audio`: Apps for creating/editing audio and music.
    /// - `edit::cad`: Apps for computer aided design of 3D objects.
    /// - `edit::office`: Apps for editing documents/presentations/etc.
    /// - `edit::code`: Apps for editing and/or testing code (programming)
    /// - `play`: Apps for viewing/playing movies, music, art, other multimedia
    /// - `play::game`: Apps for playing video games
    /// - `info`: Apps to report information (weather/maps, news, etc.)
    /// - `info::learn`: Apps for learning
    /// - `social`: Apps for socializing (social media, teleconferencing, etc.)
    /// - `organize`: Apps for organizing your data locally
    /// - `internet`: Apps for transfering data over the web (pages, files, ...)
    /// - `science`: Simulations, space observation, etc.
    category: Vec<String>,
    /// List of dependencies
    deps: Vec<Dependency>,
}

impl Cala {
    fn new() -> muon_rs::Result<Cala> {
        let mut path = std::env::current_dir().unwrap();
        path.push("cala.muon");
        muon_rs::from_reader(File::open(&path).unwrap_or_else(|_| panic!("Couldn't read file: {:?}", path)))
    }
}

/// Convert a URL () to a package name.
fn url_to_packagename(url: &str) -> String {
    let mut packagename = String::new();

    // Leave off beginning protocol, if any
    let url = if url.starts_with("https://") {
        url.get("https://".len()..).unwrap()
    } else if url.starts_with("http://") {
        url.get("http://".len()..).unwrap()
    } else {
        url.get(..).unwrap()
    };
    
    let (domain, url) = if let Some(index) = url.find('/') {
        (url.get(..index).unwrap(), url.get(index..))
    } else {
        (url, None)
    };

    if let Some(url) = url {
        for filename in url.rsplit('/') {
            let first = filename[0..].chars().next().unwrap();
            let last = filename[filename.len()-1..].chars().next().unwrap();
            if !(first.is_ascii_alphabetic() || first.is_ascii_digit())
                || !(last.is_ascii_alphabetic() || last.is_ascii_digit())
            {
                panic!("Invalid webpage: - at beginning or end of label");
            }
            packagename.push_str(filename);
            packagename.push('.');
        }
    }
    
    for label in domain.rsplit('.') {
        let first = label[0..].chars().next().unwrap();
        let last = label[label.len()-1..].chars().next().unwrap();
        if !(first.is_ascii_alphabetic() || first.is_ascii_digit())
            || !(last.is_ascii_alphabetic() || last.is_ascii_digit())
        {
            panic!("Invalid webpage: - at beginning or end of label");
        }
        packagename.push_str(label);
        packagename.push('.');
    }
    
    // Convert to lowercase and remove trailing .
    let packagename = packagename[..packagename.len()-1].to_string();

    if !packagename.is_ascii() {
        panic!("Please use ascii to specify domain names: punycode conversion \
            not implemented");
    }

    packagename
}

fn generate_cargo_toml(cala: &Cala, crate_name: &str, path: &Path, extra_section: &str) -> PathBuf {
    let cargo_toml_path = path.join("Cargo.toml");
    let mut cargo_toml_data = format!("\
        [package]\n\
        name = \"{crate_name}\"\n\
        version = \"{crate_version}\"\n\
        authors = {crate_authors:?}\n\
        edition = \"{crate_edition}\"\n\
        homepage = \"{crate_homepage}\"\n\
        {extra_section}
        ",
        crate_edition = cala.edition,
        crate_version = cala.version,
        crate_homepage = cala.webpage,
        crate_name = crate_name,
        crate_authors = cala.author,
        extra_section = extra_section,
    );
    for dep in cala.deps.iter() {
        cargo_toml_data.push_str(&format!("[depedencies.{}]\n", dep.name));
        if let Some(ref version) = dep.version {
            cargo_toml_data.push_str(&format!("version = \"{}\"\n", version));
        }
        if let Some(ref path) = dep.path {
            cargo_toml_data.push_str(&format!("path = \"{}\"\n", path));
        }
        if !dep.feature.is_empty() {
            cargo_toml_data.push_str(&format!("features = {:?}\n", dep.feature));
        }
    }
    std::fs::write(&cargo_toml_path, cargo_toml_data).expect("Failed to write Cargo.toml");
    cargo_toml_path
}

struct Category {
    /// From https://specifications.freedesktop.org/menu-spec/latest/apa.html
    /// Leave out "Settings" and "System" as there is no equivalent category in
    /// cala, and if there were they'd require special privileges.
    freedesktop: &'static str,
}

// Convert Category
fn convert_category() -> HashMap<&'static str, Category> {
    let mut map = HashMap::new();

    map.insert("edit", Category { freedesktop: "AudioVideo" });
    map.insert("edit::graphics", Category { freedesktop: "Graphics" });
    map.insert("edit::video", Category { freedesktop: "Video;AudioVideo" });
    map.insert("edit::audio", Category { freedesktop: "Audio;AudioVideo" });
    map.insert("edit::cad", Category { freedesktop: "Graphics" });
    map.insert("edit::office", Category { freedesktop: "Office" });
    map.insert("edit::code", Category { freedesktop: "Development" });
    map.insert("play", Category { freedesktop: "AudioVideo" });
    map.insert("play::game", Category { freedesktop: "Game" });
    map.insert("info", Category { freedesktop: "Utility" });
    map.insert("info::learn", Category { freedesktop: "Education" });
    map.insert("social", Category { freedesktop: "Network" });
    map.insert("organize", Category { freedesktop: "Utility" });
    map.insert("internet", Category { freedesktop: "Network" });
    map.insert("science", Category { freedesktop: "Science" });

    map
}

pub(super) fn web() {
    web::web();
}

pub(super) fn run() {
    if cfg!(target_os = "linux") {
        linux::run()
    } else {
        todo!()
    }
}
