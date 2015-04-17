// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use html::ToHtml;
use html::Html;

#[derive(Debug)]
pub struct Paragraph {
    contents: String,
}

impl Paragraph {
    pub fn new(contents: String) -> Paragraph {
        Paragraph {
            contents: contents,
        }
    }
}

impl ToHtml for Paragraph {
    fn to_html(&self) -> Html {
        Html::new_simple("p".to_string(), self.contents.clone())
    }
}

pub fn parse_paragraph(block: &Vec<String>) -> Paragraph {
    Paragraph::new(
        block.iter().fold(
            "".to_string(),
            |res, line| res + "\n" + line
        ).trim().to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::{Paragraph};
    use html::ToHtml;

    fn paragraph_equals(input: &str, result: &str) {
        assert_eq!(format!("{}", Paragraph::new(input.to_string()).to_html()),
                   result.to_string());
    }

    #[test]
    fn test_making_paragraphs() {
        paragraph_equals("Hello, World", "<p>Hello, World</p>");
        paragraph_equals("Hello,\nWorld", "<p>Hello,\nWorld</p>");
    }
}
