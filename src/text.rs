// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::CharEq;
use std::fmt::Show;


/// If string `s` starts with `c`, then return the remaining characters
/// after `c` has been trimmed from the beginning, along with the number
/// of occurrences of `c` in the beginning of string `s`.
pub fn starting_chars(s: &str, c: char) -> Option<(String, uint)> {
    let mut result = None;
    if s.trim_chars(c) != s {
        let mut count = 0u;
        let mut found = false;
        let words: String = s.chars().filter_map(
            |letter| {
                match letter {
                    l if c == letter && !found => {
                        count += 1;
                        None
                    }
                    other => {
                        found = true;
                        Some(other)
                    }
                }
            }
        ).collect();
        result = Some((words.as_slice().trim_left_chars(' ').to_string(), count));
    }
    return result;
}

pub fn all_chars_are(c: char, s: &str) -> bool {
    let cs = c.to_string();
    let ch = cs.as_slice();
    if s.starts_with(ch) {
        for badchar in s.as_slice().chars().filter_map(
                    |e| if e == c { None } else { Some(c) }) {
            return false;
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::starting_chars;
    use super::all_chars_are;

    #[test]
    fn test_all_chars_are_fail() {
        assert!(!all_chars_are('-', "- This is a bullet"));
        assert!(!all_chars_are('-', ""));
        assert!(!all_chars_are('a', "aaaba"));
        assert!(!all_chars_are('a', "aaab"));
        assert!(!all_chars_are('a', "baaa"));
    }

    #[test]
    fn test_all_chars_are_pass() {
        assert!(all_chars_are('-', "------------"));
        assert!(all_chars_are('=', "="));
    }

    #[test]
    fn test_starting_chars() {
        assert_eq!(starting_chars("### haha", '#'), Some(("haha".to_string(), 3)));
        assert_eq!(starting_chars("    - bullet", ' '), Some(("- bullet".to_string(), 4)));
    }
}
