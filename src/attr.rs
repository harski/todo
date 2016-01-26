// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.


use std::io::{Error, ErrorKind};

#[derive(Clone,Debug)]
pub struct Attr {
    pub key:    String,
    pub value:  String,
}


impl Attr {
    pub fn new(key: &str, value: &str) -> Attr {
        Attr {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_from_line(line: &str) -> Result<Attr, Error> {
        let mut line_it = line.splitn(2, ':');

        let key = match validate_attr_entry(line_it.next()) {
            Some(key)   => key,
            None        => { return Err(Error::new(ErrorKind::Other, "Not a valid attr line")); },
        };

        let value = match validate_attr_entry(line_it.next()) {
            Some(value)   => value,
            None        => { return Err(Error::new(ErrorKind::Other, "Not a valid attr line")); },
        };

        Ok(Attr::new(&key, &value))
    }
}


fn validate_attr_entry(so: Option<&str>) -> Option<&str> {
    match so {
        Some(val)   => {
            let trimmed = val.trim();
            if trimmed.len() > 0 {
                Some(trimmed)
            } else {
                None
            }
        },
        None    => None,
    }
}
