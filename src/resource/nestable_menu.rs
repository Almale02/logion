use bevy::prelude::*;

use crate::ui::nestable_menu::lib::nestable_menu::NestableMenu;

#[derive(Debug, Clone, Default)]
pub struct NestableMenuHolder {
    pub nestable_menu: Option<NestableMenu>,
}
impl NestableMenuHolder {
    pub fn add(
        &mut self,
        menu: NestableMenu,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        self.nestable_menu = Some(menu);
        self.nestable_menu
            .as_mut()
            .unwrap()
            .init(commands, asset_server);
    }
    pub fn hide(&mut self, commands: &mut Commands) {
        commands
            .entity(self.nestable_menu.clone().unwrap().root)
            .despawn_recursive();
        self.nestable_menu = None;
    }
}
