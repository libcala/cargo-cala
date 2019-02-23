// bin/program.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::process;
use std::process::Command;
use std::process::Stdio;

use cli;

pub struct Pipe {
    line: String,
    x: usize,
}

impl Pipe {
    pub fn new() -> Pipe {
        let line = String::new();
        let x = 0;

        Pipe { line, x }
    }

    pub fn update(&mut self, program: &mut process::Child, progress: &mut cli::Progress) -> () {
        {
            let mut output = BufReader::new(program.stdout.as_mut().unwrap());
            output.read_line(&mut self.line).unwrap();
        }

        let i = self.line.lines().nth(self.x);

        if let Some(i) = i {
            progress.update(i);

            self.x += 1;
        }
    }

    pub fn update_log(&mut self, program: &mut process::Child) {
        {
            let mut output = BufReader::new(program.stdout.as_mut().unwrap());
            output.read_line(&mut self.line).unwrap();
        }

        let i = self.line.lines().nth(self.x);

        if let Some(i) = i {
            println!("{}", i);

            self.x += 1;
        }
    }
}

/// Start a program.
pub fn spawn(prg: &str, args: Vec<&str>) -> process::Child {
    Command::new(prg)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("Couldn't Start {}", prg))
}

/// Wait on a program until it quits, updating stdout as needed.
pub fn wait_on(
    mut program: process::Child,
    mut progress: Option<cli::Progress>,
    mut pipe: Pipe,
    error: &str,
) {
    loop {
        match program.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    if let Some(ref mut pr) = progress {
                        pr.complete(true);
                    }
                } else {
                    if let Some(ref mut pr) = progress {
                        pr.complete(false);
                    }

                    let mut out = String::new();
                    match program.stdout {
                        Some(mut x) => {
                            x.read_to_string(&mut out).unwrap();
                        }
                        None => out = "Error: No Output".to_string(),
                    }

                    cli::print(&out);
                    exit(error);
                }
                break;
            }
            Ok(None) => {
                if let Some(ref mut pr) = progress {
                    pipe.update(&mut program, pr);
                } else {
                    pipe.update_log(&mut program);
                }
            }
            Err(e) => println!("error attempting to wait: {}", e),
        }
    }
}

pub fn execute(name: &str, prg: &str, args: Vec<&str>, _fail: &str, _error: &str) {
    let program = spawn(prg, args);
    let progress = cli::Progress::new(name, "Starting....");
    let pipe = Pipe::new();

    wait_on(program, Some(progress), pipe, &format!("{} failed!", name));
}

pub fn execute_log(prg: &str, args: Vec<&str>) {
    let program = spawn(prg, args);
    let pipe = Pipe::new();

    wait_on(program, None, pipe, &format!("{} failed!", prg));
}

pub fn execute_in(_wd: &str, prg: &str, args: Vec<&str>) {
    let program = spawn(prg, args);
    let progress = cli::Progress::new(prg, "Starting....");
    let pipe = Pipe::new();

    wait_on(program, Some(progress), pipe, &format!("{} failed!", prg));
}

pub fn exit(error: &str) {
    cli::print(error);
    process::exit(1)
}
