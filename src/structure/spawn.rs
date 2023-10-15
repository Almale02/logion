use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, ExternalForce, ExternalImpulse, RigidBody, Velocity,
};

use crate::{
    resource::{level_data::LevelData, registry::sb_data_type_registry::SBDataTypeRegistry},
};

use super::{
    behaivour::{
        hardware::hardwares::text_display::SBHTextDisplay,
    },
    lib::{structure_spawn_data::StructureSpawnData},
};
pub fn spawn_structure(
    _data_type_reg: &SBDataTypeRegistry,
    commands: &mut Commands,
    _level_data: &Res<LevelData>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    build_data: StructureSpawnData,
    position: Transform,
) {
    let structure = build_data.structure;
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
    };
    commands
        .spawn((
            TransformBundle::from_transform(position),
            VisibilityBundle::default(),
            RigidBody::Dynamic,
            Collider::compound(structure.colliders.clone()),
            ColliderMassProperties::Density(2.),
            structure.structure_behaviour,
            Velocity::default(),
            Spawned,
            build_data.script,
            //
            SBHTextDisplay {
                text: "3443434".into(),
            },
        ))
        .insert(ExternalForce {
            force: Vec2::new(0., 0.),
            torque: 0.,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0., 0.),
            torque_impulse: 0.,
        })
        .with_children(|children| {
            for (pos, sprite) in structure.sprites.clone() {
                children.spawn(SpriteBundle {
                    sprite,
                    transform: Transform::from_translation(pos),
                    ..default()
                });
            }
            for (pos, mesh, material) in structure.meshes.clone() {
                children.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(mesh.clone()).into(),
                    material: materials.add(material.clone()),
                    transform: Transform::from_translation(pos),
                    ..default()
                });
            }
            children.spawn((
                Text2dBundle {
                    text: Text::from_section("111", text_style),
                    transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::new(1., 1., 1.)),
                    ..default()
                },
                ZIndex::Global(10),
            ));
        });
}
#[derive(Component)]
pub struct Spawned;
