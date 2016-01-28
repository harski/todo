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
use std::rc::Rc;

use opt::Opt;
use todo_item::TodoItem;
use todo_items::{filter_items_on_date, get_todo_items};

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
    let opts_in: Options = get_options();
    let opts: Opt = parse_options(&args, &opts_in);

    if opts.debug {
        opts.dump();
    }

    let action = get_action(&opts_in, &args);

    // "trivial" actions, always return
    match action {
        Action::Help    => { print_help(&program, &opts_in); return; },
        Action::Version => { print_version(); return },
        _               => { },
    }

    // "proper" actions
    match get_todo_items(opts.todo_dir.as_path()) {
        Ok(items)   => {
            match action {
                Action::Dump    => { dump(&items); },
                Action::Today   => { print_today(&items); },
                _               => {},
            }
        },
        Err(err)    => print_err!("Could not parse todo items: {}", err),
    };
}


fn dump(items: &Vec<Rc<TodoItem>>) {
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

    // check for trivial actions (given as options)
    if matches.opt_present("d") {
        action_matches.push(Action::Dump);
    }

    if matches.opt_present("h") {
        action_matches.push(Action::Help);
    }

    if matches.opt_present("v") {
        action_matches.push(Action::Version);
    }

    // check for proper actions

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


fn get_date_today() -> Result<String, time::ParseError> {
    let now = time::now();
    match time::strftime("%Y-%m-%d", &now) {
        Ok(d_str)   => Ok(d_str),
        Err(err)    => Err(err),
    }
}


fn get_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("d", "debug", "set debug mode");
    opts.optflag("h", "help", "print this help");
    opts.optflag("v", "version", "show version");
    opts
}


fn parse_options(args: &Vec<String>, opts_in: &Options) -> Opt {
    let mut opts: Opt = Opt::new();
    let matches = match opts_in.parse(&args[1..]) {
        Ok(m)   => { m }
        Err(f)  => { panic!(f.to_string()) }
    };

    if matches.opt_present("d") {
        opts.debug = true;
    }
    opts
}


fn print_help(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [ACTION]", program);
    print!("{}", opts.usage(&brief));
}


fn print_today(items: &Vec<Rc<TodoItem>>) {
    let today_str = match get_date_today() {
        Ok(date)    => date,
        Err(err)    => {
            print_err!("Could not get today's date: {}", err);
            return;
        },
    };

    let todays = filter_items_on_date(&items, &today_str);

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
