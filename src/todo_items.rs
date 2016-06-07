// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::fs;
use std::io;
use std::rc::Rc;
use std::path::{Path, PathBuf};

use status::Status;
use todo_item::TodoItem;


pub fn get_dateless_items(items: &Vec<Rc<TodoItem>>)
                          -> Vec<Rc<TodoItem>> {
    let mut dateless: Vec<Rc<TodoItem>> = Vec::new();
    for item in items {
        if item.date.is_none() {
            dateless.push(item.clone());
        }
    }
    dateless
}


pub fn get_files_in_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    // get file list
    if try!(fs::metadata(dir)).is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            match entry {
                Err(err) => println!("Error: {}", err),
                Ok(dirent) => match dirent.file_type() {
                    Err(err) => println!("could not find file type for \
                                         file '{}'",
                                         err),
                    Ok(ft)   => if ft.is_file() { files.push(dirent.path()) },
                },
            }
        }
    }
    Ok(files)
}


pub fn get_item_by_id(items: &Vec<Rc<TodoItem>>, i: i32)
                     -> Option<Rc<TodoItem>> {
    for item in items {
        if item.id == i {
            return Some(item.clone());
        }
    };
    None
}


pub fn get_items_on_date(items: &Vec<Rc<TodoItem>>, date_str: &str)
                        -> Vec<Rc<TodoItem>> {
    let mut today: Vec<Rc<TodoItem>> = Vec::new();
    for item in items {
        match item.get_date_str() {
            Some(d) => {
                if d.eq(&date_str) {
                    today.push(item.clone());
                }
            },
            None    => {},
        };
    }
    today
}


pub fn get_items_after(items: &Vec<Rc<TodoItem>>, date_str: &str)
                      -> Vec<Rc<TodoItem>> {
    let mut list: Vec<Rc<TodoItem>> = Vec::new();
    for item in items {
        if let Some(i_date) = item.get_date_str() {
            if &i_date[..] > date_str {
                list.push(item.clone());
            }
        }
    }
    list
}


pub fn get_items_before(items: &Vec<Rc<TodoItem>>, date_str: &str)
                       -> Vec<Rc<TodoItem>> {
    let mut list: Vec<Rc<TodoItem>> = Vec::new();
    for item in items {
        if let Some(i_date) = item.get_date_str() {
            if &i_date[..] < date_str {
                list.push(item.clone());
            }
        }
    }
    list
}


pub fn get_todo_items(path: &Path) -> io::Result<Vec<Rc<TodoItem>>> {
    let mut items: Vec<Rc<TodoItem>> = Vec::new();
    let files = try!(get_files_in_dir(path));
    for (id, file) in (1..).zip(files.iter()) {
        match TodoItem::new_from_file(&file, id) {
            Ok(i)   => items.push(Rc::new(i)),
            Err(err)=> print_err!("Could not load todo file '{:?}': {}",
                                  file, err),
        };
    };

    // Sort items here, so filtered items will be "automatically" in order too
    items.sort();
    Ok(items)
}


// TODO: None does not end up to undone items?
pub fn get_undone_items(items: &Vec<Rc<TodoItem>>) -> Vec<Rc<TodoItem>> {
    let mut undone: Vec<Rc<TodoItem>> = Vec::new();
    for item in items {
        match item.status {
            Some(ref s) => {
                if *s == Status::Todo { undone.push(item.clone()); }
            },
            None    => undone.push(item.clone()),
        };
    };
    undone
}


pub fn remove_item_by_id(items: &mut Vec<Rc<TodoItem>>, i: i32)
                        -> Option<Rc<TodoItem>> {
    items.iter().position(|ref p| p.id == i).map(|e| items.remove(e))
}
