// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::env;
use std::path::PathBuf;

use action::Action;

pub struct Opt {
    pub actions:    Vec<Action>,
    pub agenda_days:i64,
    pub debug:      bool,
    pub item_id:    i32,
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
            actions:    Vec::new(),
            agenda_days:8,
            debug:      false,
            item_id:    0,
            todo_dir:   todo_dir,
        }
    }
}
