use crate::block::blocks::{air::AirBlock, dirt::DirtBlock, stone::StoneBlock};
use crate::block::lib::Block;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum BlockType {
    Air(AirBlock),
    Dirt(DirtBlock),
    Stone(StoneBlock),
}
pub trait BlockConvertible {
    fn as_block(&self) -> &dyn Block;
    fn as_mut_block(&mut self) -> &mut dyn Block;
}
impl BlockConvertible for BlockType {
    fn as_block(&self) -> &dyn Block {
        match self {
            BlockType::Air(x) => x,
            BlockType::Dirt(x) => x,
            BlockType::Stone(x) => x,
            // Implement other variants as needed
        }
    }
    fn as_mut_block(&mut self) -> &mut dyn Block {
        match self {
            BlockType::Air(x) => x,
            BlockType::Dirt(x) => x,
            BlockType::Stone(x) => x,
            // Implement other variants as needed
        }
    }
}
