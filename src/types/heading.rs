use std::cmp::PartialEq;

use rustdoc::html::escape::Escape;

use html::ToHtml;
use html::Html;
use blocks::Block;
use blocks::FromBlock;

#[deriving(Show)]
pub struct Heading {
    contents: String,
    depth: uint,
}

impl Heading {
    pub fn new(title: String, depth: uint) -> Heading {
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
        Html::new_simple(name, self.contents.as_slice())
    }
}

impl FromBlock for Heading {
    fn from_block(block: Block) -> Option<Heading> {
        if block.len() != 1 {
            return None;
        }
        pound_heading(block[0].as_slice())
    }
}

fn pound_heading(s: &str) -> Option<Heading> {
    let mut result = None;
    if s.starts_with("#") {
        let mut count = 0u;
        let title: String = s.chars().filter_map(
            |c|
            match c {
                '#' => {
                    count += 1;
                    None
                }
                c => {
                    Some(c)
                }
            }
        ).collect();
        let title: &str = title.as_slice().trim_left_chars(' ');
        result = Some(Heading::new(title.to_string(), count));
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::{Heading, pound_heading};
    use html::ToHtml;

    fn pound_heading_equals(input: &str, result: &str) {
        match pound_heading(input) {
            Some(heading) => {
                assert_eq!(format!("{}", heading.to_html()), result.to_string());
            }
            None => { fail!("Didn't count as a heading") }
        }
    }

    #[test]
    fn test_pound_heads() {
        assert_eq!(pound_heading("# Hello, world"),
                   Some(Heading {
                       contents: "Hello, world".to_string(),
                       depth: 1,
                   })
                   );
        assert_eq!(pound_heading("## Hello again, world!"),
                   Some(Heading {
                       contents: "Hello again, world!".to_string(),
                       depth: 2,
                   }));
    }

    #[test]
    fn test_heads_fmt() {
        pound_heading_equals("# Hello, world", "<h1>Hello, world</h1>");
        pound_heading_equals("## Hello again, world!",
                             "<h2>Hello again, world!</h2>");
        pound_heading_equals("## <h2>Hello again, world!</h2>",
                             "<h2>&lt;h2&gt;Hello again, world!&lt;/h2&gt;</h2>");
    }

    #[test]
    fn test_non_pound_heads() {
        assert_eq!(pound_heading("Hello, world"), None);
        assert_eq!(pound_heading(" ## Hello, world"), None);
    }
}
