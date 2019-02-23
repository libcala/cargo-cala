// bin/password.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use rand;
use rand::distributions::{IndependentSample, Range};

pub fn new() -> Vec<u8> {
    let mut string = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..64 {
        let range = match rand::random::<u32>() % 3 {
            0 => Range::new(b'a', b'z'),
            1 => Range::new(b'A', b'Z'),
            2 => Range::new(b'0', b'9'),
            _ => unreachable!(),
        };

        string.push(range.ind_sample(&mut rng));
    }

    string
}
