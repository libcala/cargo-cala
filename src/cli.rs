// cli.rs
// Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use std::io;
use std::io::Write;

pub struct Progress(usize);

impl Progress {
	pub fn new(string: &str, init_msg: &str) -> Progress {
		print!("{} {}", string, init_msg);
		update();

		Progress(init_msg.len())
	}

	pub fn update(&mut self, message: &str) -> () {
		// Replace Last Printed With New Message
		self.overwrite(message);

		// Update length
		self.0 = message.len();
	}

	pub fn complete(&self, success: bool) {
		// Print out status
		self.overwrite(match success {
			true => "success!\n",
			false => "failed!\n",
		});
	}

	fn overwrite(&self, message: &str) -> () {
		// Clear the old message
		for _ in 0..self.0 {
			print!("\u{8} \u{8}");
		}

		// Write the new message
		print!("{}", message);

		// Update Display
		update();
	}
}

/// Print a message to the terminal.
pub fn print(message: &str) {
	println!("{}", message);
}

/// Update the text in the terminal.
pub fn update() {
	io::stdout().flush().ok().expect("Could not flush stdout");
}
