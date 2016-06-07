// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use time;
use time::Tm;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(e) = write!(&mut ::std::io::stderr(),
                                   "{}\n",
                                   format_args!($($arg)*)) {
                panic!("Failed to write to stderr.\
                       \nOriginal error output: {}\
                       \nSecondary error writing to stderr: {}",
                       format!($($arg)*), e);
            }
        }
    )
}


pub fn date_to_str(date: &Tm) -> Result<String, time::ParseError> {
    time::strftime("%Y-%m-%d", &date)
}
