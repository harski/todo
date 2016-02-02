// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

use time;
use time::Tm;

use attr::Attr;

macro_rules! try_opt(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

#[derive(Clone,Debug)]
pub struct TodoItem {
    pub attrs:      Vec<Attr>,
    pub body:       String,
    pub date:       Option<Tm>,
    pub filename:   String,
    pub heading:    String,
    pub id:         i32,
}


fn get_date_from_attrs(attrs: &mut Vec<Attr>) -> Result<Tm, Error> {
    // TODO: Use combinators instead
    for attr in attrs {
        if attr.key.eq("date") {
            return match time::strptime(&attr.value, "%Y-%m-%d") {
                Ok(d)   => Ok(d),
                Err(e)  => Err(Error::new(ErrorKind::Other, e)),
            };
        }
    }
    Err(Error::new(ErrorKind::NotFound, ""))
}


impl TodoItem {
    pub fn get_date(&self) -> Option<String> {
        // change Tm to str
        // TODO: Use combinators instead
        let ds = match self.date {
            Some(d) => {
                 match time::strftime("%Y-%m-%d", &d) {
                     Ok(ds) => Some(ds),
                     _      => None,
                 }
            },
            None    => None,
        };

        ds
    }


    pub fn new(attrs: Vec<Attr>, body: &str, date: Option<Tm>,
               filename: &str, heading: &str, id: i32) -> TodoItem {
        TodoItem {
            attrs:      attrs,
            body:       body.to_string(),
            date:       date,
            filename:   filename.to_string(),
            heading:    heading.to_string(),
            id:         id,
        }
    }


    pub fn new_from_file(file: &Path, id: i32) -> Result<TodoItem, Error> {
        let filename = file.to_str().unwrap();
        let mut fd = try!(File::open(file));
        let mut contents = String::new();
        try!(fd.read_to_string(&mut contents));

        let mut line_it = contents.lines();

        // get heading
        let heading = match line_it.next() {
            Some(line) => {
                if line.trim().len() > 0 {
                    line
                } else {
                    return Err(Error::new(ErrorKind::Other, "Heading empty"))
                }
            },
            None => return Err(Error::new(ErrorKind::Other, "Heading not found")),
        };

        // get attributes
        let mut attrs = Vec::new();
        while let Some(line) = line_it.next() {
            // check if line is body separator
            if line.len() == 0 {
                break;
            }

            match Attr::new_from_line(line) {
                Ok(attr)    => { attrs.push(attr); },
                Err(err)    => { print_err!("{}", err); },
            };
        }

        // get body
        let mut body = String::new();
        while let Some(line) = line_it.next() {
            body = body + line + "\n";
        }

        // TODO: get_date_from_attrs
        let date = match get_date_from_attrs(&mut attrs) {
            Ok(d)   => Some(d),
            Err(e)  => {
                if e.kind().eq(&ErrorKind::NotFound) {
                    None
                } else {
                    print_err!("Can't parse date for {}: {}", heading, e);
                    return Err(Error::new(ErrorKind::Other, "Could not initialize date"));
                }
            },
        };

        Ok(TodoItem::new(attrs, body.trim(), date, &filename, &heading, id))
    }
}
