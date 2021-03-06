// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::ffi::OsString;
use std::fs;
use std::ops::Add;
use std::process::Command;
use std::rc::Rc;

use getopts::Options;
use time;

use error::{TodoError, TodoErrorKind, TodoResult};
use opt::Opt;
use todo_item::TodoItem;
use todo_items;
use util;

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub enum Action {
    Agenda,
    Delete,
    Dump,
    Edit,
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


pub fn agenda(opt: &Opt, items: &Vec<Rc<TodoItem>>) {
    // TODO: Error handling, get rid of unwrap()
    let today: time::Tm = time::now() ;
    let dur = time::Duration::days(opt.agenda_days);
    let up_limit = today.add(dur);
    let limit_str = util::date_to_str(&up_limit).unwrap();

    let undone = todo_items::get_undone_items(&items);
    let before = todo_items::get_items_before(&undone, &limit_str);
    let agenda = todo_items::get_items_after(&before,
                                             &util::date_to_str(&today).unwrap());

    // print agenda
    if agenda.len() > 0 {
        println!("Agenda for the next {} days:", opt.agenda_days);

        let mut date_str: String = agenda.first().unwrap().get_date_str().unwrap();
        println!("{}:", date_str);

        // loop for different days
        for item in agenda {
            // update date if necessary
            let date_tmp = item.get_date_str().unwrap();
            if !date_str.eq(&date_tmp) {
                date_str = date_tmp;
                println!("\n{}:", date_str);
            }
            print_item(&item);
        }
    } else {
        println!("Agenda is empty for the next {} days.", opt.agenda_days);
    }
}


pub fn delete_item(mut items: &mut Vec<Rc<TodoItem>>, i: i32) {
    if i != 0 {
        /* TODO: get_item to remove_item */
        match todo_items::get_item_by_id(&items, i) {
            Some(item) => {
                match fs::remove_file(&item.filename) {
                    Ok(_)   => { todo_items::remove_item_by_id(&mut items, i); },
                    Err(e)  =>
                        print_err!("Error: cannot delete item {}: {}",
                                   item.id, e),
                };
            },
            None    =>
                print_err!("Error: cannot delete item {}: item not found", i),
        };
    } else {
        print_err!("Item ID not set");
    }
}


pub fn dump(items: &Vec<Rc<TodoItem>>) {
    for item in items {
        println!("{:?}", item);
    }
}


pub fn edit_item(items: &Vec<Rc<TodoItem>>,
                 i: i32,
                 editor: &Option<String>) -> TodoResult<()> {
    if i == 0 {
        return Err(TodoError::new(TodoErrorKind::Other,
                                  "Item ID not set".to_string()));
    }

    // TODO: improve error handling when editor is Some, but does not exist
    let editor = match *editor {
        Some(ref path)  => path,
        None            => {
            return Err(TodoError::new(TodoErrorKind::Other,
                                      "Editor not set".to_string()));
        },
    };

    // TODO: better error handling for the None case?
    match todo_items::get_item_by_id(&items, i) {
        Some(item) => {
            // run editor command for item
            match Command::new(OsString::from(editor))
                              .arg(&item.filename)
                              .status() {
                Ok(_)   => {},
                Err(e)  => {
                    print_err!("Error: editor exited with status {}",
                               e);
                },
            };
        },
        None    =>
            print_err!("Error: cannot edit item {}: item not found", i),
    };

    Ok(())
}


fn get_date_today_str() -> Result<String, time::ParseError> {
    util::date_to_str(&time::now())
}


pub fn print_help(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [ACTION]", program);
    print!("{}", opts.usage(&brief));
}


fn print_item(item: &TodoItem) {
    println!("\t[{:3}]: {}", item.id, item.heading);
}


pub fn print_today(items: &Vec<Rc<TodoItem>>) {
    let today_str = match get_date_today_str() {
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
    let today_str = match get_date_today_str() {
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
    // 0 indicates id was not set
    if i != 0 {
        match todo_items::get_item_by_id(&items, i) {
            Some(i) => {
                println!("id: {}", i.id);
                println!("filename: {}", i.filename);
                println!("heading: {}", i.heading);
                println!("\n{}", i.body);
            },
            None    => print_err!("Error: Item {} not found", i),
        };
    } else {
        print_err!("Item ID not set");
    }
}
