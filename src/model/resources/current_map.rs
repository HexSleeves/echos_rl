use bevy::prelude::*;

use crate::model::{ModelConstants, resources::Map};

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct CurrentMap(pub Map);

impl FromWorld for CurrentMap {
    fn from_world(world: &mut World) -> Self {
        Self(Map::new(&mut world.commands(), (ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT)))
    }
}
