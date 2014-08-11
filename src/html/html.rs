use std::fmt;

use rustdoc::html::escape::Escape;

use super::HtmlAttribute;

pub struct Html<'a> {
    name: String,
    contents: Option<Escape<'a>>,
    attributes: Vec<HtmlAttribute<'a>>,
}

impl<'a> Html<'a> {
    pub fn new<'a>(name: String,
               contents: Option<Escape<'a>>,
               attributes: Vec<HtmlAttribute<'a>>) -> Html<'a> {
        Html {
            name: name,
            contents: contents,
            attributes: attributes,
        }
    }

    pub fn new_simple<'a>(name: String, contents: &'a str) -> Html<'a> {
        Html {
            name: name,
            contents: Some(Escape(contents)),
            attributes: vec![],
        }
    }
}

pub trait ToHtml {
    fn to_html(&self) -> Html;
}

impl<'a> fmt::Show for Html<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = Escape(self.name.as_slice());
        try!(fmt.write(format!("<{}", name).into_bytes().as_slice()));
        for attr in self.attributes.iter() {
            try!(fmt.write(format!(" {}", attr).into_bytes().as_slice()));
        }
        match self.contents {
            Some(contents) => {
                try!(fmt.write(format!(">").into_bytes().as_slice()));
                try!(fmt.write(format!("{}", contents).into_bytes().as_slice()));
                try!(fmt.write(format!("</{}>", name).into_bytes().as_slice()));
            }
            None => {
                try!(fmt.write(format!(" />").into_bytes().as_slice()));
            }
        }
        Ok(())
    }
}
