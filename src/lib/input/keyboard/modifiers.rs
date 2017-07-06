// lib/input/keyboard/modifiers.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use Input;
use Key;
use Msg;
use Emphasis;
use Align;

const NONE : u8 = 0b0000_0000;
const SHIFT : u8 = 0b0000_0001;
const CTRL : u8 = 0b0000_0010;
const ALT : u8 = 0b0000_0100;
const COMPOSE : u8 = 0b0000_1000;
const ALT_SHIFT : u8 = ALT | SHIFT;
const CTRL_SHIFT : u8 = CTRL | SHIFT;

pub(crate) struct Modifiers {
	held: u8,
}

impl Modifiers {
	pub fn create() -> Modifiers {
		Modifiers { held: NONE }
	}

	pub fn update(&mut self, queue: &mut Vec<Input>, input: Input) -> () {
		match input {
			Input::Text(_) => match self.held {
				NONE | SHIFT => {},
				_ => return, // Ctrl,Shift,Alt shouldn't print.
			},
			Input::KeyPress(key) => match key {
				Key::Ctrl(_) => self.held |= CTRL,
				Key::Shift(_) => self.held |= SHIFT,
				Key::Alt => self.held |= ALT,
				Key::Compose => if self.held & COMPOSE == 0 {
					self.held |= COMPOSE
				} else {
					self.held &= !COMPOSE
				},
				Key::A => self.a(queue),
				Key::B => self.b(queue),
				Key::C => self.c(queue),
				Key::D => self.d(queue),
				Key::E => self.e(queue),
				Key::F => self.f(queue),
				Key::G => self.g(queue),
				Key::H => self.h(queue),
				Key::I => self.i(queue),
				Key::J => self.j(queue),
				Key::K => self.k(queue),
				Key::L => self.l(queue),
				Key::M => self.m(queue),
				Key::N => self.n(queue),
				Key::O => self.o(queue),
				Key::P => self.p(queue),
				Key::Q => self.q(queue),
				Key::R => self.r(queue),
				Key::S => self.s(queue),
				Key::T => self.t(queue),
				Key::U => self.u(queue),
				Key::V => self.v(queue),
				Key::W => self.w(queue),
				Key::X => self.x(queue),
				Key::Y => self.y(queue),
				Key::Z => self.z(queue),
				Key::Enter => self.enter(queue),
				Key::Apostrophe => self.apostrophe(queue),
				Key::Semicolon => self.semicolon(queue),
				Key::Num1 => self.num1(queue),
				Key::Num2 => self.num2(queue),
				Key::Num3 => self.num3(queue),
				Key::Num4 => self.num4(queue),
				Key::Num5 => self.num5(queue),
				Key::Num6 => self.num6(queue),
				Key::Num7 => self.num7(queue),
				Key::Num8 => self.num8(queue),
				Key::Num9 => self.num9(queue),
				Key::Num0 => self.num0(queue),
				_ => {},
			},
			Input::KeyRelease(key) => match key {
				Key::Ctrl(_) => self.held &= !CTRL,
				Key::Shift(_) => self.held &= !SHIFT,
				Key::Alt => self.held &= !ALT,
				_ => {},
			},
			_ => {},
		}
		queue.push(input)
	}

	fn a(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Select),
			ALT => return, // TODO: Aldaron's OS: To App Screen
			_ => return,
		})
	}

	fn b(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Emphasis(Emphasis::Bold)),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn c(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Copy),
			ALT => Input::Msg(Msg::Cancel),
			_ => return,
		})
	}

	fn d(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Delete),
			ALT => Input::Text('δ'),
			ALT_SHIFT => Input::Text('Δ'),
			_ => return,
		})
	}

	fn e(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Emphasis(Emphasis::None)),
			ALT => Input::Text('ə'),
			ALT_SHIFT => Input::Text('€'),
			_ => return,
		})
	}

	fn f(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Find),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn g(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Toggle Graphics / Terminal Mode
			_ => return,
		})
	}

	fn h(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Help),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn i(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Emphasis(Emphasis::Italic)), // 𝘢
			CTRL_SHIFT => Input::Msg(Msg::Info), // 🛈
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn j(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn k(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn l(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Align(Align::Left)),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn m(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn n(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Quit), // TODO: New Session.
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn o(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: File Open Popup Window
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn p(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Print),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn q(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Quit),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn r(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn s(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::SaveAs), //⭳ TODO: FileSys Popup
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn t(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Open(None)),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn u(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Emphasis(Emphasis::Underline)),//⎁
			CTRL_SHIFT => Input::Msg(Msg::Emphasis(Emphasis::UnderlineDC)),//⎂
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn v(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Paste),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn w(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Close),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn x(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Cut),
			ALT => Input::Text('×'),
			_ => return,
		})
	}

	fn y(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Redo),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn z(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Undo),
			CTRL_SHIFT => Input::Msg(Msg::Redo),
			ALT => Input::Text('÷'),
			_ => return,
		})
	}

	fn enter(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Align(Align::Justify)),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn apostrophe(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Align(Align::Right)),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn semicolon(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Align(Align::Middle)),
			ALT => Input::Text('°'),
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num1(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Emphasis(Emphasis::None)),
			ALT => return, // TODO: Aldaron's OS / No OS: 🔈 MUTE 🔇
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num2(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: 🔈 Volume - 🔉
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num3(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: 🔈 Volume + 🔊
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num4(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: ⏯ Play⏵,Pause⏸
			_ => return,
		})
	}

	fn num5(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Emphasis(Emphasis::StrikeOut)),
			ALT => return, // TODO: Aldaron's OS / No OS: ⏹ Stop
			_ => return,
		})
	}

	fn num6(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Emphasis(Emphasis::Overline)),
			ALT => return, // TODO: Aldaron's OS / No OS: ⏮ Track
			_ => return,
		})
	}

	fn num7(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Emphasis(Emphasis::UnderlineX2)),
			ALT => return, // TODO: Aldaron's OS / No OS: ⏭ Track
			_ => return,
		})
	}

	fn num8(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Msg(Msg::Emphasis(Emphasis::InvertColor)),
			ALT => return, // TODO: Brightness ☀ - 🔅
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num9(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Brightness ☀ + 🔆
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num0(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Toggle Monitor Config 🖵
			_ => return,
		})
	}
}
