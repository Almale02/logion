

use bevy::{prelude::*, sprite::Sprite};
use bevy_rapier2d::prelude::Collider;

use crate::{
    lib::identifier::Identifier,
};

use super::{structure_behaviour::StructureBehaviour, structure_helpers::StructTextureHelper};

#[derive(Debug, Clone, Component)]
pub struct Structure {
    pub template_id: Option<Identifier>,
    pub colliders: Vec<(Vec2, f32, Collider)>,
    pub sprites: Vec<(Vec3, Sprite)>,
    pub meshes: Vec<(Vec3, Mesh, ColorMaterial)>,
    pub structure_behaviour: StructureBehaviour,
}
impl Structure {
    pub fn new(
        template_id: Option<Identifier>,
        colliders: Vec<(Vec2, f32, Collider)>,
        sprites: Vec<(Vec3, Sprite)>,
        meshes: Vec<(Vec3, Mesh, ColorMaterial)>,
        behaviour: StructureBehaviour,
    ) -> Self {
        Self {
            template_id,
            colliders,
            sprites,
            meshes,
            structure_behaviour: behaviour,
        }
    }
    pub fn change_transparency(self: &mut Self, alpha: f32) {
        for mesh in self.meshes.iter_mut() {
            mesh.2.color.set_a(alpha);
        }
        for sprite in self.sprites.iter_mut() {
            sprite.1.color.set_a(alpha);
        }
    }
}
impl Default for Structure {
    fn default() -> Self {
        Self::new(
            Some(Identifier::new(Identifier::STRUCTURE, "default")),
            vec![(
                Vec2::new(0., 0.),
                0.,
                Collider::triangle(Vec2::new(0., 0.), Vec2::new(32., 0.), Vec2::new(32., 32.)),
            )],
            Vec::default(),
            StructTextureHelper::gen_meshes_from_pos(
                //shape::Circle::new(16.).into(),
                StructTextureHelper::gen_grid_right_triangle_mesh(Color::TEAL),
                ColorMaterial::from(Color::TEAL),
                vec![Vec3::new(0., 0., 0.)],
            ),
            StructureBehaviour::default(),
        )
    }
}
impl PartialEq for Structure {
    fn eq(&self, other: &Self) -> bool {
        if let Some(self_template) = &self.template_id {
            if let Some(other_template) = &other.template_id {
                return self_template == other_template;
            }
        }
        return false;
    }
}
impl Eq for Structure {}
