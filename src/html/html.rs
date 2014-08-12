use std::fmt;

use rustdoc::html::escape::Escape;

use super::HtmlAttribute;

/// The contents of an HTML tag.
///
/// Three possibilities exist:
///
/// 1. Nothing. This makes the tag close early, e.g., <br />.
/// 2. A string. Provided for convenience.
/// 3. A nested list of more HTML tags. The simplest example of this would
///    be a nested set, like:
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
#[deriving(Clone)]
pub enum HtmlContents {
    Bare(String),
    Tags(Vec<Html>),
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
    pub fn new(name: String,
               contents: HtmlContents,
               attributes: Vec<HtmlAttribute>) -> Html {
        Html {
            name: name,
            contents: contents,
            attributes: attributes,
        }
    }

    pub fn new_empty(name: String) -> Html {
        Html {
            name: name,
            contents: Empty,
            attributes: vec![],
        }
    }

    pub fn new_simple(name: String, contents: String) -> Html {
        Html {
            name: name,
            contents: Bare(contents),
            attributes: vec![],
        }
    }

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
                write!(fmt, ">");
                write!(fmt, "{}", Escape(contents.as_slice()));
                write!(fmt, "</{}>", name);
            }
            Tags(ref html) => {
                for elem in html.iter() {
                    write!(fmt, "{}", elem);
                }
            }
            Empty => {
                write!(fmt, " />");
            }
        }
        Ok(())
    }
}
