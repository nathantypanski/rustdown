use std::fmt;

use rustdoc::html::escape::Escape;

use super::HtmlAttribute;

#[deriving(Clone)]
pub enum HtmlContents {
    Bare(String),
    Tags(Vec<Html>),
    Empty,
}

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

    pub fn new_simple(name: String, contents: String) -> Html {
        Html {
            name: name,
            contents: Bare(contents),
            attributes: vec![],
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
