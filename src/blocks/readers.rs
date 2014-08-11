use std::io::fs::File;
use std::io::BufferedReader;

use super::Block;

pub fn blockify_file(file: File) -> Vec<Block> {
    let mut reader = BufferedReader::new(file);
    read_to_blocks(reader)
}

pub fn read_to_blocks<R: Reader>(mut reader: BufferedReader<R>) -> Vec<Block> {
    let mut blockbuf: Vec<String> = vec![];
    let mut blocks = reader.lines().fold(vec![], |mut vec, line| {
        match line {
            Ok(line) => {
                if is_block_separator(line.as_slice()) {
                    vec.push(Block::new(blockbuf.clone()));
                    blockbuf = vec![];
                }
                else {
                    blockbuf.push(line.as_slice().trim_right_chars('\n').to_string());
                }
            },
            Err(_) => {
                // TODO: Handle.
                fail!("Error reading input file");
            },
        }
        vec
    });
    blocks.push(Block::new(blockbuf));
    blocks
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
