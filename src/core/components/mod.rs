pub mod position;
pub use position::*;

pub mod tag;
pub use tag::*;

use bevy::prelude::*;

/// Description component for entities
#[derive(Component, Reflect, Default, Debug, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct Description(pub String);

impl Description {
    pub fn new(description: impl ToString) -> Self { Self(description.to_string()) }
}

#[derive(Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct FieldOfView(pub u8);

impl Default for FieldOfView {
    fn default() -> Self { Self(4) }
}

impl FieldOfView {
    pub fn new(radius: u8) -> Self { Self(radius) }
}
