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


impl TodoItem {
    pub fn get_date_str(&self) -> Option<String> {
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


    pub fn new(id: i32, filename: String) -> TodoItem {
        TodoItem {
            attrs:      Vec::new(),
            body:       "".to_string(),
            date:       None,
            filename:   filename,
            heading:    "".to_string(),
            id:         id,
        }
    }


    pub fn new_from_file(file: &Path, id: i32) -> Result<TodoItem, Error> {
        // TODO: use combinators?
        let filename = file.to_str().unwrap();
        let mut fd = try!(File::open(file));
        let mut contents = String::new();
        try!(fd.read_to_string(&mut contents));

        let mut line_it = contents.lines();

        // init temporary TodoItem
        let mut item = TodoItem::new(id, filename.to_string());

        // get heading
        item.heading = match line_it.next() {
            Some(line) => {
                if line.trim().len() > 0 {
                    line.trim().to_string()
                } else {
                    return Err(Error::new(ErrorKind::Other, "Heading empty"))
                }
            },
            None => return Err(Error::new(ErrorKind::Other, "Heading not found")),
        };

        // get attributes
        // TODO: don't add know attributes to the vector
        //       (fix the "date" hack below)
        while let Some(line) = line_it.next() {
            // check if line is body separator
            if line.len() == 0 {
                break;
            }

            match Attr::new_from_line(line) {
                Ok(attr)    => {
                    item.attrs.push(attr);
                },
                Err(err)    => { print_err!("{}", err); },
            };
        }

        // get body
        while let Some(line) = line_it.next() {
            item.body = item.body + line + "\n";
        }

        // TODO: parse this in general attr parser
        item.date = match get_date_from_attrs(&mut item.attrs) {
            Ok(d)   => Some(d),
            Err(e)  => {
                if e.kind().eq(&ErrorKind::NotFound) {
                    None
                } else {
                    print_err!("Can't parse date for {}: {}", item.heading, e);
                    return Err(Error::new(ErrorKind::Other, "Could not initialize date"));
                }
            },
        };

        Ok(item)
    }
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
