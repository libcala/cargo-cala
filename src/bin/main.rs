// bin/main.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

pub(crate) extern crate rand;
pub(crate) extern crate toml;
// extern crate abxml;

use std::env;

// mod android;
pub(crate) mod cli;
pub(crate) mod file;
pub(crate) mod parse;
pub(crate) mod password;
pub(crate) mod program;
pub(crate) mod resources;

mod ndk;
mod linux;

pub(crate) const PASSWORD: &'static str = ".packager/password.text";
pub(crate) const RES_ICON: &'static str = "res/icon.png";
pub(crate) const RES_SYMBOL: &'static str = "res/symbol.svg";

const USAGE: &'static str = "\
Build a software release package, install, and run

Usage:
	cargo gsp <target>

Targets:
	linux         Build a Linux FlatPak
	android       Build an Android APK                              **TODO**
	windows       Build an Installer                                **TODO**
	macos         Build a MacOS .app Package                        **TODO**
	web           Build an HTML Webpage With WASM and JS support    **TODO**
	arduino       Build an Arduino Executable                       **TODO**
	raspberrypi   Build a .img To Flash On a Micro SD Card          **TODO**
	ios           Build an IOS Application Package                  **TODO**
	switch        Build a Nintendo Switch Application               **TODO**
	aldaronsos    Build an Aldaron's OS Software Package            **TODO**

Note:
	The Install and Run Steps Will Be Skipped If The Target Environment Is \
	Not Available.
";

fn help() {
	cli::print(USAGE);
}

fn main() {
	let args: Vec<_> = env::args().collect();

	if args[1] != "gsp" {
		cli::print("\nPlease use `cargo gsp` instead of `cargo-gsp`")
	} else if args.len() != 3 || args[2] == "-h" {
		help()
	} else {
		// Read Cargo.toml
		let cargo_toml = parse::execute();
		let translations = resources::execute(&cargo_toml);

		match args[2].as_str() {
			"ndk" => ndk::execute(cargo_toml, translations),
			"linux" => linux::execute(cargo_toml, translations),
//			"android" => android::execute(),
			a => {
				cli::print(&format!("Unknown Target: {}", a));
				help()
			}
		}
	}
}
