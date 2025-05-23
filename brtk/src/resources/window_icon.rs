use bevy::prelude::*;

#[derive(Resource, Deref)]
pub struct WindowIcon(pub &'static [u8]);
