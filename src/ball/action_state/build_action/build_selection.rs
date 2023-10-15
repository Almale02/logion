use inflector::Inflector;

use bevy::{ecs::component::Tick, prelude::*};

use crate::{
    ball::action_state::{resource::ActionStateData, state::PlayerActionState},
    lib::{identifier::Identifier, nesting_tree::nesting_tree::NTPath},
    resource::{
        nestable_menu::NestableMenuHolder,
        registry::{sb_data_type_registry::SBDataTypeRegistry, structures::StructureRegistry},
    },
    ui::nestable_menu::lib::nestable_menu::NestableMenu,
};

use super::lib::SelectButtonData;

pub fn despawn_selection_ui(
    mut commands: Commands,
    mut nestable_menu_holder: NonSendMut<NestableMenuHolder>,
) {
    nestable_menu_holder.hide(&mut commands)
}
pub fn spawn_selection_ui(
    mut commands: Commands,
    mut nestable_menu_holder: NonSendMut<NestableMenuHolder>,
    data_type_registry: Res<SBDataTypeRegistry>,
    asset_server: Res<AssetServer>,
) {
    let mut nestable_menu = NestableMenu::default();

    nestable_menu.nesting_tree = StructureRegistry::init(&data_type_registry).get_nesting_tree();

    nestable_menu_holder.add(nestable_menu, &mut commands, &asset_server);
}
pub fn handle_select(
    nestable_menu_holder: NonSend<NestableMenuHolder>,
    struct_registry: Res<StructureRegistry>,
    key_code: Res<Input<KeyCode>>,
    mut action_data: ResMut<ActionStateData>,
    mut next_player_action_state: ResMut<NextState<PlayerActionState>>,
) {
    if key_code.just_pressed(KeyCode::Return) {
        let menu = nestable_menu_holder.nestable_menu.as_ref().unwrap();
        if !menu.render_buffer[menu.selected_index]
            .item_type
            .is_endpoint()
        {
            return;
        }
        let select_path = NTPath::from_parts(
            menu.nesting_tree.pointer.clone(),
            menu.render_buffer[menu.selected_index].nesting_id.clone(),
        );
        action_data.build_placement_data.struct_build_data = struct_registry
            .0
            .get(&Identifier::new_structure(&select_path.into_string()))
            .unwrap()
            .clone();
        next_player_action_state.set(PlayerActionState::BuildPlacement);
    }
}

#[derive(Clone, Debug)]
pub enum SelectedStructure {
    Triangle,
}
impl Default for SelectedStructure {
    fn default() -> Self {
        SelectedStructure::Triangle
    }
}
