// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::env;
use std::path::PathBuf;


pub struct Opt {
    pub debug:      bool,
    pub todo_dir:   PathBuf,
}


impl Opt {
    pub fn dump(&self) {
        println!("Opts are:");
        println!("debug: {:?}", self.debug);
        println!("todo_dir: {:?}", self.todo_dir);
        println!("");
    }

    pub fn new() -> Opt {
        let mut todo_dir = match env::home_dir() {
            Some(path) => path,
            None => { panic!("Could not get home dir"); },
        };
        todo_dir.push(".todo/");

        Opt {
            debug: false,
            todo_dir: todo_dir,
        }
    }
}
