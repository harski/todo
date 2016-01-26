// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::fs;
use std::io;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use time;
use todo_item::TodoItem;


pub fn filter_todays_items(items: &Vec<Rc<TodoItem>>)
                           -> Result<Vec<Rc<TodoItem>>, time::ParseError> {
    let date_today = try!(get_date_today());
    let mut today: Vec<Rc<TodoItem>> = Vec::new();
    for item in items {
        match item.get_date() {
            Some(d) => {
                if d.eq(&date_today) {
                    today.push(item.clone());
                }
            },
            None    => {},
        };
    }
    Ok(today)
}


fn get_date_today() -> Result<String, time::ParseError> {
    let now = time::now();
    match time::strftime("%Y-%m-%d", &now) {
        Ok(d_str)   => Ok(d_str),
        Err(err)    => Err(err),
    }
}


pub fn get_files_in_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    // get file list
    if try!(fs::metadata(dir)).is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            match entry {
                Err(err) => println!("Error: {}", err),
                Ok(dirent) => match dirent.file_type() {
                    Err(err) => println!("could not find file type for file: {} ", err),
                    Ok(ft)   => if ft.is_file() { files.push(dirent.path()) },
                },
            }
        }
    }
    Ok(files)
}


pub fn get_todo_items(path: &Path) -> io::Result<Vec<Rc<TodoItem>>> {
    let mut items: Vec<Rc<TodoItem>> = Vec::new();
    let files = try!(get_files_in_dir(path));
    for file in files {
        match TodoItem::new_from_file(&file) {
            Ok(i)   => items.push(Rc::new(i)),
            Err(err)=> print_err!("Could not load todo file '{:?}': {}", file, err),
        };
    };

    Ok(items)
}