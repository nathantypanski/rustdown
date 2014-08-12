use html::ToHtml;
use html::Html;

/// An element in a bulleted list.
///
/// *Nested* means there's a sublist,
/// while *Lone* means this is just a list item.
///
#[deriving(Eq, PartialEq, Clone, Show)]
pub enum BulletElement {
    Nested(BulletList),
    Lone(Bullet),
}

impl BulletElement {}

impl ToHtml for BulletElement {
    fn to_html(&self) -> Html {
        match self {
            &Nested(ref bullet_list) => bullet_list.to_html(),
            &Lone(ref bullet) => bullet.to_html(),
        }
    }
}

/// A list element.
///
/// *contents* is the contents of the list item.
/// *tag* determines the tag to use for the bullet item.
///
/// The only valid tag for normal HTML would be `li`.
///
#[deriving(Eq, PartialEq, Clone, Show)]
pub struct Bullet {
    contents: String,
    tag: String,
}

/// A markdown bullet.
///
/// For example:
///
/// - This is a bullet.
///
impl Bullet {
    pub fn new(contents: String) -> Bullet {
        Bullet {
            contents: contents,
            tag: "li".to_string(),
        }
    }
}

impl ToHtml for Bullet {
    fn to_html(&self) -> Html {
        let name: String = self.tag.clone();
        Html::new_simple(name, self.contents.clone())
    }
}

/// A markdown bulleted list.
///
/// For example:
///
/// - This is a bullet.
/// - But this second bullet makes it a bulleted list.
///     - They can have somewhat arbitrary depth.
///
#[deriving(Eq, PartialEq, Clone, Show)]
pub struct BulletList {
    contents: Vec<BulletElement>,
    tag: String,
}

impl BulletList {
    /// Create a new empty ordered list (`ol` is the tag).
    fn new_ordered() -> BulletList {
        BulletList {
            contents: vec![],
            tag: "ol".to_string(),
        }
    }

    /// Create a new empty unordered list (`ul` is the tag).
    fn new_unordered() -> BulletList {
        BulletList {
            contents: vec![],
            tag: "ul".to_string(),
        }
    }

    /// Add a `BulletElement` to this list.
    fn push(&mut self, elem: BulletElement) {
        self.contents.push(elem);
    }
}

impl ToHtml for BulletList {
    fn to_html(&self) -> Html {
        let name: String = "ul".to_string();
        let mut html = Html::new_empty(name);
        for bullet in self.contents.iter() {
            let tag = bullet.to_html();
            html.add_tag(tag);
        }
        html
    }
}

#[cfg(test)]
mod tests {
    use super::BulletElement;
    use super::Lone;
    use super::Nested;
    use super::Bullet;
    use super::BulletList;
    use html::ToHtml;

    #[test]
    fn test_bullet_fmt() {
        let bullet = Bullet::new("Hello, world".to_string());
        assert_eq!(format!("{}", bullet.to_html()),
                   "<li>Hello, world</li>".to_string());
    }

    #[test]
    fn test_nested_bullets() {
        let mut bullets = BulletList::new_unordered();
        bullets.push(Lone(Bullet::new("Hello".to_string())));
        bullets.push(Lone(Bullet::new("World".to_string())));
        assert_eq!(format!("{}", bullets.to_html()),
                   "<ul><li>Hello</li><li>World</li></ul>".to_string());
    }
}
