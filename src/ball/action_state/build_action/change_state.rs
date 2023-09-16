use bevy::{ecs::component::Tick, prelude::*};

use crate::ball::action_state::state::PlayerActionState;

pub fn change_build_state(
    player_action_state: Res<State<PlayerActionState>>,
    mut next_player_action_state: ResMut<NextState<PlayerActionState>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        next_player_action_state.set_last_changed(Tick::new(0));
        match player_action_state.get() {
            PlayerActionState::BuildSelect => {
                next_player_action_state.set(PlayerActionState::BuildPlacement);
            }
            PlayerActionState::BuildPlacement => {
                next_player_action_state.set(PlayerActionState::BuildSelect);
            }
            PlayerActionState::None => {
                next_player_action_state.set(PlayerActionState::BuildPlacement);
            }
        }
    }
}
