// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::io::{Error, ErrorKind};

use getopts::Options;

use action::Action;
use opt::Opt;

// TODO: rephrase option messages
pub fn get_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("a", "agenda", "show agenda");
    opts.optopt("A", "agenda-days", "set agenda days", "NUM");
    opts.optflag("D", "debug", "set debug mode");
    opts.optflag("d", "dump", "show raw todo items");
    opts.optflag("h", "help", "print this help");
    opts.optopt("i", "id", "select item by ID", "ID");
    opts.optflag("s", "show", "show item identified by -i");
    opts.optflag("t", "today", "print today's and past undone items");
    opts.optflag("T", "today-only", "print only today's items");
    opts.optflag("v", "version", "show version");
    opts.optflag("X", "delete", "delete item");
    opts
}


pub fn parse_options(args: &Vec<String>, opts_in: &Options)
                     -> Result<Opt, Error> {
    let mut opts: Opt = Opt::new();

    let matches = match opts_in.parse(&args[1..]) {
        Ok(m)   => { m }
        Err(f)  => { panic!(f.to_string()) }
    };

    if matches.opt_present("A") {
        opts.actions.push(Action::Agenda);
        match matches.opt_str("A") {
            Some(id)  => match id.parse::<i64>() {
                Ok(i)    => { opts.agenda_days = i; },
                Err(err) => {
                    let err_msg =
                        format!("Invalid '--agenda-days' argument '{}': {}",
                                id, err);
                    return Err(Error::new(ErrorKind::Other, err_msg));
                },
            },
            None  => {},
        };
    };
    if matches.opt_present("a") { opts.actions.push(Action::Agenda); }
    if matches.opt_present("D") { opts.debug = true; }
    if matches.opt_present("d") { opts.actions.push(Action::Dump); }
    if matches.opt_present("h") { opts.actions.push(Action::Help); }
    if matches.opt_present("i") {
        match matches.opt_str("i") {
            Some(id)  => match id.parse::<i32>() {
                Ok(i)    => { opts.item_id = i; },
                Err(err) => {
                    let err_msg =
                        format!("Invalid item ID '{}': {}", id, err);
                    return Err(Error::new(ErrorKind::Other, err_msg));
                },
            },
            None  => {},
        };
    }
    if matches.opt_present("s") { opts.actions.push(Action::Show); };
    if matches.opt_present("t") { opts.actions.push(Action::Today); }
    if matches.opt_present("T") { opts.actions.push(Action::TodayOnly); }
    if matches.opt_present("v") { opts.actions.push(Action::Version); }
    if matches.opt_present("X") { opts.actions.push(Action::Delete); }
    opts.actions.sort();
    opts.actions.dedup();
    Ok(opts)
}
