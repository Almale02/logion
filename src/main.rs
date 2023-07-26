use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};

pub mod component;
pub mod resource;
mod system;

use crate::component::*;
use crate::resource::*;
use crate::system::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(
                        (Window {
                            title: "logion".into(),
                            resolution: (600., 600.).into(),
                            resizable: true,
                            ..default()
                        }),
                    ),
                    ..default()
                })
                .build(),
            bevy_framepace::FramepacePlugin,
        ))
        .init_resource::<LevelData>()
        //.add_systems(Startup, ())
        //.add_systems(Update, ())
        .run();
}
fn frame_rate(mut rate: ResMut<FramepaceSettings>) {
    rate.limiter = Limiter::from_framerate(90.)
}
