// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::env;
use std::path::PathBuf;

use action::Action;

#[derive(Debug)]
pub struct Opt {
    pub actions:    Vec<Action>,
    pub agenda_days:i64,
    pub debug:      bool,
    pub editor:     Option<String>,
    pub item_id:    i32,
    pub todo_dir:   PathBuf,
}


impl Opt {
    pub fn dump(&self) {
        println!("Opts are:");
        println!("{:?}", self);
    }

    pub fn new() -> Opt {
        // get editor
        // TODO: get env visual or editor
        let editor: Option<String> = env::var("VISUAL")
                                        .or(env::var("EDITOR"))
                                        .ok();

        // get todo dir
        let mut todo_dir = match env::home_dir() {
            Some(path) => path,
            None => { panic!("Could not get home dir"); },
        };
        todo_dir.push(".todo/");

        Opt {
            actions:    Vec::new(),
            agenda_days:8,
            debug:      false,
            editor:     editor,
            item_id:    0,
            todo_dir:   todo_dir,
        }
    }
}
