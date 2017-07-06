// lib/os_window/windows/string.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

// Windows uses UTF-16
pub fn native(what: &str) -> Vec<u8> {
	let mut rtn : Vec<u8> = Vec::new();
	for c in what.encode_utf16() {
		rtn.push((c % 255) as u8);
		rtn.push((c / 255) as u8);
	}
	rtn.push(0);
	rtn.push(0);
	rtn
}
