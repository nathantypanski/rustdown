use std::fmt;

use rustdoc::html::escape::Escape;

use super::HtmlAttribute;

/// The contents of an HTML tag.
///
#[deriving(Clone)]
pub enum HtmlContents {
/// Just a string, provided for convenience.
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
    Tags(Vec<Html>),
/// Nothing. This also makes the tag close early, e.g., <br />.
///
    Empty,
}

/// An HTML tag.
///
/// These can have attributes, as well as nested contents.
///
#[deriving(Clone)]
pub struct Html {
    name: String,
    contents: HtmlContents,
    attributes: Vec<HtmlAttribute>,
}

impl Html {
    /// Create a new HTML tag, given its name, contents,
    /// and attributes.
    ///
    pub fn new(name: String,
               contents: HtmlContents,
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
            contents: Empty,
            attributes: vec![],
        }
    }

    /// Create a new HTML tag with `Bare` contents and a name.
    ///
    pub fn new_simple(name: String, contents: String) -> Html {
        Html {
            name: name,
            contents: Bare(contents),
            attributes: vec![],
        }
    }

    /// Add a new `Html` element to this HTML tag. This assumes the `contents`
    /// of this tag are either `Tags` or `Empty`, and will return an `Err` if
    /// that is not the case.
    ///
    pub fn add_tag(&mut self, tag: Html) -> Result<(), String> {
        match self.contents {
            Bare(_) => {
                Err("This elment had strings already!".to_string())
            },
            Empty => {
                self.contents = Tags(vec![tag]);
                Ok(())
            }
            Tags(ref mut list) => {
                list.push(tag);
                Ok(())
            }
        }
    }

    /// Add a `String` to this HTML tag. This assumes the `contents` of this tag are
    /// `Empty`, and will return an `Err` if that is not the case.
    ///
    pub fn add_string(&mut self, s: String) -> Result<(), String> {
        match self.contents {
            Bare(_) => {
                Err("This elment had strings already!".to_string())
            },
            Empty => {
                self.contents = Bare(s);
                Ok(())
            }
            Tags(ref mut list) => {
                Err("This elment has tags!".to_string())
            }
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
        match self.contents {
            Bare(ref contents) => {
                // Just a tag with contents
                write!(fmt, ">");
                write!(fmt, "{}", Escape(contents.as_slice()));
                write!(fmt, "</{}>", name);
            }
            Tags(ref html) => {
                // Multiple nested HTML elements.
                write!(fmt, ">");
                for elem in html.iter() {
                    write!(fmt, "{}", elem);
                }
                write!(fmt, "</{}>", name);
            }
            Empty => {
                // Nothing! Close the tag.
                write!(fmt, " />");
            }
        }
        Ok(())
    }
}
