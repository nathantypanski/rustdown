// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;

pub fn replace_tabs(lines: &Vec<String>) -> Vec<String> {
    lines.iter().map(|s| str::replace("\t", "    ", s.as_slice())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_string(vx: Vec<str>, vy: Vec<str>) {
        assert_eq!(vx.iter().map(|x| (*x).to_string()).collect(),
                   vy.iter().map(|x| (*x).to_string()).collect())
    }

    #[test]
    fn test_replace_tabs() {
        assert_string(replace_tabs(&vec!("Hi")), "Hi");
    }
}
