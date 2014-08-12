use rustdoc::html::escape::Escape;
use std::fmt;

#[deriving(Clone)]
pub struct HtmlAttribute {
    name: String,
    contents: String,
}

impl fmt::Show for HtmlAttribute {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}={}", Escape(self.name.as_slice()),
                             Escape(self.contents.as_slice()))
    }
}
