pub use self::heading::Heading;

mod heading;
mod paragraph;

enum MarkdownStructure {
    Heading,
    Paragraph,
}
