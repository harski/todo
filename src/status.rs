// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

#[derive(Clone,Debug,PartialEq)]
pub enum Status {
    Done,
    Todo,
}


pub fn parse_status_val(val: &str) -> Option<Status> {
    match val {
        "done"  => Some(Status::Done),
        "todo"  => Some(Status::Todo),
        _       => None,
    }
}
