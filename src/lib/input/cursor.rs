// lib/input/cursor.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Click {
	Left,
	Middle,
	Right,
	Touch,
}

impl ::std::fmt::Display for Click {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		use Click::*;

		// TODO: Write in language of the user.
		write!(f, "{}", match *self {
			Left => "Left Click",
			Middle => "Middle Click",
			Right => "Right Click",
			Touch => "Touch",
		})
	}
}
