// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::error::Error;
use std::io;
use std::fmt;

pub type TodoResult<T> = Result<T, TodoError>;

pub struct TodoError {
    kind: TodoErrorKind,
    message: String
}

pub enum TodoErrorKind {
    Other,
    Parse
}

impl TodoError {
    pub fn new(kind: TodoErrorKind, msg: String)
           -> TodoError {
        TodoError {
            kind: kind,
            message: msg
        }
    }
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{})", self.message)
    }
}

impl From<io::Error> for TodoError {
    fn from(err: io::Error) -> TodoError {
        TodoError { kind: TodoErrorKind::Other, message: err.description().to_string() }
    }
}
