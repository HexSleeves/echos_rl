use bevy::prelude::*;

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Description(pub String);

impl Description {
    pub fn new(description: impl ToString) -> Self { Self(description.to_string()) }
}
