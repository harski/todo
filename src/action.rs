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
    Show,
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


fn print_item(item: &TodoItem) {
    println!("\t[{:3}]: {}", item.id, item.heading);
}


pub fn print_today(items: &Vec<Rc<TodoItem>>) {
    let today_str = match get_date_today() {
        Ok(date)    => date,
        Err(err)    => {
            print_err!("Could not get today's date: {}", err);
            return;
        },
    };

    // get all undone items, and print past and today's items
    let undone = todo_items::get_undone_items(&items);
    let before = todo_items::get_items_before(&undone, &today_str);
    let dateless = todo_items::get_dateless_items(&undone);
    let todays = todo_items::get_items_on_date(&undone, &today_str);

    println!("Items for today, {}", today_str);
    if todays.len() > 0 {
        for item in todays {
            print_item(&item);
        }
    } else {
        println!("\tNothing to do today :)");
    }

    if before.len() > 0 {
        println!("\nPast unfinished tasks:");

        let mut date_str: String = before.first().unwrap().get_date_str().unwrap();
        println!("{}:", date_str);

        // loop for different days
        for item in before {
            // update date if necessary
            let date_tmp = item.get_date_str().unwrap();
            if !date_str.eq(&date_tmp) {
                date_str = date_tmp;
                println!("\n{}:", date_str);
            }
            print_item(&item);
        }
    }

    if dateless.len() > 0 {
        println!("\nDateless unfinished tasks:");
        for item in dateless {
            print_item(&item);
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

    println!("Items for today, {}", today_str);
    if todays.len() > 0 {
        for item in todays {
            print_item(&item);
        }
    } else {
        println!("\tNothing to do today :)");
    }
}


pub fn print_version() {
    println!("todo version {}", VERSION);
    println!("{}", LICENSE_STR);
}


pub fn show_item(items: &Vec<Rc<TodoItem>>, i: i32) {
    match todo_items::get_item_by_id(&items, i) {
        Some(i) => {
            println!("id: {}", i.id);
            println!("filename: {}", i.filename);
            println!("heading: {}", i.heading);
            println!("\n{}", i.body);
        },
        None    => print_err!("Error: Item {} not found", i),
    };
}
