use bevy::prelude::*;
use std::marker::PhantomData;

pub struct Identifier<T: Identifiable> {
    pub id: String,
    phantom: PhantomData<T>,
}
impl<T: Identifiable> Identifier<T> {
    #[allow(dead_code)]
    fn get_as_string(&self) -> String {
        T::id_prefix() + &self.id
    }
}

pub trait Identifiable {
    fn id_prefix() -> String;
}
