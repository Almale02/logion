use bevy::prelude::{Handle, Image, Vec2};
use image::{Rgb, RgbImage};

use crate::lib::Identifier::Identifier;
use crate::material::materials::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum MaterialType {
    Dirt(m_dirt::DirtMaterial),
    Stone(m_stone::StoneMaterial),
}
impl MaterialType {
    fn as_mateiral(&self) -> &dyn Material {
        match self {
            MaterialType::Dirt(x) => x,
            MaterialType::Stone(x) => x,
        }
    }
}
pub trait Material {
    fn id(&self) -> &Identifier;
    fn base_block(&self) -> Identifier;
    fn write_pixles(
        &self,
        image: RgbImage,
        pixel_usage: Vec<Vec<bool>>,
        base_image: Handle<Image>,
        percent: u8,
    ) -> RgbImage;
}
