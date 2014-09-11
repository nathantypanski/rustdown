// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::fs::File;
use std::io::BufferedReader;
use std::iter::FromIterator;

pub fn split_file_lines(file: File) -> Vec<String> {
    let mut reader = BufferedReader::new(file);
    read_to_lines(reader)
}

pub fn read_to_lines<R: Reader>(mut reader: BufferedReader<R>) -> Vec<String> {
    FromIterator::from_iter(reader.lines().map(|e| e.unwrap()))
}
