use bevy::prelude::*;

use crate::{
    core::types::{ActionCategory, GameAction, GameError},
    impl_debug_with_field, impl_game_action,
};

#[derive(Clone)]
pub struct Wait {
    entity: Entity,
}

impl GameAction for Wait {
    fn category(&self) -> ActionCategory { ActionCategory::Wait }

    fn entity(&self) -> Option<Entity> { Some(self.entity) }

    fn perform(&self, _world: &mut World) -> Result<u64, GameError> {
        info!("Entity {} is waiting", self.entity);
        Ok(1000)
    }
}

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

impl_debug_with_field!(Wait, entity);
impl_game_action!(Wait, WaitBuilder, entity);

///////////////////////////
// Simple actions
///////////////////////////

#[derive(Debug, Clone)]
pub struct WaitAction {
    entity: Entity,
    duration: u64,
}

impl WaitAction {
    pub fn new(entity: Entity, duration: u64) -> Self { Self { entity, duration } }
}

impl GameAction for WaitAction {
    fn entity(&self) -> Option<Entity> { Some(self.entity) }
    fn category(&self) -> ActionCategory { ActionCategory::Wait }
    fn perform(&self, _world: &mut World) -> Result<u64, GameError> { Ok(self.duration) }
}
