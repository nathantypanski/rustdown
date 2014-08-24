// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter::FromIterator;
use std::string::String;
use std::slice::Items;
use std::slice::Slice;

#[deriving(Show)]
pub struct Blocks(Vec<Vec<String>>);

impl Blocks {
    pub fn len(&self) -> uint {
        match self { &Blocks(ref blocks) => blocks.len() }
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> Items<'a,Vec<String>> {
        match self { &Blocks(ref blocks) => blocks.as_slice().iter() }
    }
}

impl Index<uint, Vec<String>> for Blocks {
    fn index<'a>(&'a self, index: &uint) -> &'a Vec<String> {
        let block = match self { &Blocks(ref blocks) => blocks.get(*index) };
        block
    }
}


impl Slice<Vec<String>> for Blocks {
    #[inline]
    fn as_slice<'a>(&'a self) -> &'a [Vec<String>] {
        let blocks = match self { &Blocks(ref blocks) => blocks.as_slice() };
        blocks
    }
}

impl FromIterator<String> for Blocks {
    fn from_iter<T: Iterator<String>>(mut iterator: T) -> Blocks {
        let mut blockbuf: Vec<String> = vec![];
        let mut blocks = iterator.fold(vec![], |mut vec, line| {
            if is_block_separator(line.as_slice()) {
                vec.push(blockbuf.clone());
                blockbuf = vec![];
            }
            else {
                blockbuf.push(line.as_slice().trim_right_chars('\n').to_string());
            }
            vec
        });
        blocks.push(blockbuf);
        Blocks(blocks)
    }
}

pub fn is_block_separator(s: &str) -> bool {
    let s = s.trim_right_chars('\n').trim_left_chars(' ').trim_left_chars('\t');
    s == ""
}

#[cfg(test)]
mod tests {
    use super::{is_block_separator};

    #[test]
    fn test_determining_block_separators() {
        assert!(is_block_separator(""));
        assert!(is_block_separator("    "));
        assert!(is_block_separator("\t"));
        assert!(!is_block_separator(" ## t"));
    }
}
