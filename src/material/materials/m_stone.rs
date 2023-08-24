use rand::Rng;

use bevy::prelude::Vec2;
use bevy_rapier2d::na::Vector2;
use image::buffer::EnumeratePixels;
use image::{PixelWithColorType, Rgb, Rgba, RgbaImage};

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
    fn base_block(&self) -> Identifier {
        Identifier {
            id: "block:{stone}".into(),
        }
    }
    fn write_pixles(&self, image: &mut RgbaImage, pixel_usage: &mut [[bool; 16]; 16], count: u8) {
        let mut pixels_put: Vec<Vec2> = Vec::default();
        for y in 0..16 {
            for x in 0..16 {
                pixels_put.push(Vec2 {
                    x: x as f32,
                    y: y as f32,
                });
            }
        }

        for x in 0..(256 - count as u16) {
            pixels_put.remove(rand::thread_rng().gen_range(0..pixels_put.len()));
        }

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            if pixels_put.contains(&Vec2 {
                x: x as f32,
                y: y as f32,
            }) {
                *pixel = Rgba([115, 115, 115, 255])
            }
        }
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
