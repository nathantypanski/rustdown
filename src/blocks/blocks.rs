use std::iter::FromIterator;
use std::string::String;
use std::slice::Items;

use super::Block;

#[deriving(Show)]
pub struct Blocks(Vec<Block>);

impl Blocks {
    pub fn len(&self) -> uint {
        match self { &Blocks(ref blocks) => blocks.len() }
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> Items<'a,Block> {
        match self { &Blocks(ref blocks) => blocks.as_slice().iter() }
    }
}

impl Index<uint, Block> for Blocks {
    fn index<'a>(&'a self, index: &uint) -> &'a Block {
        let block = match self { &Blocks(ref blocks) => blocks.get(*index) };
        block
    }
}


impl Vector<Block> for Blocks {
    #[inline]
    fn as_slice<'a>(&'a self) -> &'a [Block] {
        let blocks = match self { &Blocks(ref blocks) => blocks.as_slice() };
        blocks
    }
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
