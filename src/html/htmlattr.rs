use rustdoc::html::escape::Escape;
use std::fmt;

/// A HTML attribute.
///
/// For example, in the HTML `<a href="http://example.com">Example</a>`,
/// `href` would be the `name` of the `HtmlAttribute`, and
/// `http://example.com` would be the `HtmlAttribute`'s contents.
///
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
