use bevy::prelude::*;

use crate::{lib::identifier::Identifier, structure::lib::structure::Structure};

#[derive(Clone, PartialEq, Eq, Debug, Default, States, Hash)]
pub enum PlayerActionState {
    #[default]
    None,
    BuildSelect,
    BuildPlacement,
}
