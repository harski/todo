// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::cmp::Ordering;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;
use std::str::Lines;

use time;
use time::Tm;

use attr::Attr;
use status::{Status, parse_status_val};
use util;

macro_rules! try_opt(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

// TODO: make body an Option
#[derive(Clone,Debug)]
pub struct TodoItem {
    pub attrs:      Vec<Attr>,
    pub body:       String,
    pub date:       Option<Tm>,
    pub filename:   String,
    pub heading:    String,
    pub id:         i32,
    pub status:     Option<Status>,
}


impl TodoItem {
    pub fn get_date_str(&self) -> Option<String> {
        // change Tm to str
        self.date.map(|date| util::date_to_str(&date).ok()).unwrap_or(None)
    }


    pub fn new(id: i32, filename: String) -> TodoItem {
        TodoItem {
            attrs:      Vec::new(),
            body:       "".to_string(),
            date:       None,
            filename:   filename,
            heading:    "".to_string(),
            id:         id,
            status:     None,
        }
    }


    pub fn new_from_file(file: &Path, id: i32) -> Result<TodoItem, Error> {
        let filename = file.to_str().unwrap();

        // init temporary TodoItem
        let mut item = TodoItem::new(id, filename.to_string());

        let file_contents = try!(get_file_contents(&file));
        let mut line_it = file_contents.lines();

        item.heading = try!(get_heading(&mut line_it));
        let attrs = get_attrs(filename, &mut line_it);
        get_body(&mut line_it, &mut item.body);

        parse_attrs(&attrs, &mut item);

        Ok(item)
    }
}


impl Eq for TodoItem {}


impl Ord for TodoItem {
    // TODO: compare Tm fields instead of strs
    fn cmp(&self, other: &Self) -> Ordering {
        let sd = self.get_date_str();
        let od = other.get_date_str();

        if sd.is_none() && od.is_none() {
            return Ordering::Equal;
        } else if sd.is_some() && od.is_none() {
            return Ordering::Less;
        } else if sd.is_none() && od.is_some() {
            return Ordering::Greater;
        } else {
            return sd.unwrap().cmp(&od.unwrap());
        }
    }
}


impl PartialEq for TodoItem {
    fn eq(&self, other: &Self) -> bool {
        let sd = &self.date;
        let od = &other.date;

        if sd.is_none() && od.is_none() {
            return true;
        } else if sd.is_some() && od.is_some() &&
                  self.cmp(other) == Ordering::Equal {
                return true;
        }
        false
    }
}


impl PartialOrd for TodoItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn get_attrs(file: &str, line_it: &mut Lines) -> Vec<Attr> {
    let mut attrs: Vec<Attr> = Vec::new();
    while let Some(line) = line_it.next() {
        // check if line is body separator
        if line.len() == 0 {
            break;
        }

        match Attr::new_from_line(line) {
            Ok(attr)    => { attrs.push(attr); },
            Err(err)    => { print_err!("{}: {}", file, err); },
        };
    }

    attrs
}


fn get_body(line_it: &mut Lines, body: &mut String) {
    // TODO: only add '\n' if not last line
    while let Some(line) = line_it.next() {
        body.push_str(&line);
        body.push('\n');
    }
}


fn get_file_contents(file: &Path) -> Result<String, Error> {
         let mut contents = String::new();
         let mut fd = try!(File::open(file));
         try!(fd.read_to_string(&mut contents));
         Ok(contents)
}


fn get_heading(line_it: &mut Lines) -> Result<String, Error> {
    match line_it.next() {
        Some(line) => {
            if line.trim().len() > 0 {
                Ok(line.trim().to_string())
            } else {
                println!("line '{}' is too short", line);
                return Err(Error::new(ErrorKind::Other, "Heading empty"))
            }
        },
        None => return Err(Error::new(ErrorKind::Other, "Heading not found")),
    }
}


fn parse_attrs(attrs: &Vec<Attr>, item: &mut TodoItem) {
    for attr in attrs {
        match &attr.key[..] {
            "date"      => {
                match parse_date(&attr.value) {
                    Ok(date)    => item.date = Some(date),
                    Err(err)    => print_err!("{}: {}", item.filename, err),
                };
            }
            "status"    => {
                item.status = parse_status_val(&attr.value);
            },
            _           => {
                print_err!("{}: invalid attr: key='{}', value='{}'",
                           item.filename, attr.key, attr.value )
            },
        }
    }
}


fn parse_date(date_str: &str) -> Result<Tm, time::ParseError> {
    time::strptime(date_str, "%Y-%m-%d")
}
