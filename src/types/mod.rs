// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::bulletlist::Bullet;
pub use self::bulletlist::BulletList;
pub use self::heading::Heading;
pub use self::paragraph::Paragraph;
pub use self::heading::parse_heading;
pub use self::paragraph::parse_paragraph;
pub use self::bulletlist::parse_bulletlist;
use html::Html;
use html::ToHtml;

mod inline;
mod heading;
mod paragraph;
mod bulletlist;

#[deriving(Show)]
pub enum MarkdownStructure {
    MDH(Heading),
    MDP(Paragraph),
    MDB(BulletList),
}

impl MarkdownStructure {

}

impl ToHtml for MarkdownStructure {
    fn to_html(&self) -> Html {
        match self {
            &MarkdownStructure::MDH(ref heading) => heading.to_html().clone(),
            &MarkdownStructure::MDP(ref paragraph) => paragraph.to_html().clone(),
            &MarkdownStructure::MDB(ref bulletlist) => bulletlist.to_html().clone(),
        }
    }
}
