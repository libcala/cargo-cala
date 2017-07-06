// lib/input/joystick.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use std::fmt;

use Input;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Button {
	A,
	B,
	C,
	D,
	E(usize),
}

impl fmt::Display for Button {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Button::A => write!(f, "A ( missile ) Button"),
			Button::B => write!(f, "B ( trigger ) Button"),
			Button::C => write!(f, "C ( x / right ) Button"),
			Button::D => write!(f, "D ( y / back ) Button"),
			Button::E(e) => write!(f, "Ext. Button {}", e),
		}
	}
}

use super::ffi::Joystick as NativeJoystick;

const VIRTUAL_AXIS_MOVE : usize = 0;
const VIRTUAL_AXIS_POV : usize = 2;
const VIRTUAL_AXIS_THROTTLE : usize = 4;

pub struct Joystick {
	joystick: NativeJoystick,
	oldstate: (Vec<f32>, Vec<bool>),
	state: (Vec<f32>, Vec<bool>),
	name: String,
}

impl Joystick {
	pub fn create() -> Joystick {
		let joystick = NativeJoystick::create();
		let (n_axis, n_buttons, is_out) = joystick.map();

		if is_out {
			return Joystick {
				joystick: joystick,
				oldstate: (Vec::new(), Vec::new()),
				state: (Vec::new(), Vec::new()),
				name: "".to_string(),
			};
		}

		let name = joystick.name();

		let mut axis = Vec::new();
		let mut buttons = Vec::new();

		axis.resize(n_axis, 0.0);
		buttons.resize(n_buttons, false);

		println!("adi_screen: New Joystick: {}", name);

		Joystick {
			joystick: joystick,
			oldstate: (axis.clone(), buttons.clone()),
			state: (axis, buttons),
			name: name,
		}
	}

	pub fn update(&mut self, input: &mut Vec<Input>) -> () {
		if self.not_plugged_in() {
			return
		}

		while self.joystick.poll_event(&mut self.state) { }

		// TODO: Create GUI widget to configure joystick.
		// Current configuration:
		//	JoystickMove - 0 -> 1 (Locked)
		//	JoystickThrottle - 2
		//	JoystickPov - 3 -> 4
		//	JoystickTrigger(Down,Up) -> 0
		//	JoystickButton[0] -> 1
		//	JoystickButton[1] -> 2
		//	JoystickButton[2] -> 3

		let js_axis_move = 0;
		let js_axis_throttle = 2;
		let js_axis_pov = 3;

		self.check_axis(input, (js_axis_move, VIRTUAL_AXIS_MOVE));
		self.check_axis(input, (js_axis_pov, VIRTUAL_AXIS_POV));
		self.check_axis(input,(js_axis_throttle,VIRTUAL_AXIS_THROTTLE));

		let js_button_trigger = 0;
		let js_button_0 = 1;
		let js_button_1 = 3;
		let js_button_2 = 2;

		self.check_button(input, (js_button_trigger, Button::B));
		self.check_button(input, (js_button_0, Button::A));
		self.check_button(input, (js_button_1, Button::C));
		self.check_button(input, (js_button_2, Button::D));
	}

	fn check_button(&mut self, input: &mut Vec<Input>, i: (usize,Button)) {
		if self.state.1[i.0] != self.oldstate.1[i.0] {
			let value = self.state.1[i.0];

			self.oldstate.1[i.0] = value;

			input.push(match value {
				false => Input::JoystickButtonUp(i.1),
				true => Input::JoystickButtonDown(i.1),
			});
		}
	}

	fn check_axis(&mut self, input: &mut Vec<Input>, i: (usize,usize)) {
		if match i.1 {
			VIRTUAL_AXIS_MOVE | VIRTUAL_AXIS_POV => {
				self.state.0[i.0+1] != self.oldstate.0[i.0+1]
			}
			_ => false
		} || self.state.0[i.0] != self.oldstate.0[i.0] {
			let x = self.state.0[i.0];

			self.oldstate.0[i.0] = x;

			input.push(match i.1 {
				VIRTUAL_AXIS_MOVE => {
					let y = self.state.0[i.0 + 1];

					self.oldstate.0[i.0 + 1] = y;
					Input::JoystickMove(x, y)
				}
				VIRTUAL_AXIS_POV => {
					let y = self.state.0[i.0 + 1];

					self.oldstate.0[i.0 + 1] = y;
					Input::JoystickPov(x, y)
				}
				VIRTUAL_AXIS_THROTTLE => {
					Input::JoystickThrottle(x)
				}
				_ => panic!("Nonexistant Virtual Axis")
			});
		}
	}

	fn not_plugged_in(&mut self) -> bool {
		if self.joystick.is_plugged_in() {
			let (_, _, is_out) = self.joystick.map();

			if is_out {
				println!("adi_screen: Unplugged Joystick: {}",
					self.name);
				self.joystick.disconnect();
			}

			is_out
		} else {
			self.joystick = NativeJoystick::create();
			self.name = self.joystick.name();
			let (n_axis, n_buttons, is_out) = self.joystick.map();

			if is_out == false {
				self.state.0.resize(n_axis, 0.0);
				self.state.1.resize(n_buttons, false);
				self.oldstate.0.resize(n_axis, 0.0);
				self.oldstate.1.resize(n_buttons, false);

				println!("adi_screen: New Joystick: {}",
					self.name);
			}

			is_out
		}
	}
}
