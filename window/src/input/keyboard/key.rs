// lib/input/keyboard/key.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

/// This enum represents a physical key on a keyboard.
#[derive(PartialEq, Eq)]
#[derive(Copy, Clone)]
pub enum Key {
	// Note: These rows are not necessarily the rows these keys are found.
	// Row1
	Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9, Num0,
		Minus, EqualSign, Backspace,
	// Row2
	Tab, Q, W, E, R, T, Y, U, I, O, P, BracketOpen, BracketClose, BackSlash,
	// Row3
	Compose, A, S, D, F, G, H, J, K, L, Semicolon, Apostrophe, Enter,
	// Row4
	Shift(bool), Z, X, C, V, B, N, M, Comma, Period, Slash,
	// Row5
	Ctrl(bool), Alt, Space, Up, Down, Left, Right,
	// Ext ( May require 2 keys to be pressed on some platforms )
	ExtBacktick, ExtDelete, ExtInsert, ExtNumLock, ExtPageUp, ExtPageDown,
	ExtHome, ExtEnd, ExtAsterisk, ExtPlus, ExtAltGr
}

impl ::std::fmt::Display for Key {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		use Key::*;

		// TODO: Write in keyboard layout & language of the user.
		write!(f, "{}", match *self {
			Num1 => "1",
			Num2 => "2",
			Num3 => "3",
			Num4 => "4",
			Num5 => "5",
			Num6 => "6",
			Num7 => "7",
			Num8 => "8",
			Num9 => "9",
			Num0 => "0",
			Minus => "-",
			EqualSign => "=",
			Backspace => "Backspace",
			Tab => "tab",
			Q => "Q",
			W => "W",
			E => "E",
			R => "R",
			T => "T",
			Y => "Y",
			U => "U",
			I => "I",
			O => "O",
			P => "P",
			BracketOpen => "[",
			BracketClose => "]",
			BackSlash => "\\",
			Compose => "Compose",
			A => "A",
			S => "S",
			D => "D",
			F => "F",
			G => "G",
			H => "H",
			J => "J",
			K => "K",
			L => "L",
			Semicolon => ";",
			Apostrophe => "'",
			Enter => "enter",
			Shift(false) => "Left Shift",
			Shift(true) => "Right Shift",
			Z => "Z",
			X => "X",
			C => "C",
			V => "V",
			B => "B",
			N => "N",
			M => "M",
			Comma => ",",
			Period => ".",
			Slash => "/",
			Ctrl(false) => "Left Ctrl",
			Ctrl(true) => "Right Ctrl",
			Alt => "Alt",
			ExtAltGr => "AltGr",
			Space => "space",
			Up => "Up",
			Down => "Down",
			Left => "Left",
			Right => "Right",
			ExtBacktick => "`",
			ExtDelete => "Delete",
			ExtInsert => "Insert",
			ExtNumLock => "NumLock",
			ExtPageUp => "PageUp",
			ExtPageDown => "PageDown",
			ExtHome => "Home",
			ExtEnd => "End",
			ExtAsterisk => "*",
			ExtPlus => "+",
		})
	}
}

impl Key {
	// create a Key from keycode
	pub(crate) fn create(physical_key: u32) -> Option<Key> {
		use os_window::key;

		Some( match physical_key {
			key::ext::BACKTICK => Key::ExtBacktick,
			key::ext::NUM_PAD_PLUS => Key::ExtPlus,
			key::ext::NUM_PAD_ASTERISK => Key::ExtAsterisk,
			key::SLASH | key::ext::NUM_PAD_SLASH => Key::Slash,
			key::ENTER | key::ext::NUM_PAD_ENTER => Key::Enter,
			key::NUM_1 | key::ext::NUM_PAD_1 => Key::Num1,
			key::NUM_2 | key::ext::NUM_PAD_2 => Key::Num2,
			key::NUM_3 | key::ext::NUM_PAD_3 => Key::Num3,
			key::NUM_4 | key::ext::NUM_PAD_4 => Key::Num4,
			key::NUM_5 | key::ext::NUM_PAD_5 => Key::Num5,
			key::NUM_6 | key::ext::NUM_PAD_6 => Key::Num6,
			key::NUM_7 | key::ext::NUM_PAD_7 => Key::Num7,
			key::NUM_8 | key::ext::NUM_PAD_8 => Key::Num8,
			key::NUM_9 | key::ext::NUM_PAD_9 => Key::Num9,
			key::NUM_0 | key::ext::NUM_PAD_0 => Key::Num0,
			key::PERIOD | key::ext::NUM_PAD_PERIOD => Key::Period,
			key::MINUS | key::ext::NUM_PAD_MINUS => Key::Minus,
			key::EQUAL_SIGN => Key::EqualSign,
			key::BACKSPACE => Key::Backspace,
			key::TAB => Key::Tab,
			key::Q => Key::Q,
			key::W => Key::W,
			key::E => Key::E,
			key::R => Key::R,
			key::T => Key::T,
			key::Y => Key::Y,
			key::U => Key::U,
			key::I => Key::I,
			key::O => Key::O,
			key::P => Key::P,
			key::BRACKET_OPEN => Key::BracketOpen,
			key::BRACKET_CLOSE => Key::BracketClose,
			key::LEFT_CTRL => Key::Ctrl(false),
			key::RIGHT_CTRL => Key::Ctrl(true),
			key::LEFT_SHIFT => Key::Shift(false),
			key::RIGHT_SHIFT => Key::Shift(true),
			key::LEFT_ALT => Key::Alt,
			key::ext::ALT_GR => Key::ExtAltGr,
			key::CAPS_LOCK => Key::Compose,
			key::A => Key::A,
			key::S => Key::S,
			key::D => Key::D,
			key::F => Key::F,
			key::G => Key::G,
			key::H => Key::H,
			key::J => Key::J,
			key::K => Key::K,
			key::L => Key::L,
			key::SEMICOLON => Key::Semicolon,
			key::APOSTROPHE => Key::Apostrophe,
			key::BACKSLASH => Key::BackSlash,
			key::Z => Key::Z,
			key::X => Key::X,
			key::C => Key::C,
			key::V => Key::V,
			key::B => Key::B,
			key::N => Key::N,
			key::M => Key::M,
			key::COMMA => Key::Comma,
			key::SPACE => Key::Space,
			key::ext::NUMLOCK => Key::ExtNumLock,
			key::ext::HOME => Key::ExtHome,
			key::ext::END => Key::ExtEnd,
			key::ext::PAGE_UP => Key::ExtPageUp,
			key::ext::PAGE_DOWN => Key::ExtPageDown,
			key::ext::INSERT => Key::ExtInsert,
			key::ext::DELETE => Key::ExtDelete,
			key::UP => Key::Up,
			key::LEFT => Key::Left,
			key::RIGHT => Key::Right,
			key::DOWN => Key::Down,
			_ => return None,
		} )
	}
}
