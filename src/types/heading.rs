pub struct Heading {
    contents: String,
    depth: uint,
}

impl Heading {
    pub fn new(title: String, depth: uint) -> Heading {
        Heading {
            contents: title,
            depth: uint,
        }
    }
}

fn pound_heading(s: &str) -> Option<String> {
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
        let head = format!("<h{}>{}</h{}>", count, title, count);
        result = Some(head);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::{pound_heading};

    #[test]
    fn test_pound_heads() {
        assert_eq!(pound_heading("# Hello, world"),
                   Some("<h1>Hello, world</h1>".to_string()));
        assert_eq!(pound_heading("## Hello, world"),
                   Some("<h2>Hello, world</h2>".to_string()));
    }

    #[test]
    fn test_non_pound_heads() {
        assert_eq!(pound_heading("Hello, world"), None);
        assert_eq!(pound_heading(" ## Hello, world"), None);
    }
}
