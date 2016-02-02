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

use action::Action;
use opt::Opt;
use todo_items::get_todo_items;

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
        Action::Help    => { action::print_help(&program, &opts_in); return; },
        Action::Version => { action::print_version(); return },
        _               => { },
    }

    // "proper" actions
    match get_todo_items(opts.todo_dir.as_path()) {
        Ok(items)   => {
            match action {
                Action::Dump    => { action::dump(&items); },
                Action::Today   => { action::print_today(&items); },
                _               => {},
            }
        },
        Err(err)    => print_err!("Could not parse todo items: {}", err),
    };
}
