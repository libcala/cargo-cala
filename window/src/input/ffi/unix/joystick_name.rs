// lib/input/ffi/unix/joystick_name.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use std::str;

extern {
	fn ioctl(fd: i32, request: usize, v: *mut u8) -> i32;
}

pub fn joystick_name(fd: i32) -> String {
	let mut name = [0u8; 80];

	let error = unsafe {
		ioctl(fd, 0x80506a13, &mut name[0])
	} == -1;

	if error {
		return String::from("unknown");
	}

	String::from(str::from_utf8(&name[..]).unwrap_or("unknown"))
}
