pub use self::block::Block;
pub use self::blocks::Blocks;
pub use self::readers::blockify_file;
pub use self::readers::read_to_blocks;

mod block;
mod blocks;
mod readers;

pub trait FromBlock {
    fn from_block(block: Block) -> Option<Self>;
}
