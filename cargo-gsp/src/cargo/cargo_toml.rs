// bin/cargo/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use file;
use parse;

pub fn save(_cargo_toml: &parse::CargoToml) {
    // TODO: Can't have cargo.toml in directory under invalid cargo.toml
    /*	let mut deps = String::new();

        for elem in &cargo_toml.dependencies {
            deps.push_str(&format!("{} = {}\n", elem.0, elem.1));
        }

        let data = format!("\
            [package]\n\
            name = \"gsp_program\"\n\
            version = \"0.0.0\"\n\
            authors = [\"generated\"]\n\
            \n\
            [[bin]]\n\
            name = \"gsp_program\"\n\
            path = \"src/main.rs\"\n\
            \n\
            [lib]\n\
            path = \"src/lib.rs\"\n\
            crate_type = [\"cdylib\"]\n\
            \n\
            [dependencies]\n\
            {}\n\
        ", deps);
    //		cargo-gsp = { path = ".." }
    //");

        file::save("target/cargo/Cargo.toml", data);*/
}
