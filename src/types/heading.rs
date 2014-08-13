use std::cmp::PartialEq;

use html::ToHtml;
use html::Html;
use blocks::Block;
use generic;

macro_rules! parse (
    ($e:expr) => (match $e { Some(e) => return Some(e), None => () })
)

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
        Html::new_simple(name, self.contents.clone())
    }
}

pub fn parse_heading(block: &Block) -> Option<Heading> {
    parse!(pound_heading(block));
    parse!(line_heading(block));
    None
}


fn pound_heading(b: &Block) -> Option<Heading> {
    if b.len() != 1 { return None }
    let s = b[0].as_slice();
    match generic::starting_chars(s, '#') {
        Some((title, count)) => Some(Heading::new(title.to_string(), count)),
        None => None,
    }
}

fn line_heading(b: &Block) -> Option<Heading> {
    if b.len() != 2 { return None }
    let mut depth = 0u;
    if generic::all_chars_are('=', b[1].as_slice()) {
        return Some(Heading::new(b[0].to_string(), 1));
    }
    if generic::all_chars_are('-', b[1].as_slice()) {
        return Some(Heading::new(b[0].to_string(), 2));
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::Heading;
    use super::pound_heading;
    use super::line_heading;
    use blocks::Block;
    use html::ToHtml;

    fn pound_heading_equals(input: &str, result: &str) {
        let block = &Block::new_oneline(input.to_string());
        match pound_heading(block) {
            Some(heading) => {
                assert_eq!(format!("{}", heading.to_html()), result.to_string());
            }
            None => { fail!("Didn't count as a heading") }
        }
    }

    #[test]
    fn test_pound_heads() {
        assert_eq!(pound_heading(&Block::new_oneline("# Hello, world".to_string())),
                   Some(Heading {
                       contents: "Hello, world".to_string(),
                       depth: 1,
                   })
                   );
        assert_eq!(pound_heading(&Block::new_oneline("## Hello again, world!".to_string())),
                   Some(Heading {
                       contents: "Hello again, world!".to_string(),
                       depth: 2,
                   }));
    }

    fn test_line_heads() {
        assert_eq!(line_heading(&Block::new_oneline("Hello\n=====".to_string())),
                   Some(Heading {
                       contents: "Hello".to_string(),
                       depth: 1,
                   }));
        assert_eq!(line_heading(&Block::new_oneline("Hello\n-----".to_string())),
                   Some(Heading {
                       contents: "Hello".to_string(),
                       depth: 2,
                   }));
        assert_eq!(line_heading(&Block::new_oneline("Hello\n =====".to_string())),
                   None);
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
        assert_eq!(pound_heading(&Block::new_oneline("Hello, world".to_string())), None);
        assert_eq!(pound_heading(&Block::new_oneline(" ## Hello, world".to_string())), None);
    }
}
