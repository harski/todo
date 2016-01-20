// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

extern crate getopts;

use getopts::Options;
use std::env;
use std::path::{Path, PathBuf};

mod file_utils;
use ::file_utils::get_files_in_dir;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const LICENSE_STR: &'static str =
    "Copyright 2016 Tuomo Hartikainen <tth@harski.org>.\n\
     Licensed under the 2-clause BSD license, see LICENSE for details.";

fn main() {
    // TODO: Parse options
    // TODO: Parse action
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help");
    opts.optflag("v", "version", "show version");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)   => { m }
        Err(f)  => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_help(&program, opts);
        return;
    }

    if matches.opt_present("v") {
        print_version();
        return;
    }

    let mut files: Vec<PathBuf> = Vec::new();
    let dir = Path::new(".");
    match get_files_in_dir(&dir, &mut files) {
        Ok(()) => (),
        Err(err) => panic!("Error reading todo files: {}", err),
    }

    for file in files {
        if let Some(fname) = file.to_str() {
            println!("Found file '{}'", fname);
        }
    }
}


fn print_help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [ACTION]", program);
    print!("{}", opts.usage(&brief));
}


fn print_version() {
    println!("todo version {}", VERSION);
    println!("{}", LICENSE_STR);
}
