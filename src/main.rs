use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};

use bevy_rapier2d::prelude::*;

pub mod component;
pub mod resource;
mod system;
mod world_gen;

use world_gen::system::{gen_world_from_level_data, generate_world};

use component::*;
use resource::*;
use system::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "logion".into(),
                        resolution: (1000., 600.).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            bevy_framepace::FramepacePlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<LevelData>()
        .add_systems(
            Startup,
            (
                frame_rate,
                init_rendering,
                generate_world,
                gen_world_from_level_data,
                spawn_ball,
            )
                .chain(),
        )
        .add_systems(Update, (move_camera, move_ball))
        .run();
}
fn frame_rate(mut rate: ResMut<FramepaceSettings>) {
    rate.limiter = Limiter::from_framerate(90.)
}
