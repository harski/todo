// Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
// Licensed under the 2-clause BSD license, see LICENSE for details.

use std::io;
use std::fs;
use std::path::{Path, PathBuf};


pub fn get_files_in_dir(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    // get file list
    if try!(fs::metadata(dir)).is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            match entry {
                Err(err) => println!("Error: {}", err),
                Ok(dirent) => match dirent.file_type() {
                    Err(err) => println!("could not find file type for file: {} ", err),
                    Ok(ft)   => if ft.is_file() { files.push(dirent.path()) },
                },
            }
        }
    }
    Ok(())
}
