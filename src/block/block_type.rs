use crate::block::lib::Block;
use crate::block::blocks::{
    air::AirBlock,
    dirt::DirtBlock,
    stone::StoneBlock
};

#[derive(Clone)]
pub enum BlockType {
    Air(AirBlock),
    Dirt(DirtBlock),
    Stone(StoneBlock)
}