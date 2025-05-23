use bevy::prelude::*;

pub fn remove_resource<T: Resource>(mut commands: Commands) { commands.remove_resource::<T>(); }
