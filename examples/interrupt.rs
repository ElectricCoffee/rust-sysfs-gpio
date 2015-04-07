// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Edge, Pin};
use std::env;
use std::io;

fn interrupt(pin : u64) -> io::Result<()> {
    let input = Pin::new(pin);
    input.with_exported(|| {
        try!(input.set_direction(Direction::In));
        let edge = try!(input.get_edge());
        println!("Edge: {:?}", edge);
        try!(input.set_edge(Edge::BothEdges));
        let edge = try!(input.get_edge());
        println!("Edge: {:?}", edge);
        Ok(())
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./interrupt <pin>");
    } else {
        match args[1].parse::<u64>() {
            Ok(pin) => match interrupt(pin) {
                Ok(()) => println!("Interrupting Complete!"),
                Err(err) => println!("Error: {}", err),
            },
            Err(_) => println!("Usage: ./interrupt <pin>"),
        }
    }
}
