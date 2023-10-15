use bevy::prelude::*;

use crate::GameState;

use super::{
    action_state::{
        build_action::{
            build_placement::{
                build_placement, delete_build_ghost, move_build_ghost, spawn_build_ghost,
            },
            build_selection::{despawn_selection_ui, handle_select, spawn_selection_ui},
            change_state::change_build_state,
        },
        resource::ActionStateData,
        state::PlayerActionState,
    },
    movement::system::{move_camera, CameraFollowState},
    system::move_ball,
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_ball,
                move_camera.run_if(in_state(CameraFollowState::PlayerMain)),
                change_build_state,
            )
                .run_if(in_state(GameState::Game)),
        )
        // SECTION: BUILD_PLACEMENT
        .add_systems(
            Update,
            (move_build_ghost, build_placement.after(Xx::A))
                .run_if(in_state(PlayerActionState::BuildPlacement)),
        )
        .add_systems(
            OnEnter(PlayerActionState::BuildPlacement),
            spawn_build_ghost,
        )
        .add_systems(
            OnExit(PlayerActionState::BuildPlacement),
            delete_build_ghost,
        )
        // SECTION: BUILD_SELECT
        .add_systems(
            Update,
            (handle_select).run_if(in_state(PlayerActionState::BuildSelect)),
        )
        .add_systems(OnEnter(PlayerActionState::BuildSelect), spawn_selection_ui)
        .add_systems(
            OnExit(PlayerActionState::BuildSelect),
            despawn_selection_ui.in_set(Xx::A),
        )
        //
        .init_resource::<ActionStateData>()
        .add_state::<PlayerActionState>();
    }
}
#[derive(Hash, Debug, Eq, PartialEq, Clone, SystemSet)]
enum Xx {
    A,
    B,
}
