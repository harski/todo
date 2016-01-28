// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

extern crate getopts;
extern crate time;

// keep macros up here to be able to use them in submodules
#[macro_use]
mod util;
mod action;
mod attr;
mod opt;
mod optutil;
mod todo_item;
mod todo_items;

use getopts::Options;
use std::env;
use std::rc::Rc;

use action::Action;
use opt::Opt;
use todo_item::TodoItem;
use todo_items::{filter_items_on_date, get_todo_items};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const LICENSE_STR: &'static str =
    "Copyright 2016 Tuomo Hartikainen <tth@harski.org>.\n\
     Licensed under the 2-clause BSD license, see LICENSE for details.";

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts_in: Options = optutil::get_options();
    let opts: Opt = optutil::parse_options(&args, &opts_in);

    if opts.debug {
        opts.dump();
    }

    let action = optutil::get_action(&opts_in, &args);

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


fn get_date_today() -> Result<String, time::ParseError> {
    let now = time::now();
    match time::strftime("%Y-%m-%d", &now) {
        Ok(d_str)   => Ok(d_str),
        Err(err)    => Err(err),
    }
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
