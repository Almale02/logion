use bevy::prelude::*;

use crate::{
    resource::nestable_menu::NestableMenuHolder, ui::nestable_menu::lib::nestable_menu::NMItemType,
};

use super::{dir_component::NMDirComponent, nest_id_text::NMNestIdText, text::NMTextComponent};

pub fn update_text_components(
    menu_holder: NonSendMut<NestableMenuHolder>,
    mut text: Query<&mut Text>,
) {
    if menu_holder.nestable_menu.is_none() {
        return;
    }
    let menu = menu_holder.nestable_menu.as_ref().unwrap().clone();

    for item in menu.render_buffer {
        if let NMItemType::Item(data) = &item.item_type {
            for component in data {
                if component.is::<NMTextComponent>() {
                    let text_component = component.downcast_ref::<NMTextComponent>().unwrap();
                    if text.get(text_component.id).is_err() {
                        break;
                    }
                    text.get_mut(text_component.id)
                        .unwrap()
                        .sections
                        .first_mut()
                        .unwrap()
                        .value = text_component.value.into();
                }
                if component.is::<NMNestIdText>() {
                    let text_component = component.downcast_ref::<NMNestIdText>().unwrap();
                    if text.get(text_component.id).is_err() {
                        break;
                    }
                    text.get_mut(text_component.id).unwrap().sections[0].value =
                        item.nesting_id.clone();
                }
            }
        }
        if let NMItemType::Directory(data) = &item.item_type {
            if data.is::<NMDirComponent>() {
                let mut textx = item.nesting_id.clone();
                textx.push(':');
                let text_component = data.downcast_ref::<NMDirComponent>().unwrap();
                if text.get(text_component.id).is_err() {
                    break;
                }
                text.get_mut(text_component.id)
                    .unwrap()
                    .sections
                    .first_mut()
                    .unwrap()
                    .value = textx;
            }
        }
        //
    }
}
