use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_rapier2d::prelude::*;

use image::{Rgb, RgbImage};

// SECTION: root_file
mod component;
mod system;

// SECTION: FOLDER
mod ball;
mod block;
mod lib;
mod material;
mod resource;
mod world_gen;
mod world_load;

use ball::system::*;
use block::blocks::{dirt::DirtBlock, stone::StoneBlock};
use resource::level_data::LevelData;
use system::*;
use world_gen::system::*;
use world_load::system::load_world;

// TODO: asdaasd LEVES
// WARN: leveses TÁL
// INFO: lila
// SECTION: reddy baddy lali
// INFO: https://www.reddit.com/r/bevy/comments/y5fzkl/how_to_get_a_texture_from_a_in_memory_image_not_a/

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
        .add_systems(
            Startup,
            (
                frame_rate,
                init_rendering,
                generate_world,
                generate_grass,
                load_world,
                spawn_ball,
            )
                .chain(),
        )
        .add_systems(Update, (move_ball, move_camera))
        .run();
}
fn frame_rate(mut rate: ResMut<FramepaceSettings>) {
    rate.limiter = Limiter::from_framerate(90.)
}
fn gen_image() {
    let mut image = RgbImage::new(16, 16);

    for y in 0..16 {
        for x in 0..16 {
            image.put_pixel(x, y, Rgb([22, 55, 156]))
        }
    }
}
