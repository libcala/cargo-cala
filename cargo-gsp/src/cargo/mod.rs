// bin/cargo/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use file;
use parse;

mod cargo_toml;

const MAIN: &'static [u8] = include_bytes!("res/main.rs");
const LIB: &'static [u8] = include_bytes!("res/lib.rs");

fn copy_program() -> () {
    file::copy("src", "target/crate/app").unwrap();
    //	file::copy("res", "target/crate/res");
}

fn save_entrys() -> () {
    file::save("target/crate/main.rs", MAIN);
    file::save("target/crate/lib.rs", LIB);
}

pub fn execute(cargo_toml: &parse::CargoToml) -> () {
    cargo_toml::save(cargo_toml);
    copy_program();
    save_entrys();
}
