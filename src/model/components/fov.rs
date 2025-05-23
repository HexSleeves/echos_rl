use bevy::prelude::*;

/// Add this as a component to entities that can see
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ViewShed {
    pub radius: i32,
}
