use bevy::prelude::*;

use crate::{
    impl_debug_with_field, impl_game_action,
    model::types::{GameAction, GameError},
};

#[derive(Default)]
pub struct WaitBuilder {
    entity: Option<Entity>,
}

impl WaitBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entity = Some(entity);
        self
    }
}

pub struct Wait {
    entity: Entity,
}

impl GameAction for Wait {
    fn entity(&self) -> Option<Entity> { Some(self.entity) }
    fn perform(&self, _world: &mut World) -> Result<u64, GameError> {
        log::info!("Entity {} is waiting", self.entity);
        Ok(1000)
    }
}

impl_debug_with_field!(Wait, entity);
impl_game_action!(Wait, WaitBuilder, entity);
