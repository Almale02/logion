use bevy::prelude::*;

use crate::ball::system::*;
use crate::GameState;

use crate::world_gen::ore_gen::system::*;
use crate::world_gen::system::*;
use crate::world_load::system::load_world;
use crate::*;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (
                // SECTION: terrain height
                gen_terrain_height,
                fill_terrain_list,
                // SECTION: smooth world
                generate_grass,
                generate_grass,
                // SECTION: ore gen
                gen_ore_patch_centers,
                materialize,
                // SECTION: laod world
                load_world,
                // SECTION: after gen
                spawn_ball,
                init_rendering,
                init_physics,
            )
                .chain(),
        );
    }
}
