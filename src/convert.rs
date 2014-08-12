use blocks::Blocks;
use blocks::Block;
use types::Heading;
use types::MarkdownStructure;
use types::MDH;
use types::MDP;
use types::parse_heading;
use types::parse_paragraph;

pub fn parse_block(block: &Block) -> MarkdownStructure {
    match parse_heading(block) {
        Some(heading) => return MDH(heading),
        None => {}
    }
    MDP(parse_paragraph(block))
}

pub fn convert(blocks: Blocks) -> Vec<MarkdownStructure> {
    let mut markdown: Vec<MarkdownStructure> = vec![];
    for block in blocks.iter() {
        markdown.push(parse_block(block))
    }
    markdown
}
