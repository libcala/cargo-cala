// file.rs
// Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn save(name: &str, data: &[u8]) {
	let path = Path::new(name);
	let parent = path.parent().unwrap();

	if parent.exists() == false {
		fs::create_dir_all(parent).unwrap();
	}

	fs::File::create(name).unwrap().write_all(data).unwrap();
}

pub fn load(name: &str) -> Vec<u8> {
	let mut file = fs::File::open(name).unwrap();
	let mut contents = Vec::new();

	file.read_to_end(&mut contents).unwrap();

	contents
}
