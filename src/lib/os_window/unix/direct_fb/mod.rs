// lib/os_window/unix/direct_fb/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use Input;

pub struct FBWindow {
	// TODO
}

impl FBWindow {
	pub fn poll_event(&self, input: &mut Vec<Input>, wh: &mut (u32, u32))
		-> bool
	{
		// TODO: this breaks
		false
	}
}
