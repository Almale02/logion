use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};

use bevy_rapier2d::prelude::*;

// SECTION: root_file
mod component;
mod system;

// SECTION: FOLDER
mod ball;
mod block;
mod lib;
mod resource;
mod world_gen;
mod world_load;

use ball::system::*;
use resource::{block_texture::BlockTexture, level_data::LevelData};
use system::*;
use world_gen::system::generate_world;
use world_load::system::load_world;

// TODO: asdaasd LEVES
// WARN: leveses TÁL
// INFO: lila
// SECTION: reddy baddy lali

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
        //.add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<LevelData>()
        .init_resource::<BlockTexture>()
        .add_systems(
            Startup,
            (
                frame_rate,
                init_rendering,
                generate_world,
                load_world,
                spawn_ball,
            )
                .chain(),
        )
        .add_systems(Update, (move_ball, move_camera).chain())
        .run();
}
fn frame_rate(mut rate: ResMut<FramepaceSettings>) {
    rate.limiter = Limiter::from_framerate(90.)
}
