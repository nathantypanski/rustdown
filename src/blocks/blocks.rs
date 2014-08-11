use std::iter::FromIterator;
use std::string::String;

use super::Block;

#[deriving(Show)]
pub struct Blocks(Vec<Block>);

impl Blocks {

}

impl FromIterator<String> for Blocks {
    fn from_iter<T: Iterator<String>>(mut iterator: T) -> Blocks {
        let mut blockbuf: Vec<String> = vec![];
        let mut blocks = iterator.fold(vec![], |mut vec, line| {
            if is_block_separator(line.as_slice()) {
                vec.push(Block::new(blockbuf.clone()));
                blockbuf = vec![];
            }
            else {
                blockbuf.push(line.as_slice().trim_right_chars('\n').to_string());
            }
            vec
        });
        blocks.push(Block::new(blockbuf));
        Blocks(blocks)
    }
}

pub fn is_block_separator(s: &str) -> bool {
    let s = s.trim_right_chars('\n').trim_left_chars(' ').trim_left_chars('\t');
    s == ""
}

#[cfg(test)]
mod tests {
    use super::{is_block_separator};

    #[test]
    fn test_determining_block_separators() {
        assert!(is_block_separator(""));
        assert!(is_block_separator("    "));
        assert!(is_block_separator("\t"));
        assert!(!is_block_separator(" ## t"));
    }
}
