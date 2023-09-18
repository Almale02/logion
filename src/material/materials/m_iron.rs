use rand::Rng;

use rand::seq::SliceRandom;

use bevy::prelude::Vec2;

use image::{Rgb, Rgba, RgbaImage};

use crate::lib::identifier::Identifier;
use crate::lib::math::usvec2::USVec2;
use crate::material::lib::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct IronMaterial {
    id: Identifier,
    color: Rgb<u8>,
}
impl Material for IronMaterial {
    fn id(&self) -> &Identifier {
        &self.id
    }
    fn base_block(&self) -> Identifier {
        Identifier { id: "none".into() }
    }
    fn write_pixles(&self, image: &mut RgbaImage, pixel_usage: &mut [[bool; 16]; 16], count: u8) {
        let mut rem = count;
        loop {
            let base_x = rand::thread_rng().gen_range(0..15);
            let base_y = rand::thread_rng().gen_range(0..15);

            let mut free_pixels: Vec<USVec2> = Vec::default();
            for y1 in base_y..=base_y + 3 {
                for x1 in base_x..base_x + 3 {
                    if let Some(val) = get_coordinate(*pixel_usage, x1, y1) {
                        free_pixels.push(USVec2::new(x1, y1))
                    }
                }
            }
            if free_pixels.len() > 1 {
                let mut count = 0;
                if free_pixels.len() as u8 > rem {
                    count = rem;
                    rem = 0;
                } else {
                    rem -= free_pixels.len() as u8;
                    count = free_pixels.len() as u8;
                }

                free_pixels.shuffle(&mut rand::thread_rng());
                for pixel_pos in &free_pixels[0..count as usize] {
                    pixel_usage[pixel_pos.y][pixel_pos.x] = true;
                    image.put_pixel(
                        pixel_pos.x as u32,
                        pixel_pos.y as u32,
                        //Rgba([255, 255, 255, 255]),
                        Rgba([130, 130, 130, 255]),
                    )
                }
            }
            if rem == 0 {
                break;
            }
        }
    }
}
impl Default for IronMaterial {
    fn default() -> Self {
        IronMaterial {
            id: Identifier::new(Identifier::MATERIAL, "game:iron"),
            color: Rgb([115; 3]),
        }
    }
}

fn get_coordinate(free_pixels: [[bool; 16]; 16], x: usize, y: usize) -> Option<bool> {
    if let Some(y1) = free_pixels.get(y) {
        if let Some(val) = y1.get(x) {
            Some(val.clone())
        } else {
            None
        }
    } else {
        None
    }
}
