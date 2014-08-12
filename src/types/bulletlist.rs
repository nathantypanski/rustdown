use html::ToHtml;
use html::Html;

#[deriving(Eq, PartialEq, Clone)]
pub enum BulletType {
    Nested(BulletList),
    Lone(Bullet),
}

impl BulletType {}

impl ToHtml for BulletType {
    fn to_html(&self) -> Html {
        match self {
            &Nested(ref bullet_list) => bullet_list.to_html(),
            &Lone(ref bullet) => bullet.to_html(),
        }
    }
}

#[deriving(Eq, PartialEq, Clone)]
pub struct Bullet {
    contents: String,
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
        }
    }
}

impl ToHtml for Bullet {
    fn to_html(&self) -> Html {
        let name: String = "li".to_string();
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
#[deriving(Eq, PartialEq, Clone)]
pub struct BulletList {
    contents: Vec<BulletType>,
}

impl BulletList {
    fn new() -> BulletList {
        BulletList {
            contents: vec![],
        }
    }

    fn push(&mut self, elem: BulletType) {
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
    use super::BulletType;
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
        let mut bullets = BulletList::new();
        bullets.push(Lone(Bullet::new("Hello".to_string())));
        bullets.push(Lone(Bullet::new("World".to_string())));
        assert_eq!(format!("{}", bullets.to_html()),
                   "<ul><li>Hello</li><li>World</li></ul>".to_string());
    }
}
