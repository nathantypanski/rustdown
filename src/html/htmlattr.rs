use rustdoc::html::escape::Escape;
use std::fmt;

pub struct HtmlAttribute<'a> {
    name: Escape<'a>,
    contents: Escape<'a>,
}

impl<'a> fmt::Show for HtmlAttribute<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let format = format!("{}={}", self.name, self.contents);
        fmt.write(format.into_bytes().as_slice())
    }
}
