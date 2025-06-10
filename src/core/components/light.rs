use bevy::prelude::*;

/// Component for entities that emit light.
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Light {
    /// The maximum range of the light.
    pub range: u32,
    /// The color of the light.
    pub color: Color,
    /// How quickly the light intensity falls off with distance.
    /// A value of 1.0 means linear falloff, 2.0 means quadratic, etc.
    pub falloff: f32,
}

impl Default for Light {
    fn default() -> Self { Self { range: 8, color: Color::WHITE, falloff: 1.5 } }
}
