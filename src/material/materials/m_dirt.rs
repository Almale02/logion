use bevy::prelude::{Handle, Image, Vec2};
use image::{Rgb, RgbImage};

use crate::lib::Identifier::Identifier;
use crate::material::lib::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DirtMaterial {
    id: Identifier,
    color: Rgb<u8>,
}
impl Material for DirtMaterial {
    fn id(&self) -> &Identifier {
        &self.id
    }
    fn color(&self) -> Rgb<u8> {
        self.color
    }
    fn base_block(&self) -> Identifier {
        Identifier {
            id: "block:{dirt}".into(),
        }
    }
    fn write_pixles(
        &self,
        image: RgbImage,
        pixel_usage: Vec<Vec2>,
        base_image: Handle<Image>,
        percent: u8,
    ) -> RgbImage {
        // TODO: implement write_pixles for dirt material
        todo!()
    }
}
impl Default for DirtMaterial {
    fn default() -> Self {
        DirtMaterial {
            id: Identifier {
                id: "material:{dirt}".to_string(),
            },
            color: Rgb([130, 91, 60]),
        }
    }
}
