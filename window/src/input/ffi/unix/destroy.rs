// lib/input/ffi/unix/destroy.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

extern {
	fn close(fd: i32) -> i32;
}

pub fn joystick(fd: i32) -> () {
	let failure = unsafe {
		close(fd) == -1
	};

	if failure {
		panic!("Failed to disconnect joystick.");
	}
}
