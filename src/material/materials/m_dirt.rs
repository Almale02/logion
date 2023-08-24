use bevy::prelude::Vec2;
use rand::Rng;

use crate::lib::Identifier::Identifier;
use crate::material::lib::*;
use image::{Rgba, RgbaImage};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DirtMaterial {
    id: Identifier,
}
impl Material for DirtMaterial {
    fn id(&self) -> &Identifier {
        &self.id
    }
    fn base_block(&self) -> Identifier {
        Identifier {
            id: "block:{dirt}".into(),
        }
    }
    fn write_pixles(&self, image: &mut RgbaImage, _pixel_usage: &mut [[bool; 16]; 16], count: u8) {
        let mut pixels_put: Vec<Vec2> = Vec::default();
        for y in 0..16 {
            for x in 0..16 {
                pixels_put.push(Vec2 {
                    x: x as f32,
                    y: y as f32,
                });
            }
        }
        for _x in 0..(256 - count as u16) {
            pixels_put.remove(rand::thread_rng().gen_range(0..pixels_put.len()));
        }

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            if pixels_put.contains(&Vec2 {
                x: x as f32,
                y: y as f32,
            }) {
                *pixel = Rgba([141, 111, 86, 255])
            }
        }
    }
}
impl Default for DirtMaterial {
    fn default() -> Self {
        DirtMaterial {
            id: Identifier {
                id: "material:{dirt}".to_string(),
            },
        }
    }
}
