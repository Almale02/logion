use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

#[derive(Clone, Debug, Component, Resource, PartialEq, Default)]
pub struct Vec1(f32);

impl Deref for Vec1 {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Vec1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
