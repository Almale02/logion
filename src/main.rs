use bevy::prelude::*;

pub mod component;
mod system;

use crate::component::*;
use crate::system::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(PreStartup, init)
        .run();
}
