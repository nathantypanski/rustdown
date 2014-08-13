use std::fmt;
use std::slice::Items;
use std::iter::Iterator;

use rustdoc::html::escape::Escape;

use super::HtmlAttribute;

/// The contents of an HTML tag.
///
#[deriving(Clone)]
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
#[deriving(Clone)]
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
            contents: Some(vec![Bare(contents)]),
            attributes: vec![],
        }
    }

    /// Add a new `Html` element to this HTML tag.
    ///
    pub fn add_tag(&mut self, tag: Html) {
        match self.contents {
            Some(ref mut contents) => {
                contents.push(Tag(tag));
            },
            None => {
                self.contents = Some(vec![Tag(tag)]);
            }
        }
    }

    /// Add a `String` to this HTML tag.
    ///
    pub fn add_string(&mut self, s: String) {
        match self.contents {
            Some(ref mut contents) => {
                contents.push(Bare(s));
            },
            None => {
                self.contents = Some(vec![Bare(s)]);
            }
        }
    }

    pub fn slice_contents<'a>(&'a self) -> Option<&'a [HtmlContents]> {
        match self.contents {
            Some(ref contents) => Some(contents.as_slice()),
            None => None,
        }
    }
}

pub trait ToHtml {
    fn to_html(&self) -> Html;
}

impl fmt::Show for Html {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = Escape(self.name.as_slice());
        write!(fmt, "<{}", name);
        for attr in self.attributes.iter() {
            write!(fmt, " {}", attr);
        }
        match self.slice_contents() {
            Some(contents) => {
                write!(fmt, ">");
                for elem in contents.iter() {
                    match elem {
                        &Tag(ref html) => {
                            // Multiple nested HTML elements.
                            write!(fmt, "{}", html);
                        }
                        &Bare(ref s) => {
                            write!(fmt, "{}", Escape(s.as_slice()));
                        }
                    }
                };
                write!(fmt, "</{}>", name);
            }
            None => {
                // Nothing! Close the tag.
                write!(fmt, " />");
            }
        }
        Ok(())
    }
}
