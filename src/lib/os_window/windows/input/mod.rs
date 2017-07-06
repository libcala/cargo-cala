// lib/os_window/windows/input/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

// pub fn english(physical_key: u16, scan_key: u16) -> Key {
//	let physical_key = (physical_key as u32) | ( scan_key as u32 >> 16 );
// }

pub mod key {
	pub const A : u32 = 65;
	pub const B : u32 = 66;
	pub const C : u32 = 67;
	pub const D : u32 = 68;
	pub const E : u32 = 69;
	pub const F : u32 = 70;
	pub const G : u32 = 71;
	pub const H : u32 = 72;
	pub const I : u32 = 73;
	pub const J : u32 = 74;
	pub const K : u32 = 75;
	pub const L : u32 = 76;
	pub const M : u32 = 77;
	pub const N : u32 = 78;
	pub const O : u32 = 79;
	pub const P : u32 = 80;
	pub const Q : u32 = 81;
	pub const R : u32 = 82;
	pub const S : u32 = 83;
	pub const T : u32 = 84;
	pub const U : u32 = 85;
	pub const V : u32 = 86;
	pub const W : u32 = 87;
	pub const X : u32 = 88;
	pub const Y : u32 = 89;
	pub const Z : u32 = 90;
	pub const NUM_1 : u32 = 49;
	pub const NUM_2 : u32 = 50;
	pub const NUM_3 : u32 = 51;
	pub const NUM_4 : u32 = 52;
	pub const NUM_5 : u32 = 53;
	pub const NUM_6 : u32 = 54;
	pub const NUM_7 : u32 = 55;
	pub const NUM_8 : u32 = 56;
	pub const NUM_9 : u32 = 57;
	pub const NUM_0 : u32 = 48;
	pub const MINUS : u32 = 189;
	pub const EQUAL_SIGN : u32 = 187;
	pub const BACKSPACE : u32 = 8;
	pub const TAB : u32 = 9;
	pub const BRACKET_OPEN : u32 = 219;
	pub const BRACKET_CLOSE : u32 = 221;
	pub const ENTER : u32 = 13;
	pub const LEFT_CTRL : u32 = 17 | (0b_0000_0000 >> 16);
	pub const RIGHT_CTRL : u32 = 17 | (0b_1000_11101 >> 16);
	pub const SEMICOLON : u32 = 186;
	pub const APOSTROPHE : u32 = 222;
	pub const LEFT_SHIFT : u32 = 16 | (0b_0000_0000 >> 16);
	pub const RIGHT_SHIFT : u32 = 16 | (0b_0011_0110 >> 16);
	pub const BACKSLASH : u32 = 220;
	pub const COMMA : u32 = 188;
	pub const PERIOD : u32 = 190;
	pub const LEFT_ALT : u32 = 18 | (0b0_0000_0000 >> 16);
	pub const CAPS_LOCK : u32 = 20;
	pub const SPACE : u32 = 32;
	pub const SLASH : u32 = 111;
	pub const UP : u32 = 38;
	pub const DOWN : u32 = 40;
	pub const LEFT : u32 = 37;
	pub const RIGHT : u32 = 39;

	pub mod ext {
		pub const NUM_PAD_1 : u32 = 97;
		pub const NUM_PAD_2 : u32 = 98;
		pub const NUM_PAD_3 : u32 = 99;
		pub const NUM_PAD_4 : u32 = 100;
		pub const NUM_PAD_5 : u32 = 101;
		pub const NUM_PAD_6 : u32 = 102;
		pub const NUM_PAD_7 : u32 = 103;
		pub const NUM_PAD_8 : u32 = 104;
		pub const NUM_PAD_9 : u32 = 105;
		pub const NUM_PAD_0 : u32 = 96;
		pub const NUM_PAD_MINUS : u32 = 109;
		pub const NUM_PAD_ENTER : u32 = ;
		pub const NUM_PAD_PERIOD : u32 = 110;
		pub const NUM_PAD_ASTERISK : u32 = 106;
		pub const NUM_PAD_PLUS : u32 = 107;
		pub const NUM_PAD_SLASH : u32 = 191;
		pub const BACKTICK : u32 = 192;
		pub const NUMLOCK : u32 = 144;
		pub const ALT_GR : u32 = 18 | (0b1_0011_1000 >> 16);
		pub const FULLSCREEN : u32 = 122;
		pub const HOME : u32 = 36;
		pub const END : u32 = 35;
		pub const PAGE_UP : u32 = 33;
		pub const PAGE_DOWN : u32 = 34;
		pub const INSERT : u32 = 45;
		pub const DELETE : u32 = 46;
	}

	pub mod lib {
		pub const ESCAPE : u32 = 27;
	}
}
