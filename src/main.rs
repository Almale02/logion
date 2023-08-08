use std::default;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
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

use resource::{game_assets::GameAssets, level_data::LevelData};
use system::*;
use world_gen::system::*;
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
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        //.init_resource::<GameAssets>()
        .init_resource::<LevelData>()
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Game),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::AssetLoading)
        .add_systems(
            OnEnter(GameState::Game),
            (
                frame_rate,
                init_rendering,
                generate_world,
                generate_grass,
                materialize,
                load_world,
                spawn_ball,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (move_ball, move_camera).run_if(in_state(GameState::Game)),
        )
        .run();
}
fn frame_rate(mut rate: ResMut<FramepaceSettings>) {
    rate.limiter = Limiter::from_framerate(90.);
    let _a = 3;
}
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Game,
}
