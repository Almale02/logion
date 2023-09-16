use inflector::Inflector;

use bevy::{ecs::component::Tick, input::mouse::MouseButtonInput, prelude::*};

use crate::{
    ball::action_state::{resource::ActionStateData, state::PlayerActionState},
    resource::registry::structures::StructureRegistry,
};

use super::lib::SelectButtonData;

pub fn despawn_selection_ui(mut commands: Commands, action_state_data: Res<ActionStateData>) {
    commands
        .get_entity(action_state_data.build_selection_data.selection_ui)
        .unwrap()
        .despawn_recursive();
}
pub fn handle_struct_select(
    q_button: Query<(&Interaction, &SelectButtonData), Changed<Interaction>>,
    mut action_state_data: ResMut<ActionStateData>,
    mut next_action_state: ResMut<NextState<PlayerActionState>>,
) {
    for (interaction, data) in &q_button {
        match interaction {
            Interaction::None => (),
            Interaction::Hovered => (),
            Interaction::Pressed => {
                action_state_data.build_placement_data.build_struct = data.structure.clone();
                next_action_state.set(PlayerActionState::BuildPlacement);
                next_action_state.set_last_changed(Tick::new(0));
                action_state_data.build_placement_data.click_delay = true;
            }
        }
    }
}
pub fn spawn_selection_ui(
    mut commands: Commands,
    mut action_state_data: ResMut<ActionStateData>,
    struct_registry: Res<StructureRegistry>,
    asset_server: ResMut<AssetServer>,
) {
    let id = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .id();
    commands.get_entity(id).unwrap().with_children(|spawn| {
        spawn
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(20.),
                    height: Val::Percent(20.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(5.),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba_u8(40, 40, 40, 100)),
                ..default()
            })
            .with_children(|spawn| {
                for structure in struct_registry.0.clone() {
                    spawn
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(70.),
                                    height: Val::Px(40.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::rgba_u8(
                                    100, 100, 100, 180,
                                )),
                                ..default()
                            },
                            SelectButtonData {
                                structure: structure.clone(),
                            },
                        ))
                        .with_children(|spawn| {
                            spawn.spawn(
                                TextBundle::from_section(
                                    structure.template_id.unwrap().get_name().to_title_case(),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 18.,
                                        color: Color::BLACK,
                                    },
                                )
                                .with_text_alignment(TextAlignment::Center)
                                .with_style(Style { ..default() }),
                            );
                        });
                }
            });
    });
    action_state_data.build_selection_data.selection_ui = id;
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
