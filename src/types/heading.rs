// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::PartialEq;

use html::ToHtml;
use html::Html;
use text;

macro_rules! parse (
    ($e:expr) => (match $e { Some(e) => return Some(e), None => () })
);

/// A markdown text heading. `depth` signifies the level of the heading, e.g.,
/// `# head` is depth 1, `## head` is depth 2, and so on.
///
#[derive(Debug)]
pub struct Heading {
    contents: String,
    depth: u32,
}

impl Heading {
    pub fn new(title: String, depth: u32) -> Heading {
        Heading {
            contents: title,
            depth: depth,
        }
    }
}

impl PartialEq for Heading {
    fn eq(&self, other: &Heading) -> bool {
        self.depth == other.depth && self.contents == other.contents
    }

    fn ne(&self, other: &Heading) -> bool {
        !self.eq(other)
    }
}

impl ToHtml for Heading {
    fn to_html(&self) -> Html {
        let name: String = format!("h{}", self.depth);
        Html::new_simple(name, self.contents.clone())
    }
}

/// Parse all the different types of headings, regardless
/// of how they're formatted. Return `Some(heading)` if one was found
/// in this block.
pub fn parse_heading(block: &Vec<String>) -> Option<Heading> {
    parse!(pound_heading(block));
    parse!(line_heading(block));
    None
}

/// A heading created with a pound sign, like:
///
///     # Heading
///
fn pound_heading(b: &Vec<String>) -> Option<Heading> {
    if b.len() != 1 { return None }
    let s = &b[0];
    match text::starting_chars(s, '#') {
        Some((title, count)) => Some(Heading::new(title.to_string(), count)),
        None => None,
    }
}

/// A heading created with an underline, like:
///
///     Heading
///     =======
///
fn line_heading(b: &Vec<String>) -> Option<Heading> {
    if b.len() != 2 { return None }
    let mut depth = 0usize;
    if text::all_chars_are('=', &b[1]) {
        return Some(Heading::new(b[0].to_string(), 1));
    }
    if text::all_chars_are('-', &b[1]) {
        return Some(Heading::new(b[0].to_string(), 2));
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::Heading;
    use super::pound_heading;
    use super::line_heading;
    use html::ToHtml;

    fn pound_heading_equals(input: &str, result: &str) {
        let block = &vec![input.to_string()];
        match pound_heading(block) {
            Some(heading) => {
                assert_eq!(format!("{}", heading.to_html()), result.to_string());
            }
            None => { panic!("Didn't count as a heading") }
        }
    }

    #[test]
    fn test_pound_heads() {
        assert_eq!(pound_heading(&vec!["# Hello, world".to_string()]),
                   Some(Heading {
                       contents: "Hello, world".to_string(),
                       depth: 1,
                   })
                   );
        assert_eq!(pound_heading(&vec!("## Hello again, world!".to_string())),
                   Some(Heading {
                       contents: "Hello again, world!".to_string(),
                       depth: 2,
                   }));
    }

    fn test_line_heads() {
        assert_eq!(line_heading(&vec!("Hello\n=====".to_string())),
                   Some(Heading {
                       contents: "Hello".to_string(),
                       depth: 1,
                   }));
        assert_eq!(line_heading(&vec!("Hello\n-----".to_string())),
                   Some(Heading {
                       contents: "Hello".to_string(),
                       depth: 2,
                   }));
        assert_eq!(line_heading(&vec!("Hello\n =====".to_string())),
                   None);
    }

    #[test]
    fn test_pound_heads_fmt() {
        pound_heading_equals("# Hello, world", "<h1>Hello, world</h1>");
        pound_heading_equals("## Hello again, world!",
                             "<h2>Hello again, world!</h2>");
        pound_heading_equals("## <h2>Hello again, world!</h2>",
                             "<h2>&lt;h2&gt;Hello again, world!&lt;/h2&gt;</h2>");
    }

    #[test]
    fn test_non_pound_heads() {
        assert_eq!(pound_heading(&vec!("Hello, world".to_string())), None);
        assert_eq!(pound_heading(&vec!(" ## Hello, world".to_string())), None);
    }
}
