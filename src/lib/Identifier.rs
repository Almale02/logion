use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Identifier {
    pub id: String,
}
