// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use getopts::Options;

use action::Action;
use opt::Opt;

pub fn get_action(opts: &Options, args: &Vec<String>) -> Action {
    let matches = match opts.parse(&args[1..]) {
        Ok(m)   => { m }
        Err(f)  => { panic!(f.to_string()) }
    };

    let mut action_matches: Vec<Action> = Vec::new();

    // check for trivial actions (given as options)
    if matches.opt_present("h") {
        action_matches.push(Action::Help);
    }

    if matches.opt_present("v") {
        action_matches.push(Action::Version);
    }

    // check for proper actions
    get_proper_actions(&args, &mut action_matches);

    // TODO: handle with `match`?
    if action_matches.len() > 1 {
        // error, too many actions found
        // TODO: elaborate which actions
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


pub fn get_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("d", "debug", "set debug mode");
    opts.optflag("h", "help", "print this help");
    opts.optflag("v", "version", "show version");
    opts
}


pub fn parse_options(args: &Vec<String>, opts_in: &Options) -> Opt {
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


fn get_proper_actions(args: &Vec<String>, actions: &mut Vec<Action>)
{
    for arg in args {
        match arg as &str {
            "dump"  => { actions.push(Action::Dump); },
            "today" => { actions.push(Action::Today); },
            _       => {},
        }
    }
}
