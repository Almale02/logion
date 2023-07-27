use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};

pub mod component;
pub mod resource;
mod world_gen;
mod system;


use world_gen::{*, system::{generate_world, gen_world_from_level_data}};

use component::*;
use resource::*;
use system::*;

fn main() {    
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(
                        (Window {
                            title: "logion".into(),
                            resolution: (1000., 600.).into(),
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
        .add_systems(Startup, (
		    frame_rate,
            init_rendering,
            generate_world,
            gen_world_from_level_data.after(generate_world)
	    ))
        .add_systems(Update, (
            move_camera
        ))
        .run();
}
fn frame_rate(
    mut rate: ResMut<FramepaceSettings>,

) {        
    rate.limiter = Limiter::from_framerate(90.)
}   