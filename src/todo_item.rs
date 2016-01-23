// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

use attr::Attr;


#[derive(Debug)]
pub struct TodoItem {
    pub filename:   String,
    pub heading:    String,
    pub attrs:      Vec<Attr>,
}


impl TodoItem {
    pub fn new(filename: &str, heading: &str, attrs: Vec<Attr>) -> TodoItem {
        TodoItem {
            filename:   filename.to_string(),
            heading:    heading.to_string(),
            attrs:      attrs,
        }
    }


    pub fn new_from_file(file: &Path) -> Result<TodoItem, Error> {
        let filename = file.to_str().unwrap();
        let mut fd = try!(File::open(file));
        let mut contents = String::new();
        try!(fd.read_to_string(&mut contents));

        let mut line_it = contents.lines();

        let heading = match line_it.next() {
            Some(line)  => {
                if line.trim().len() > 0 {
                    line
                } else {
                    return Err(Error::new(ErrorKind::Other, "Heading empty"))
                }
            },
            None        => return Err(Error::new(ErrorKind::Other, "Heading not found")),
        };

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

        Ok(TodoItem::new(&filename, &heading, attrs))
    }
}
