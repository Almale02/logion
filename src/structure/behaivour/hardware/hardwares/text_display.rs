use bevy::prelude::*;

use crate::structure::behaivour::hardware::lib::SBHardwareData;

#[derive(Debug, Clone, Component, Default)]
pub struct SBHTextDisplay {
    pub text: String,
}
impl SBHardwareData for SBHTextDisplay {}
pub fn update_text_display(
    chidrens: Query<(&mut Children, &SBHTextDisplay)>,
    mut text: Query<&mut Text>,
) {
    for chidren in &chidrens {
        let mut iter = text.iter_many_mut(chidren.0);
        let mut first = iter.fetch_next().unwrap();

        first.sections[0].value = chidren.1.text.clone();
    }
}
