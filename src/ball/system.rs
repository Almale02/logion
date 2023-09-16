use bevy::input::mouse::*;
use bevy::prelude::*;

use bevy_rapier2d::{
    na::Vector1,
    prelude::*,
    rapier::prelude::{ImpulseJointSet, Vector},
};

use crate::{
    ball::component::*,
    lib::math::{deg_rad::deg_to_rad, vec1::Vec1},
    resource::level_data::LevelData,
};

pub fn move_ball(
    mut q_movement: Query<
        (
            &mut Vec1,
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &mut Velocity,
        ),
        With<Ball>,
    >,
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera2d>>,
        Query<&mut Transform, With<Ball>>,
    )>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut should_jump_on_fall: Local<bool>,
) {
    let move_speed = 3.5;
    let fall_speed = 5.5;

    let e = Entity::PLACEHOLDER;

    for (mut y_vel, mut controller, controller_info, mut velocity) in q_movement.iter_mut() {
        controller.slide = true;
        controller.min_slope_slide_angle = deg_to_rad(69);
        controller.max_slope_climb_angle = deg_to_rad(55);
        controller.offset = CharacterLength::Relative(0.05);

        let mut x_vel = 0.;
        {
            if keyboard.pressed(KeyCode::Right) {
                x_vel = move_speed * time.delta_seconds() * 100.
            }
            if keyboard.pressed(KeyCode::Left) {
                x_vel = -move_speed * time.delta_seconds() * 100.
            }
        }
        {
            if keyboard.just_pressed(KeyCode::Space) {
                if controller_info.grounded {
                    **y_vel += 18. * (fall_speed / 2.5)
                } else {
                    *should_jump_on_fall = true;
                }
            }
            if controller_info.grounded && *should_jump_on_fall {
                **y_vel += 18. * (fall_speed / 2.5);
                *should_jump_on_fall = false;
            }
        }

        // INFO: gravity
        **y_vel -= fall_speed * time.delta_seconds() * 50.;
        // INFO: ceil gravity
        if **y_vel < -fall_speed {
            **y_vel = -fall_speed
        }

        controller.translation = Some(Vect {
            x: x_vel,
            y: **y_vel,
        });

        set.p1().single_mut().rotate_z(x_vel);
        velocity.angvel = x_vel * 100.;
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_data: Res<LevelData>,
) {
    commands.spawn((
        Ball,
        SpriteBundle {
            texture: asset_server.load("image/player.png"),
            transform: Transform::from_scale(Vec3 {
                x: 2.15,
                y: 2.15,
                z: 0.,
            })
            .with_translation(Vec3 {
                x: 0.,
                y: (level_data.terrain_height[0] as f32 + 5.) * level_data.grid_unit as f32,
                z: 0.,
            }),
            ..default()
        },
        //Collider::ball(16. / 3. + 3.27),
        Collider::ball(16. / 2.15),
        ColliderMassProperties::Density(1.2),
        RigidBody::KinematicVelocityBased,
        KinematicCharacterController::default(),
        KinematicCharacterControllerOutput::default(),
        Velocity::default(),
        Vec1::default(),
    ));
}
