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

fn paragraph(s: &str) -> String {
    "<p>".to_string() + s + "</p>".to_string()
}

#[cfg(test)]
mod tests {
    use super::{pound_heading, paragraph};

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

    #[test]
    fn test_making_paragraphs() {
        assert_eq!(paragraph("Hello, World"),
                   "<p>Hello, World</p>".to_string());
    }
}
