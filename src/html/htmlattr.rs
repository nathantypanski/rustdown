// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustdoc::html::escape::Escape;
use std::fmt;

/// A HTML attribute.
///
/// For example, in the HTML `<a href="http://example.com">Example</a>`,
/// `href` would be the `name` of the `HtmlAttribute`, and
/// `http://example.com` would be the `HtmlAttribute`'s contents.
///
#[derive(Clone)]
pub struct HtmlAttribute {
    name: String,
    contents: String,
}

impl fmt::Display for HtmlAttribute {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}={}", Escape(&self.name),
                             Escape(&self.contents))
    }
}
