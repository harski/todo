// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

extern crate getopts;
extern crate time;

// keep macros up here to be able to use them in submodules
#[macro_use]
mod util;

mod attr;
mod opt;
mod todo_item;
mod todo_items;

use getopts::Options;
use std::env;
use std::path::Path;

use opt::Opt;
use todo_items::{filter_todays_items, get_todo_items};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const LICENSE_STR: &'static str =
    "Copyright 2016 Tuomo Hartikainen <tth@harski.org>.\n\
     Licensed under the 2-clause BSD license, see LICENSE for details.";

enum Action {
    Dump,
    Help,
    Today,
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
        Action::Dump    => { dump(opts.todo_dir.as_path()); },
        Action::Help    => { print_help(&program, &opts_in); },
        Action::Today   => { print_today(opts.todo_dir.as_path()); },
        Action::Version => { print_version(); },
    }
}


fn dump(path: &Path) {
    let items = match get_todo_items(&path) {
        Ok(items) => items,
        Err(err)  => panic!("Error reading todo files from {:?}: {}", path, err),
    };

    for item in items {
        println!("{:?}", item);
    }
}


fn get_action(opts: &Options, args: &Vec<String>) -> Action {
    let matches = match opts.parse(&args[1..]) {
        Ok(m)   => { m }
        Err(f)  => { panic!(f.to_string()) }
    };

    let mut action_matches: Vec<Action> = Vec::new();

    if matches.opt_present("d") {
        action_matches.push(Action::Dump);
    }

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
        Action::Today
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


fn print_today(path: &Path) {
    let items = match get_todo_items(&path) {
        Ok(items)   => items,
        Err(err)    => panic!("Error reading todo files: {}", err),
    };
    let todays = match filter_todays_items(&items) {
        Ok(todos)   => todos,
        Err(err)    => {
            println!("Error filtering todays items: {}", err);
            return
        },
    };

    let mut first = true;
    for item in todays {
        match first {
            true    => first = false,
            false   => println!(""),
        };

        println!("{}", item.heading);
        for attr in &item.attrs {
            println!("{}: {}", attr.key, attr.value);
        }
    }
}


fn print_version() {
    println!("todo version {}", VERSION);
    println!("{}", LICENSE_STR);
}
