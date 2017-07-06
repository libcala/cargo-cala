// lib/os_window/windows/connection_create.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::void_pointer::*;

extern "system" {
	fn GetModuleHandleW(a: VoidPointer) -> VoidPointer;
}

pub fn connection_create() -> VoidPointer {
	unsafe {
		GetModuleHandleW(NULL)
	}
}
