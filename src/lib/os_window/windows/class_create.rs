// lib/os_window/windows/class_create.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::void_pointer::*;
use super::{ string };
use image::Image;

use std::mem;

#[repr(C)]
struct WndClassEx {
	cb_size: u32,
	style: u32,
	lpfn_wnd_proc: extern "C" fn(a: VoidPointer, b: u32, c: VoidPointer,
		d: VoidPointer) -> isize,
	cb_cls_extra: i32,
	cb_wnd_extra: i32,
	h_instance: VoidPointer,
	h_icon: VoidPointer,
	h_cursor: VoidPointer,
	hbr_background: VoidPointer,
	lpsz_menu_name: usize, // Char *
	lpsz_class_name: *const [u8;80],
	h_icon_sm: VoidPointer,
}

extern "system" {
	fn CreateIcon(hi: VoidPointer, w: i32, h: i32, planes: u8,
		bitspixel: u8, and: *const u8, xor: *const u8) -> VoidPointer;
	fn LoadCursorW(hi: VoidPointer, cursorName: usize) -> VoidPointer;
	fn GetStockObject(fnObject: i32) -> VoidPointer;
	fn RegisterClassExW(a: *const WndClassEx) -> u16;
}

pub fn class_create(hi: VoidPointer, title: &str, icon: Image,
	wnd_proc: extern "C" fn(a: VoidPointer, b: u32, c: VoidPointer,
		d: VoidPointer) -> isize)
	-> [u8; 80]
{
	let mut name : [u8; 80] = [0u8; 80];
	let nam = string::native(title);

	for i in 0..nam.len() {
		name[i] = nam[i];
	}

	let width = icon.size.0 as i32;
	let height = icon.size.1 as i32;

	let mut and : Vec<u8> = Vec::new();
	let mut xor : Vec<u8> = Vec::new();

	let w = icon.size.0 as usize;
	for i in 0usize..icon.size.0 as usize{
		for j in 0usize..icon.size.1 as usize {
			// Xor
			xor.push(icon.pixels[1 + 3 * (j + (w * i))]);
			xor.push(icon.pixels[0 + 3 * (j + (w * i))]);
			xor.push(icon.pixels[2 + 3 * (j + (w * i))]);
			xor.push(0xFF);
			// And
			and.push(0xFF);
			and.push(0xFF);
			and.push(0xFF);
			and.push(0xFF);
		}
	}

	let new_icon = unsafe {
		CreateIcon(hi, width, height, 1, 32, &and[0], &xor[0])
	};
	
	let window_class = WndClassEx {
		cb_size: mem::size_of::<WndClassEx>() as u32,
		style: 0x0002 | 0x0001,
		lpfn_wnd_proc: wnd_proc,
		cb_cls_extra: 0,
		cb_wnd_extra: 0,
		h_instance: hi,
		h_icon: new_icon,
		h_cursor: unsafe { LoadCursorW(NULL, 32512) },
		hbr_background: unsafe { GetStockObject(0) },
		lpsz_menu_name: 0,
		lpsz_class_name: &name,
		h_icon_sm: new_icon,
	};
	
	if unsafe { RegisterClassExW(&window_class) } == 0 {
		panic!("Failed to register windows class.");
	}
	
	name
}
