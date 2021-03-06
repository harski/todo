// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

extern crate getopts;
extern crate time;

// keep macros up here to be able to use them in submodules
#[macro_use]
mod util;
mod action;
mod attr;
mod error;
mod opt;
mod optutil;
mod status;
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
    let opts: Opt = match optutil::parse_options(&args, &opts_in) {
        Ok(opt) => opt,
        Err(e)  => {
            print_err!("Error parsing options: {}", e);
            std::process::exit(1);
        },
    };

    let action = match check_actions(&opts.actions) {
        Some(ac)    => { ac },
        None        => {
            return;
        },
    };

    if opts.debug {
        opts.dump();
    }

    // "trivial" actions, always return
    match action {
        Action::Help    => { action::print_help(&program, &opts_in); return; },
        Action::Version => { action::print_version(); return },
        _               => { },
    }

    // "proper" actions
    match get_todo_items(opts.todo_dir.as_path()) {
        Ok(mut items)   => {
            match action {
                Action::Agenda  => { action::agenda(&opts, &items); },
                Action::Delete  => { action::delete_item(&mut items, opts.item_id); },
                Action::Dump    => { action::dump(&items); },
                Action::Edit    => {
                    match action::edit_item(&items, opts.item_id, &opts.editor) {
                        Err(e)  => { print_err!("Error editing item: {}", e ) },
                        Ok(())  => {},
                    }
                },
                Action::Show    => { action::show_item(&items, opts.item_id); },
                Action::Today   => { action::print_today(&items); },
                Action::TodayOnly   => { action::print_today_only(&items); },
                _               => {},
            }
        },
        Err(err)    => print_err!("Could not parse todo items: {}", err),
    };
}


/// Check that only one action has been called
fn check_actions(actions: &Vec<Action>) -> Option<Action> {
    if actions.len() == 0 {
        print_err!("Action not set");
        return None;
    } else if actions.len() > 1 {
        print_err!("Too many actions set");
        return None;
    } else {
        return actions.last().cloned();
    }
}
