// 1/10 Cl 3.55
// 1/4 N 3.5
// 3 H 3
// .5 O 8

#[macro_use]
extern crate mopa;

use std::any::{self, Any, TypeId};

use ball::plugin::Ball;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::label::DynEq;
use bevy_asset_loader::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_rapier2d::prelude::*;

mod ball;
mod block;
mod lib;
mod material;
mod resource;
mod structure;
mod ui;
mod world_gen;
mod world_load;

use resource::registry::structures::StructureRegistry;
use resource::test_structure::TestStructure;
use resource::{game_assets::GameAssets, level_data::LevelData};
use structure::lib::structure::Structure;
use structure::predefined::general::PreStructGeneral;
use world_gen::plugin::WorldGen;

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.))
        .init_resource::<LevelData>()
        .init_resource::<TestStructure>()
        .init_resource::<StructureRegistry>()
        //
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Game),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::AssetLoading)
        // SECTION: game plugins
        .add_plugins(WorldGen)
        .add_plugins(Ball)
        // SECTION: end
        .add_systems(Update, (frame_rate).run_if(in_state(GameState::Game)))
        .add_systems(Startup, ui)
        .run();
}
fn frame_rate(mut commands: Commands, mut rate: ResMut<FramepaceSettings>) {
    rate.limiter = Limiter::from_framerate(75.);
    //dbg!(time.delta_seconds() * 1000.);
}
fn ui(mut commands: Commands, asset_server: Res<AssetServer>) {}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Game,
}
pub fn init_rendering(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3 {
            x: (0.),
            y: (360.),
            z: (100.),
        }),
        ..default()
    });
}
pub fn init_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0., -16. * 30.);
}
