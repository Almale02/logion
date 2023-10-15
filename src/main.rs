// 1/10 Cl 3.55
// 1/4 N 3.5
// 3 H 3
// .5 O 8

#[macro_use]
extern crate mopa;

use ball::movement::system::CameraFollowState;
use ball::plugin::BallPlugin;

use bevy::prelude::*;

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

use lib::nesting_tree::nesting_tree::NestingTree;

use resource::nestable_menu::NestableMenuHolder;
use resource::registry::sb_data_type_registry::SBDataTypeRegistry;
use resource::registry::sb_function_registry::SBFunctionRegistry;
use resource::registry::structures::StructureRegistry;
use resource::{game_assets::GameAssets, level_data::LevelData};

use structure::plugin::StructurePlugin;
use ui::nestable_menu::builtin_components::nest_id_text::NMNestIdText;

use ui::nestable_menu::builtin_components::update_text_components::update_text_components;
use ui::nestable_menu::lib::nestable_menu::{NMItem, NMItemType, NestableMenu};
use ui::nestable_menu::update_menu::update_nestable_menu;
use world_gen::plugin::WorldGenPlugin;

use crate::lib::nesting_tree::nesting_tree::NTPath;

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
        .init_resource::<StructureRegistry>()
        .init_resource::<SBDataTypeRegistry>()
        .init_resource::<SBFunctionRegistry>()
        .insert_resource(ClearColor(Color::rgb_u8(30, 40, 150)))
        //
        .add_state::<GameState>()
        .add_state::<CameraFollowState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Game),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::AssetLoading)
        // SECTION: game plugins
        .add_plugins(WorldGenPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(StructurePlugin)
        // SECTION: end
        .add_systems(
            Update,
            (frame_rate, update_nestable_menu, update_text_components)
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(Startup, (insert_non_send, ui).chain())
        .run();
}
fn frame_rate(mut rate: ResMut<FramepaceSettings>) {
    rate.limiter = Limiter::from_framerate(75.);
    //dbg!(time.delta_seconds() * 1000.);
}
fn ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nestable_menu_handler: NonSendMut<NestableMenuHolder>,
    mut struct_reg: ResMut<StructureRegistry>,
    data_type_registry: Res<SBDataTypeRegistry>,
) {
    *struct_reg = StructureRegistry::init(&data_type_registry);
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Game,
}
fn insert_non_send(world: &mut World) {
    world.insert_non_send_resource(NestableMenuHolder::default())
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
    rapier_config.gravity = Vec2::new(0., -16. * 20.);
}
