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
}

impl<PartialEq> Block {
    fn contains(&self, x: &String) -> bool {
        self.contents.contains(x)
    }

    fn dedup(&mut self) {
        self.contents.dedup()
    }
}
