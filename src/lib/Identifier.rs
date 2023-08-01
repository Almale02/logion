use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(PartialEq, Eq, Hash)]
pub struct Identifier {
    pub id: String,
}
