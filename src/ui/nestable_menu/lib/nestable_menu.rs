use bevy::prelude::*;
use dyn_clone::DynClone;
use mopa::{mopafy, Any};
use std::{cmp::Ordering, fmt::Debug};

use crate::{
    lib::nesting_tree::nesting_tree::NestingTree,
    ui::nestable_menu::builtin_components::{
        dir_component::NMDirComponent, nest_id_text::NMNestIdText,
    },
};

pub trait NMComponent: DynClone + Any {
    fn init(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, parent: Entity);
}
mopafy!(NMComponent);
impl std::fmt::Debug for dyn NMComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "temp")
    }
}
dyn_clone::clone_trait_object!(NMComponent);

#[derive(Clone, Debug)]
pub enum NMItemType {
    Directory(Box<dyn NMComponent>),
    Item(Vec<Box<dyn NMComponent>>),
}
impl NMItemType {
    pub fn is_endpoint(&self) -> bool {
        return match self {
            Self::Item(_) => true,
            Self::Directory(_) => false,
        };
    }
}
impl Default for NMItemType {
    fn default() -> Self {
        Self::Directory(Box::new(NMDirComponent::new()))
    }
}

#[derive(Clone, Debug)]
pub struct NMItem {
    pub item_type: NMItemType,
    pub nesting_id: String,
    pub id: Entity,
}
impl NMItem {
    pub fn new(item_type: NMItemType) -> Self {
        Self {
            item_type,
            nesting_id: "".into(),
            id: Entity::PLACEHOLDER,
        }
    }
    pub fn new_nesting(item_type: NMItemType, nesting_id: String) -> Self {
        Self {
            item_type,
            nesting_id,
            id: Entity::PLACEHOLDER,
        }
    }
}
impl PartialEq for NMItem {
    fn eq(&self, other: &Self) -> bool {
        return self.item_type.is_endpoint() == other.item_type.is_endpoint();
    }
}
impl PartialOrd for NMItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if !self.item_type.is_endpoint() && other.item_type.is_endpoint() {
            return Some(Ordering::Greater);
        }
        if self.item_type.is_endpoint() && !other.item_type.is_endpoint() {
            return Some(Ordering::Less);
        }
        return Some(Ordering::Equal);
    }
}
impl Ord for NMItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Eq for NMItem {
    fn assert_receiver_is_total_eq(&self) {}
}
impl Default for NMItem {
    fn default() -> Self {
        Self {
            item_type: NMItemType::default(),
            nesting_id: "".into(),
            id: Entity::PLACEHOLDER,
        }
    }
}
#[derive(Clone, Debug)]
pub struct NestableMenu {
    pub root: Entity,
    pub container: Entity,
    pub render_buffer: Vec<NMItem>,
    pub selected_index: usize,
    pub nesting_tree: NestingTree,
}
impl NestableMenu {
    pub fn new(render_buffer: Vec<NMItem>) -> NestableMenu {
        NestableMenu {
            root: Entity::PLACEHOLDER,
            container: Entity::PLACEHOLDER,
            render_buffer,
            selected_index: 0,
            nesting_tree: NestingTree::default(),
        }
    }
    pub fn nest_down_to(&mut self, dir: String) {
        let new_chidren = self.nesting_tree.down_to(dir);

        self.render_buffer.clear();

        for (_key, val) in new_chidren.iter() {
            let is_endpoint = val.endpoint;

            if is_endpoint {
                self.render_buffer.push(NMItem::new_nesting(
                    NMItemType::Item(vec![Box::new(NMNestIdText::new())]),
                    val.value.clone(),
                ))
            } else {
                self.render_buffer.push(NMItem::new_nesting(
                    NMItemType::Directory(Box::new(NMDirComponent::new())),
                    val.value.clone(),
                ))
            }
        }
        self.render_buffer.sort();
    }
    pub fn nest_up(&mut self) {
        let new_chidren = self.nesting_tree.up();

        self.render_buffer.clear();

        for (_key, val) in new_chidren.iter() {
            let is_endpoint = val.endpoint;

            if is_endpoint {
                self.render_buffer.push(NMItem::new_nesting(
                    NMItemType::Item(vec![Box::new(NMNestIdText::new())]),
                    val.value.clone(),
                ))
            } else {
                self.render_buffer.push(NMItem::new_nesting(
                    NMItemType::Directory(Box::new(NMDirComponent::new())),
                    val.value.clone(),
                ))
            }
        }
        self.render_buffer.sort();
    }
    pub fn init(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        self.root = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                //background_color: BackgroundColor(Color::rgba_u8(10, 70, 20, 50)),
                ..default()
            })
            .id();
        commands.entity(self.root).with_children(|spawn| {
            self.container = spawn
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.),
                        height: Val::Percent(70.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::top(Val::Px(6.)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba_u8(10, 70, 120, 100)),
                    border_color: BorderColor(*Color::RED.clone().set_r(0.)),
                    ..default()
                })
                .id();
        });
        self.nest_down_to("root".into());
        self.change_render_buffer(commands, asset_server);
    }
    pub fn change_render_buffer(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        if self.render_buffer.len() > 0 {
            self.selected_index = self.selected_index.clamp(0, self.render_buffer.len() - 1);
        }
        commands
            .get_entity(self.container)
            .unwrap()
            .despawn_descendants();
        for item_type in self.render_buffer.iter_mut() {
            let mut item_container = Entity::PLACEHOLDER;

            commands.entity(self.container).with_children(|spawn| {
                item_container = spawn
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(10.),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            border: UiRect::left(Val::Px(20.)),
                            ..default()
                        },
                        border_color: BorderColor(Color::RED),
                        background_color: BackgroundColor(Color::rgba_u8(50, 90, 100, 100)),
                        ..default()
                    })
                    .id();
            });

            item_type.id = item_container;

            let _item_data = item_type.clone();
            if let NMItemType::Item(item) = &mut item_type.item_type {
                for component in item.iter_mut() {
                    component.init(commands, asset_server, item_container)
                }
            }
            if let NMItemType::Directory(dir) = &mut item_type.item_type {
                dir.init(commands, asset_server, item_container)
            }
        }
    }
}
impl Default for NestableMenu {
    fn default() -> Self {
        Self {
            root: Entity::PLACEHOLDER,
            container: Entity::PLACEHOLDER,
            render_buffer: Vec::default(),
            selected_index: 0,
            nesting_tree: NestingTree::default(),
        }
    }
}
