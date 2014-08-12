use html::ToHtml;
use html::Html;

#[deriving(Eq, PartialEq, Clone)]
pub struct Bullet {
    contents: String,
    depth: uint,
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
            depth: 0,
        }
    }

    pub fn new_depth(contents: String, depth: uint) -> Bullet {
        Bullet {
            contents: contents,
            depth: depth,
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
    contents: Vec<Bullet>,
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
    use super::Bullet;
    use html::ToHtml;

    #[test]
    fn test_bullet_fmt() {
        let bullet = Bullet::new("Hello, world".to_string());
        assert_eq!(format!("{}", bullet.to_html()),
                   "<li>Hello, world</li>".to_string());
    }
}
