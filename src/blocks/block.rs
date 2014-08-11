use std::ops::Index;
use std::slice::Items;

#[deriving(Show)]
pub struct Block {
    contents: Vec<String>,
}

impl Block {
    pub fn new(s: Vec<String>) -> Block {
        Block {
            contents: s,
        }
    }

    pub fn new_oneline(s: String) -> Block {
        Block {
            contents: vec![s],
        }
    }

    pub fn len(&self) -> uint {
        self.contents.len()
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> Items<'a,String> {
        self.as_slice().iter()
    }
}

impl Index<uint, String> for Block {
    fn index<'a>(&'a self, index: &uint) -> &'a String {
        self.contents.get(*index)
    }
}


impl Vector<String> for Block {
    #[inline]
    fn as_slice<'a>(&'a self) -> &'a [String] {
        self.contents.as_slice()
    }
}
