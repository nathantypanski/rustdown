pub use self::bulletlist::Bullet;
pub use self::heading::Heading;
pub use self::paragraph::Paragraph;
pub use self::heading::parse_heading;
pub use self::paragraph::parse_paragraph;
use html::Html;
use html::ToHtml;

mod heading;
mod paragraph;
mod bulletlist;

#[deriving(Show)]
pub enum MarkdownStructure {
    MDH(Heading),
    MDP(Paragraph),
}

impl MarkdownStructure {

}

impl ToHtml for MarkdownStructure {
    fn to_html(&self) -> Html {
        match self {
            &MDH(ref heading) => heading.to_html().clone(),
            &MDP(ref paragraph) => paragraph.to_html().clone(),
        }
    }
}

macro_rules! parse (
    ($e:expr) => (match $e { Some(e) => return Some(e), None => () })
)
