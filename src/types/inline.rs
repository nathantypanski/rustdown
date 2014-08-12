
enum Contents {
    Italic(String, Box<Contents>),
    Bold(String, Box<Contents>),
    Code(String, Box<Contents>),
    Normal(String),
    Nil
}

struct InlineContents {
    contents: Vec<String>,
}
