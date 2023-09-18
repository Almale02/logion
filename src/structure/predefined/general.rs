use crate::structure::lib::structure_behaviour::StructureBehaviour;

use crate::{
    lib::identifier::Identifier,
    structure::lib::{structure::Structure, structure_helpers::StructTextureHelper},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PreStructGeneral {}
impl PreStructGeneral {
    pub fn triangle() -> Structure {
        Structure::new(
            Some(Identifier::new(Identifier::STRUCTURE, "game:triangle")),
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
    pub fn ball() -> Structure {
        Structure::new(
            Some(Identifier::new(Identifier::STRUCTURE, "game:ball")),
            vec![(Vec2::new(0., 0.), 0., Collider::ball(16.))],
            Vec::default(),
            vec![(
                Vec3::new(0., 0., 0.),
                shape::Circle::new(16.).into(),
                ColorMaterial::from(Color::TEAL),
            )],
            StructureBehaviour::default(),
        )
    }
    pub fn square() -> Structure {
        Structure::new(
            Some(Identifier::new(Identifier::STRUCTURE, "game:square")),
            vec![(Vec2::ZERO, 0., Collider::cuboid(16., 16.))],
            vec![(
                Vec3::ZERO,
                Sprite {
                    color: Color::rgb_u8(30, 30, 30),
                    custom_size: Some(Vec2::new(32., 32.)),
                    ..default()
                },
            )],
            Vec::default(),
            StructureBehaviour::default(),
        )
    }
}
