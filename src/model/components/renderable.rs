use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Renderable {
    pub glyph: char,
    pub color: Color,
}
