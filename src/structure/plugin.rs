use bevy::prelude::*;

use crate::{
    resource::registry::{
        sb_data_type_registry::data_type_register, sb_function_registry::function_registry,
    },
    GameState,
};

use super::behaivour::{
    hardware::hardwares::text_display::update_text_display, logic::executing::update::exec_scrpits,
};

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (function_registry, data_type_register).chain())
            .add_systems(
                Update,
                (exec_scrpits, update_text_display).run_if(in_state(GameState::Game)),
            );
    }
}
