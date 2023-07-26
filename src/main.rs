use bevy::prelude::*;

pub mod component;
pub mod resource;
mod system;

use crate::component::*;
use crate::system::*;
use crate::resource::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some((Window {
                        title: "logion".into(),
                        resolution: (600., 600.).into(),
                        resizable: true,
                        ..default()
                    })),                    
                    ..default()
                    
                }).build()
        )
        .init_resource::<LevelData>()
        .add_systems(Startup, (
            init_word_grid,
            intit_entities
        ))
        .add_systems(Update, (
            move_by_velocity,
            change_dir_at_edge
        ))
        .run();
}
