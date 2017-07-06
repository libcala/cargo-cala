// bin/android.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use std::path::Path;
use abxml::apk::Apk;

pub fn execute() -> () {
	let path = Path::new("1.apk");
	let mut apk = Apk::new(path).unwrap();
	apk.export(Path::new("tmp/"), false).unwrap();
}
