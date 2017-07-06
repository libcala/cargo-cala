// main.rs
// Graphical Software Packager Demo
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

#[macro_use]
extern crate gsp;

use gsp::{ Window, Input, InputQueue, Msg };

fn main() {
	let mut window = connect!();
	let mut queue = InputQueue::create();

	'mainloop: loop {
		window.update(&mut queue);

		for input in queue.iter() {
			use Input::*;
			use Msg::*;

			match *input {
				Msg(Quit) | Msg(Back) => break 'mainloop,
	//			Input::Redraw => redraw(&mut context),
				_ => {},
			}
		}
	}
}
