// bin/linux.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use RES_ICON;
use RES_SYMBOL;
use TARGET_DIR;

use file;
use program;
use parse;
use resources;

// Convert - to _ in domain name.
// https://lists.freedesktop.org/archives/flatpak/2016-October/000444.html
// "This is a problem for organizations with dashes in the domain-name, but they
//  will have to encode it with underscore instead."
pub fn domain(cargo_toml: &parse::CargoToml) -> String {
	cargo_toml.domain.replace("-", "_")
}

pub fn execute(cargo_toml: parse::CargoToml, translations: resources::Lang) {
	let name = &cargo_toml.subdomain;

	let flatpak = "target/flatpak/".to_string();
	let flatpak_app = {
		let mut x = flatpak.clone();
		x.push_str("app/");
		x
	};
	let flatpak_repo = {
		let mut x = flatpak.clone();
		x.push_str("repo/");
		x
	};
	let repo_name = "gsp-local";

	// Build the program
	{
		program::execute_log("cargo", vec![
			"build", "--release", "--bin", "gsp_program",
		]);
	}

	// App Domain
	let app_domain = format!("{}.{}", &domain(&cargo_toml),
		&cargo_toml.subdomain).to_string();

	// Copy Executable Into FlatPak
	{
		file::copy("target/release/gsp_program",
			&format!("target/flatpak/app/files/bin/{}", name))
			.unwrap();
	}

	// Copy Icon Into FlatPak
	{
		let dst = {
			let mut x = flatpak_app.clone();

			x.push_str("export/share/icons/hicolor/scalable/apps/");
			x.push_str(&app_domain);
			x.push_str(".png");
			x
		};
		let dst32 = {
			let mut x = flatpak_app.clone();

			x.push_str("export/share/icons/hicolor/32x32/apps/");
			x.push_str(&app_domain);
			x.push_str(".png");
			x
		};
		let dst48 = {
			let mut x = flatpak_app.clone();

			x.push_str("export/share/icons/hicolor/48x48/apps/");
			x.push_str(&app_domain);
			x.push_str(".png");
			x
		};

		file::copy(RES_ICON, &dst).unwrap();
		file::copy(RES_ICON, &dst32).unwrap();
		file::copy(RES_ICON, &dst48).unwrap();
	}

	// Copy Symbol Into FlatPak
	{
		let dst = {
			let mut x = flatpak_app.clone();

			x.push_str("export/share/icons/hicolor/symbolic/apps/");
			x.push_str(&app_domain);
			x.push_str(".svg");
			x
		};

		file::copy(RES_SYMBOL, &dst).unwrap();
	}

	// Create Metadata
	{
		let metadata_data = format!(
			"\
				[Application]\n\
				name={}\n\
				runtime=org.gnome.Platform/x86_64/3.24\n\
				command={}\n\
				\n\
				[Context]\n\
				shared=ipc;network;\n\
				sockets=x11;wayland;pulseaudio;\n\
				devices=dri;\n\
				filesystems=host;\n\
			",
			&app_domain, &cargo_toml.subdomain);

		let metadata_file = {
			let mut x = flatpak_app.clone();

			x.push_str("metadata");
			x
		};

		file::save(&metadata_file, metadata_data.as_bytes());
	}

	// Create .desktop
	{
		let mut desktop_data = format!(
			"\
				[Desktop Entry]\n\
				Type=Application\n\
				Encoding=UTF-8\n\
				Name={}\n\
				Exec={}\n\
				Icon={}\n\
				Terminal=false\n",
			&cargo_toml.subdomain, &cargo_toml.subdomain,
			&app_domain);

		// localize / english
		if let Some(ref en) = translations.en {
			desktop_data.push_str(
				&format!("Name[en]={}\n", en.0 .0)
			);
			desktop_data.push_str(
				&format!("Comment={}\nComment[en]={}\n",
					en.0 .1, en.0 .1)
			);
			// TODO: Dialects
		} else {
			program::exit("No English Translation.");
		}

		let desktop_file = {
			let mut x = flatpak_app.clone();

			x.push_str("export/share/applications/");
			x.push_str(&app_domain);
			x.push_str(".desktop");
			x
		};

		file::save(&desktop_file, desktop_data.as_bytes());
	}

	// Set up Repo
	{
		program::execute("flatpak build-export", "flatpak",
			vec!["build-export", &flatpak_repo, &flatpak_app],
			"flatpak not found!", "flatpak build-export failed!");

		program::execute("flatpak remote-add", "flatpak",
			vec!["--user", "remote-add", "--no-gpg-verify",
				"--if-not-exists", repo_name, &flatpak_repo],
			"flatpak not found(2)!", "flatpak remote-add failed!");

		program::execute("flatpak install", "flatpak",
			vec!["--user", "install", repo_name, &app_domain],
			"flatpak not found(3)!", "flatpak install failed!");

		program::execute("flatpak update", "flatpak",
			vec!["--user", "update", &app_domain],
			"flatpak not found(4)!", "flatpak update failed!");

		program::execute_log("flatpak", vec!["run", &app_domain]);
	}
}
