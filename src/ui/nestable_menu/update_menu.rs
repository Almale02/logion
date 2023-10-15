use bevy::prelude::*;

use crate::resource::nestable_menu::NestableMenuHolder;

use super::lib::nestable_menu::NMItemType;

pub fn update_nestable_menu(
    mut commands: Commands,
    mut colors: Query<(&mut BackgroundColor, &mut BorderColor)>,
    asset_server: Res<AssetServer>,
    mut nestable_menu_holder: NonSendMut<NestableMenuHolder>,
    key_code: Res<Input<KeyCode>>,
    mut timer: Local<u8>,
) {
    if nestable_menu_holder.nestable_menu.is_none() {
        return;
    }
    let menu = nestable_menu_holder.nestable_menu.as_mut().unwrap();

    // SECTION: update selected item

    if key_code.pressed(KeyCode::K) && *timer == 0 {
        *timer = 10;
        if menu.selected_index < menu.render_buffer.len() - 1 {
            menu.selected_index += 1
        } else {
            menu.selected_index = 0;
        }
    }
    if key_code.pressed(KeyCode::L) && *timer == 0 {
        *timer = 10;
        if menu.selected_index > 0 {
            menu.selected_index -= 1
        } else {
            menu.selected_index = menu.render_buffer.len() - 1
        }
    }
    if key_code.just_pressed(KeyCode::J) {
        menu.nest_up();
        menu.change_render_buffer(&mut commands, &asset_server)
    }
    if key_code.just_pressed(KeyCode::Semicolon) {
        if let NMItemType::Directory(_) = &menu.render_buffer[menu.selected_index].item_type {
            let dir = &menu.render_buffer[menu.selected_index].nesting_id;

            menu.nest_down_to(dir.into());
            menu.change_render_buffer(&mut commands, &asset_server)
        }
    }
    if *timer > 0 {
        *timer -= 1;
    }

    // SECTION: update menu
    for (idx, item_type) in menu.render_buffer.iter_mut().enumerate() {
        // SECTION: highlight if selected
        if colors.get(item_type.id).is_err() {
            break;
        }
        let (mut background, mut border_color) = colors.get_mut(item_type.id).unwrap();

        if idx == menu.selected_index {
            background.0 = Color::rgba_u8(10, 80, 100, 100);
            border_color.0.set_r(0.65);
        } else {
            background.0 = Color::rgba_u8(50, 90, 100, 100);
            border_color.0.set_r(1.);
        }
    }
    if key_code.just_pressed(KeyCode::G) {
        nestable_menu_holder.hide(&mut commands);
    }
}
