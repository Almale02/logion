use crate::resource::registry::sb_data_type_registry::SBDataTypeRegistry;

use crate::{
    ball::action_state::{resource::ActionStateData},
    lib::{input::mouse::cursor_to_global_pos, math::deg_rad::deg_to_rad},
    resource::level_data::LevelData,
    structure::spawn::spawn_structure,
};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

pub fn build_placement(
    mut commands: Commands,
    mut click_event: EventReader<MouseButtonInput>,
    mut action_state_data: ResMut<ActionStateData>,
    level_data: Res<LevelData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    data_type_reg: Res<SBDataTypeRegistry>,
    asset_server: Res<AssetServer>,
    // cursor position
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    // rotation
    keyboard: Res<Input<KeyCode>>,
    // delay click
    mut tick_timer: Local<u8>,
) {
    let build_data = action_state_data
        .build_placement_data
        .struct_build_data
        .clone();
    if keyboard.just_pressed(KeyCode::F) {
        action_state_data.build_placement_data.build_struct_rotation =
            match action_state_data.build_placement_data.build_struct_rotation {
                0 => 270,
                270 => 0,
                _ => unreachable!(),
            }
    }
    if action_state_data.build_placement_data.click_delay == true {
        if *tick_timer == 0 {
            *tick_timer = 35;
        } else {
            *tick_timer -= 1;
            if *tick_timer == 0 {
                action_state_data.build_placement_data.click_delay = false;
            }
        }
        click_event.clear();
    }
    for event in click_event.iter() {
        let mouse_pos = cursor_to_global_pos(&window, &camera);
        if event.state == ButtonState::Released
            && event.button == MouseButton::Left
            && mouse_pos.is_some()
        {
            let mouse_pos = mouse_pos.unwrap();
            spawn_structure(
                &data_type_reg,
                &mut commands,
                &level_data,
                &mut meshes,
                &mut materials,
                &asset_server,
                build_data.clone(),
                mouse_pos.with_rotation(Quat::from_rotation_z(deg_to_rad(
                    action_state_data.build_placement_data.build_struct_rotation,
                ))),
            );
        }
    }
}
pub fn spawn_build_ghost(
    mut commands: Commands,
    mut action_state_data: ResMut<ActionStateData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut structure = action_state_data
        .build_placement_data
        .struct_build_data
        .structure
        .clone();
    structure.change_transparency(0.4);

    let id = commands
        .spawn((
            TransformBundle::from_transform(Transform::default()),
            VisibilityBundle::default(),
            BuildGhost,
        ))
        .with_children(|spawn| {
            for (pos, sprite) in structure.sprites.clone() {
                spawn.spawn(SpriteBundle {
                    sprite,
                    transform: Transform::from_translation(pos),
                    ..default()
                });
            }
            for (pos, mesh, material) in structure.meshes.clone() {
                spawn.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(mesh.clone()).into(),
                    material: materials.add(material.clone()),
                    transform: Transform::from_translation(pos),
                    ..default()
                });
            }
        })
        .id();
    action_state_data.build_placement_data.build_ghost_id = id;
}
pub fn delete_build_ghost(mut commands: Commands, mut action_state_data: ResMut<ActionStateData>) {
    commands
        .entity(action_state_data.build_placement_data.build_ghost_id)
        .despawn_recursive();
    action_state_data.build_placement_data.build_ghost_id = Entity::PLACEHOLDER;
}

pub fn move_build_ghost(
    mut q_transfomr: Query<&mut Transform, With<BuildGhost>>,
    // cursor to global pos
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    //
    action_state_data: Res<ActionStateData>,
) {
    if let Ok(mut transform) =
        q_transfomr.get_mut(action_state_data.build_placement_data.build_ghost_id)
    {
        if let Some(pos) = cursor_to_global_pos(&window, &camera) {
            transform.translation = pos.translation;
            transform.rotation = Quat::from_rotation_z(deg_to_rad(
                action_state_data.build_placement_data.build_struct_rotation,
            ));
        }
    }
}
#[derive(Component)]
pub struct BuildGhost;
/*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
* */
