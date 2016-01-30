// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

extern crate time;

use std::rc::Rc;

use getopts::Options;

use todo_item::TodoItem;
use todo_items;

pub enum Action {
    Dump,
    Help,
    Today,
    Version,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const LICENSE_STR: &'static str =
    "Copyright 2016 Tuomo Hartikainen <tth@harski.org>.\n\
     Licensed under the 2-clause BSD license, see LICENSE for details.";


pub fn dump(items: &Vec<Rc<TodoItem>>) {
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


pub fn print_help(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [ACTION]", program);
    print!("{}", opts.usage(&brief));
}


pub fn print_today(items: &Vec<Rc<TodoItem>>) {
    let today_str = match get_date_today() {
        Ok(date)    => date,
        Err(err)    => {
            print_err!("Could not get today's date: {}", err);
            return;
        },
    };

    let todays = todo_items::filter_items_on_date(&items, &today_str);

    if todays.len() > 0 {
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
    } else {
        println!("Nothing to do today :)");
    }
}


pub fn print_version() {
    println!("todo version {}", VERSION);
    println!("{}", LICENSE_STR);
}
