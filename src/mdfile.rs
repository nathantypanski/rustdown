// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::{IoResult, Open, Read};
use std::io::fs::File;
use std::path::posix::Path;

pub fn open_markdown_file(path: &Path) -> IoResult<File> {
    File::open_mode(path, Open, Read)
}
