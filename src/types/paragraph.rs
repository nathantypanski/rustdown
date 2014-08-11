use html::ToHtml;
use html::Html;
use blocks::Block;
use blocks::FromBlock;

#[deriving(Show)]
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
        Html::new_simple("p".to_string(), self.contents.as_slice())
    }
}

impl FromBlock for Paragraph {
    fn from_block(block: Block) -> Option<Paragraph> {
        Some(
            Paragraph::new(
                block.iter().fold(
                    "".to_string(),
                    |res, line| { res + "\n".to_string() + line.to_string() }
                )
            )
        )
    }
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
    }
}
