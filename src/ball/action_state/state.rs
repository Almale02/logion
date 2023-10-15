use bevy::prelude::*;



#[derive(Clone, PartialEq, Eq, Debug, Default, States, Hash)]
pub enum PlayerActionState {
    #[default]
    None,
    BuildSelect,
    BuildPlacement,
}
