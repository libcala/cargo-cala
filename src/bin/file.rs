// bin/file.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn save(name: &str, data: &[u8]) -> () {
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

/// 
pub fn get_permissions(name: &str) -> fs::Permissions {
	let file = fs::File::open(name).unwrap();
	
	file.metadata().unwrap().permissions()
}

/// 
pub fn set_permissions(name: &str, permissions: fs::Permissions) -> () {
	let file = fs::File::open(name).unwrap();

	file.set_permissions(permissions).unwrap()
}

pub fn copy(src: &str, dst: &str) -> () {
	let permissions = get_permissions(src);
	let data = load(src);

	save(dst, data.as_slice());
	set_permissions(dst, permissions);
}

pub fn get_exists(name: &str) -> bool {
	Path::new(name).exists()
}
