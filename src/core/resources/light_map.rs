use bevy::prelude::{Color, Resource};
use std::collections::HashMap;

/// A resource that stores the combined light intensity and color for each tile on the map.
#[derive(Resource, Default, Debug, Clone)]
pub struct LightMap {
    pub map: HashMap<(i32, i32), Color>,
}

impl LightMap {
    pub fn new() -> Self { Self { map: HashMap::new() } }

    /// Sets the light color for a given position.
    pub fn set_light(&mut self, position: (i32, i32), color: Color) { self.map.insert(position, color); }

    /// Gets the light color for a given position, or black if no light is present.
    pub fn get_light(&self, position: (i32, i32)) -> Color {
        *self.map.get(&position).unwrap_or(&Color::BLACK)
    }

    /// Clears all light information from the map.
    pub fn clear(&mut self) { self.map.clear(); }
}
