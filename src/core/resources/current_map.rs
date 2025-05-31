use bevy::prelude::*;

use crate::core::{constants::ModelConstants, resources::Map};

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct CurrentMap(pub Map);

impl FromWorld for CurrentMap {
    fn from_world(_world: &mut World) -> Self {
        Self(Map::new((ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT)))
    }
}
