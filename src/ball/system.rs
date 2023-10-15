use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::{
    ball::component::*, resource::level_data::LevelData,
    structure::behaivour::logic::lib::sb_script::SBScript,
};
pub fn move_ball(
    mut impulse: Query<&mut ExternalImpulse, With<Ball>>,
    mut scripts: Query<&mut SBScript>,
    input: Res<Input<KeyCode>>,
    mut vel: Query<&mut Velocity, With<Ball>>,
    mut _transform: Query<&mut Transform, With<Ball>>,
    //
    scan: Res<Input<ScanCode>>,
    mut evr_char: EventReader<ReceivedCharacter>,
) {
    let horizontal_speed = 40.;
    if input.pressed(KeyCode::D) {
        impulse.single_mut().impulse = Vec2::X * horizontal_speed
    }
    if input.pressed(KeyCode::A) {
        impulse.single_mut().impulse = Vec2::X * -horizontal_speed
    }
    if input.just_pressed(KeyCode::Space) {
        for mut script in scripts.iter_mut() {
            script.add_to_queue(69)
        }
        impulse.single_mut().impulse = Vec2::Y * 2000.
    }
    let mut _vel = vel.single_mut();

    for x in scan.get_just_pressed() {
        dbg!(x.0);
        if scan.pressed(ScanCode(5)) {}
    }
    for ev in evr_char.iter() {
        dbg!(ev.char);
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_data: Res<LevelData>,
) {
    let _text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
    };
    commands.spawn((
        Ball,
        SpriteBundle {
            texture: asset_server.load("image/player.png"),
            transform: Transform::from_scale(Vec3 {
                x: 2.15,
                y: 2.15,
                z: 1.,
            })
            .with_translation(Vec3 {
                x: 0.,
                y: (level_data.terrain_height[0] as f32 + 5.) * level_data.grid_unit as f32,
                z: -1.,
            }),
            ..default()
        },
        Damping {
            linear_damping: 0.5,
            angular_damping: 0.,
        },
        Friction {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Min,
        },
        GravityScale(2.5),
        Collider::ball(16. / 2.15),
        //
        ColliderMassProperties::Density(1.2),
        RigidBody::Dynamic,
        Velocity::default(),
        ExternalForce::default(),
        ExternalImpulse::default(),
    ));
}
