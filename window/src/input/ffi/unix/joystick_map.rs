// lib/input/ffi/unix/joystick_map.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

extern {
	fn ioctl(fd: i32, request: usize, v: *mut i32) -> i32;
}

pub fn joystick_map(fd: i32) -> (usize, usize, bool) {
	let mut num_axis = 0;
	let mut num_buttons = 0;

	let a = unsafe { ioctl(fd, 0x80016a11, &mut num_axis) };
	let b = unsafe { ioctl(fd, 0x80016a12, &mut num_buttons) };

	if a == -1 || b == -1 {
		return (0, 0, true)
	}

	(num_axis as usize, num_buttons as usize, false)
}
