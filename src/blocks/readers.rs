use std::io::fs::File;
use std::io::BufferedReader;
use std::iter::FromIterator;

use super::Block;
use super::blocks::Blocks;

pub fn blockify_file(file: File) -> Blocks {
    let mut reader = BufferedReader::new(file);
    read_to_blocks(reader)
}

pub fn read_to_blocks<R: Reader>(mut reader: BufferedReader<R>) -> Blocks {
    FromIterator::from_iter(reader.lines().map(|e| e.unwrap()))
}
