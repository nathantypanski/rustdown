
fn paragraph(s: &str) -> String {
    "<p>".to_string() + s + "</p>".to_string()
}

#[cfg(test)]
mod tests {
    use super::{paragraph};

    #[test]
    fn test_making_paragraphs() {
        assert_eq!(paragraph("Hello, World"),
                   "<p>Hello, World</p>".to_string());
    }
}
