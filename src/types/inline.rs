// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use url::Url;

enum Contents {
    Italic(String, Box<Contents>),
    Bold(String, Box<Contents>),
    Code(String, Box<Contents>),
    Normal(String, Box<Contents>),
    Link(String, Url, Box<Contents>),
    Nil
}

impl Contents {
    fn new(s: String) -> Contents {
        Contents::Normal(s, box Contents::Nil)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

    }
}
