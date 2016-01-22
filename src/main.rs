// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

extern crate getopts;

use getopts::Options;
use std::env;
use std::path::{Path, PathBuf};

mod file_utils;
use ::file_utils::get_files_in_dir;

mod opt;
use ::opt::Opt;

mod todo_item;
use ::todo_item::TodoItem;

#[macro_use]
mod util;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const LICENSE_STR: &'static str =
    "Copyright 2016 Tuomo Hartikainen <tth@harski.org>.\n\
     Licensed under the 2-clause BSD license, see LICENSE for details.";

enum Action {
    Dump,
    Help,
    Version,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts_in = Options::new();
    get_opt_strs(&mut opts_in);

    let mut opts: Opt = Opt::new();
    parse_options(&args, &opts_in, &mut opts);

    if opts.debug {
        opts.dump();
    }

    let action = get_action(&opts_in, &args);

    match action {
        Action::Help    => { print_help(&program, &opts_in); },
        Action::Version => { print_version(); },
        Action::Dump    => { dump(opts.todo_dir.as_path()); },
    }
}


fn dump(path: &Path) {
    let mut files: Vec<PathBuf> = Vec::new();
    match get_files_in_dir(&path, &mut files) {
        Ok(()) => (),
        Err(err) => panic!("Error reading todo files from {:?}: {}", path, err),
    }

    for file in files {
        match TodoItem::new_from_file(file.as_path()) {
            Ok(ti)  => {
                println!("file: {}", ti.filename);
                println!("heading: {}", ti.heading);
            },
            Err(err)=> print_err!("Could not init Todo item from {:?}: {}", file, err),
        };
    }
}


fn get_action(opts: &Options, args: &Vec<String>) -> Action {
    let matches = match opts.parse(&args[1..]) {
        Ok(m)   => { m }
        Err(f)  => { panic!(f.to_string()) }
    };

    let mut action_matches: Vec<Action> = Vec::new();
    if matches.opt_present("h") {
        action_matches.push(Action::Help);
    }

    if matches.opt_present("v") {
        action_matches.push(Action::Version);
    }

    // TODO: handle with `match`?
    if action_matches.len() > 1 {
        // error, too many actions found
        panic!("Error: too many actions supplied. Quitting");
    } else if action_matches.len() == 1 {
        match action_matches.pop() {
            Some(ac) => ac,
            None     => { panic!("Error reading action argument"); },
        }
    } else {
        Action::Dump
    }
}


fn get_opt_strs(opts: &mut Options) {
    opts.optflag("d", "debug", "set debug mode");
    opts.optflag("h", "help", "print this help");
    opts.optflag("v", "version", "show version");
}


fn parse_options(args: &Vec<String>, opts_in: &Options, opts: &mut Opt) {
    let matches = match opts_in.parse(&args[1..]) {
        Ok(m)   => { m }
        Err(f)  => { panic!(f.to_string()) }
    };

    if matches.opt_present("d") {
        opts.debug = true;
    }
}


fn print_help(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [ACTION]", program);
    print!("{}", opts.usage(&brief));
}


fn print_version() {
    println!("todo version {}", VERSION);
    println!("{}", LICENSE_STR);
}
