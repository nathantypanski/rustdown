// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use rustdoc::html::escape::Escape;

use super::HtmlAttribute;

/// The contents of an HTML tag.
///
#[derive(Clone)]
pub enum HtmlContents {
    /// Just a string. For example, in
    ///
    ///    <p>Hi <a href="http://example.com>there</a></p>
    ///
    /// The contents of the `<p>` are a `Bare` (string) and then a `Tag`.
    ///
    Bare(String),
    /// A nested list of more HTML tags. A simple example of this would
    /// be a nested set, like:
    ///
    ///    ```
    ///    <ul>
    ///        <li>Item1</li>
    ///        <ul>
    ///            <li>Indented item</li>
    ///        </ul>
    ///    </ul>
    ///    ```
    ///
    Tag(Html),
}

/// An HTML tag.
///
/// These can have attributes, as well as nested contents.
///
#[derive(Clone)]
pub struct Html {
    name: String,
    contents: Option<Vec<HtmlContents>>,
    attributes: Vec<HtmlAttribute>,
}

impl Html {
    /// Create a new HTML tag, given its name, contents,
    /// and attributes.
    ///
    pub fn new(name: String,
               contents: Option<Vec<HtmlContents>>,
               attributes: Vec<HtmlAttribute>) -> Html {
        Html {
            name: name,
            contents: contents,
            attributes: attributes,
        }
    }

    /// Create a new HTML tag, given just a name. The contents and attributes
    /// will be initialized as empty.
    ///
    pub fn new_empty(name: String) -> Html {
        Html {
            name: name,
            contents: None,
            attributes: vec![],
        }
    }

    /// Create a new HTML tag with `Bare` contents and a name.
    ///
    pub fn new_simple(name: String, contents: String) -> Html {
        Html {
            name: name,
            contents: Some(vec![HtmlContents::Bare(contents)]),
            attributes: vec![],
        }
    }

    /// Add a new `Html` element to this HTML tag.
    ///
    pub fn add_tag(&mut self, tag: Html) {
        match self.contents {
            Some(ref mut contents) => {
                contents.push(HtmlContents::Tag(tag));
            },
            None => {
                self.contents = Some(vec![HtmlContents::Tag(tag)]);
            }
        }
    }

    /// Add a `String` to this HTML tag.
    ///
    pub fn add_string(&mut self, s: String) {
        match self.contents {
            Some(ref mut contents) => {
                contents.push(HtmlContents::Bare(s));
            },
            None => {
                self.contents = Some(vec![HtmlContents::Bare(s)]);
            }
        }
    }

    pub fn slice_contents<'a>(&'a self) -> Option<&'a [HtmlContents]> {
        match self.contents {
            Some(ref contents) => Some(contents),
            None => None,
        }
    }
}

pub trait ToHtml {
    fn to_html(&self) -> Html;
}

impl fmt::Display for Html {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = Escape(&self.name);
        try!(write!(fmt, "<{}", name));
        for attr in self.attributes.iter() {
            try!(write!(fmt, " {}", attr))
        }
        match self.slice_contents() {
            Some(contents) => {
                try!(write!(fmt, ">"));
                for elem in contents.iter() {
                    match elem {
                        &HtmlContents::Tag(ref html) => {
                            // Multiple nested HTML elements.
                            try!(write!(fmt, "{}", html))
                        }
                        &HtmlContents::Bare(ref s) => {
                            try!(write!(fmt, "{}", Escape(s)))
                        }
                    }
                };
                try!(write!(fmt, "</{}>", name))
            }
            None => {
                // Nothing! Close the tag.
                try!(write!(fmt, " />"))
            }
        }
        Ok(())
    }
}
