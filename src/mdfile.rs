use std::io::{IoResult, Open, Read};
use std::io::fs::File;
use std::path::posix::Path;

pub fn open_markdown_file(path: &Path) -> IoResult<File> {
    File::open_mode(path, Open, Read)
}
