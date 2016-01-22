// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

pub struct TodoItem {
    pub filename:   String,
    pub heading:    String,
}


impl TodoItem {
    pub fn new(filename: &str, heading: &str) -> TodoItem {
        TodoItem {
            filename: filename.to_string(),
            heading: heading.to_string(),
        }
    }


    pub fn new_from_file(file: &Path) -> Result<TodoItem, Error> {
        let filename = file.to_str().unwrap();
        let mut fd = try!(File::open(file));
        let mut contents = String::new();
        try!(fd.read_to_string(&mut contents));

        let mut line_it = contents.lines();

        let heading = match line_it.next() {
            Some(line)  => line,
            None        => return Err(Error::new(ErrorKind::Other, "Heading not found")),
        };
        Ok(TodoItem::new(&filename, &heading))
    }
}
