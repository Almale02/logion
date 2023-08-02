use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Identifier {
    pub id: String,
}
