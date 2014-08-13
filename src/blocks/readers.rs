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

use super::blocks::Blocks;

pub fn blockify_file(file: File) -> Blocks {
    let mut reader = BufferedReader::new(file);
    read_to_blocks(reader)
}

pub fn read_to_blocks<R: Reader>(mut reader: BufferedReader<R>) -> Blocks {
    FromIterator::from_iter(reader.lines().map(|e| e.unwrap()))
}
