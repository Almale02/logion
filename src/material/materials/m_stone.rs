use bevy::prelude::{Handle, Image, Vec2};
use image::{Rgb, RgbImage};

use crate::lib::Identifier::Identifier;
use crate::material::lib::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StoneMaterial {
    id: Identifier,
    color: Rgb<u8>,
}
impl Material for StoneMaterial {
    fn id(&self) -> &Identifier {
        &self.id
    }
    fn color(&self) -> Rgb<u8> {
        self.color
    }
    fn base_block(&self) -> Identifier {
        Identifier {
            id: "block:{stone}".into(),
        }
    }
    fn write_pixles(
        &self,
        image: RgbImage,
        pixel_usage: Vec<Vec2>,
        base_image: Handle<Image>,
        percent: u8,
    ) -> RgbImage {
        // TODO: implement write_pixles for stone material
        todo!()
    }
}
impl Default for StoneMaterial {
    fn default() -> Self {
        StoneMaterial {
            id: Identifier {
                id: "material:{stone}".to_string(),
            },
            color: Rgb([115; 3]),
        }
    }
}
