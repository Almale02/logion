use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use bevy::prelude::{Handle, Image, Vec2};
use image::{Rgb, RgbaImage};

use crate::lib::Identifier::Identifier;
use crate::material::materials::*;

// SECTION: MATERIAL_TYPE
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MaterialType {
    Dirt(m_dirt::DirtMaterial),
    Stone(m_stone::StoneMaterial),
}
impl MaterialType {
    pub fn as_mateiral(&self) -> &dyn Material {
        match self {
            MaterialType::Dirt(x) => x,
            MaterialType::Stone(x) => x,
        }
    }
    pub fn as_material_mut(&mut self) -> &mut dyn Material {
        match self {
            MaterialType::Dirt(x) => x,
            MaterialType::Stone(x) => x,
        }
    }
}
// SECTION: MATERIAL
pub trait Material {
    fn id(&self) -> &Identifier;
    fn base_block(&self) -> Identifier;
    fn write_pixles(&self, image: &mut RgbaImage, pixel_usage: &mut [[bool; 16]; 16], count: u8);
}
// SECTION: MATERIAL_GEN_LIST
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MaterialGenList {
    material_count: u8,
    list: HashMap<MaterialType, u8>,
}

impl MaterialGenList {
    pub fn list(&self) -> HashMap<MaterialType, u8> {
        self.list.clone()
    }

    // INFO: returns the count of materials which are not baes materials
    pub fn material_count(&self) -> u8 {
        self.material_count
    }
    pub fn add_material(&mut self, material: MaterialType, count: u8, is_base: bool) {
        if !is_base {
            if self.material_count + count > 50 {
                // TODO: make it work with errors insead of panics!
                panic!("too much mateiral has been added")
            }
        }
        self.list.insert(material, count);
    }
}
impl Hash for MaterialGenList {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // You need to define a way to hash the fields of B
        // Here's an example implementation:
        for (key, value) in &self.list {
            key.hash(state);
            value.hash(state);
        }
    }
}
