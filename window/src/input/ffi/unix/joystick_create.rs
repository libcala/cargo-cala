// lib/input/ffi/unix/joystick_create.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use std::ffi::CString;

extern {
	fn open(pathname: *const u8, flags: i32) -> i32;
}

fn open_joystick(name: &str) -> i32 {
	let file_name = CString::new(name).unwrap();

	unsafe {
		open(file_name.as_ptr() as *const _, 0)
	}
}

pub fn joystick_create() -> i32 {
	let joystick = open_joystick("/dev/js0");

	if joystick != -1 {
		return joystick;
	}

	let joystick = open_joystick("/dev/input/js0");

	joystick
}
