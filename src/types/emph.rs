use std::char::Char;

struct Emph {
    letter: char,
    content: Vec<String>,
}

impl Emph {
    fn new(letter: char) -> Emph {
        Emph {
            letter: letter,
            content: vec![],
        }
    }
}
