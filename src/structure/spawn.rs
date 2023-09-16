use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{Collider, ColliderMassProperties, RigidBody};

use crate::resource::level_data::LevelData;

use super::lib::structure::Structure;
pub fn spawn_structure(
    mut commands: &mut Commands,
    level_data: &Res<LevelData>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    structure: Structure,
    position: Transform,
) {
    commands
        .spawn((
            TransformBundle::from_transform(position),
            VisibilityBundle::default(),
            RigidBody::Dynamic,
            Collider::compound(structure.colliders.clone()),
            ColliderMassProperties::Density(1.5),
        ))
        .with_children(|children| {
            for (pos, sprite) in structure.sprites.clone() {
                children.spawn(SpriteBundle {
                    sprite,
                    transform: Transform::from_translation(pos),
                    ..default()
                });
            }
            for (pos, mesh, mut material) in structure.meshes.clone() {
                children.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(mesh.clone()).into(),
                    material: materials.add(material.clone()),
                    transform: Transform::from_translation(pos),
                    ..default()
                });
            }
        });
}
