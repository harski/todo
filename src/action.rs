// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::rc::Rc;

use getopts::Options;
use time;

use todo_item::TodoItem;
use todo_items;

#[derive(Clone)]
pub enum Action {
    Dump,
    Help,
    Today,
    TodayOnly,
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

    // TODO: get all undone items, and print past and today's items
    let undone = todo_items::get_undone_items(&items);
    let before = todo_items::get_items_before(&undone, &today_str);
    let todays = todo_items::get_items_on_date(&undone, &today_str);

    if todays.len() > 0 {
        for item in todays {
            println!("{}", item.heading);
        }
    } else {
        println!("Nothing to do today :)");
    }

    if before.len() > 0 {
        println!("\nPast unfinished tasks:\n");

        // TODO: loop for different days
        for item in before {
            println!("{}", item.heading);
        }
    }
}


pub fn print_today_only(items: &Vec<Rc<TodoItem>>) {
    let today_str = match get_date_today() {
        Ok(date)    => date,
        Err(err)    => {
            print_err!("Could not get today's date: {}", err);
            return;
        },
    };

    let todays_all = todo_items::get_items_on_date(&items, &today_str);
    let todays = todo_items::get_undone_items(&todays_all);

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
