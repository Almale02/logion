use bevy::prelude::*;

use crate::ui::nestable_menu::lib::nestable_menu::{NMComponent};

#[derive(Clone)]
pub struct NMTextComponent {
    pub value: &'static str,
    pub id: Entity,
}
impl NMTextComponent {
    pub fn new(value: &'static str) -> Self {
        Self {
            value,
            id: Entity::PLACEHOLDER,
        }
    }
}
impl NMComponent for NMTextComponent {
    fn init(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, parent: Entity) {
        let text_style = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 15.0,
            color: Color::WHITE,
        };
        let mut parent = commands.get_entity(parent).unwrap();

        parent.with_children(|spawn| {
            self.id = spawn
                .spawn((TextBundle {
                    text: Text::from_section(self.value, text_style),
                    transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::new(1., 1., 1.)),
                    style: Style {
                        margin: UiRect::left(Val::Px(10.)),
                        ..default()
                    },
                    ..default()
                },))
                .id();
        });
    }
}
