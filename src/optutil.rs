// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use getopts::Options;

use action::Action;
use opt::Opt;

// TODO: rephrase
pub fn get_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("D", "debug", "set debug mode");
    opts.optflag("d", "dump", "show raw todo items");
    opts.optflag("h", "help", "print this help");
    opts.optflag("t", "today", "print today's and past undone items");
    opts.optflag("T", "today-only", "print only today's items");
    opts.optflag("v", "version", "show version");
    opts
}


pub fn parse_options(args: &Vec<String>, opts_in: &Options) -> Opt {
    let mut opts: Opt = Opt::new();

    let matches = match opts_in.parse(&args[1..]) {
        Ok(m)   => { m }
        Err(f)  => { panic!(f.to_string()) }
    };

    if matches.opt_present("D") { opts.debug = true; }
    if matches.opt_present("d") { opts.actions.push(Action::Dump); }
    if matches.opt_present("h") { opts.actions.push(Action::Help); }
    if matches.opt_present("t") { opts.actions.push(Action::Today); }
    if matches.opt_present("T") { opts.actions.push(Action::TodayOnly); }
    if matches.opt_present("v") { opts.actions.push(Action::Version); }
    opts
}
