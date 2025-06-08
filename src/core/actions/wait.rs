use bevy::prelude::*;

use crate::core::types::{ActionType, GameAction, GameError};

#[derive(Debug, Clone)]
pub struct WaitAction {
    entity: Entity,
    duration: u64,
}

impl WaitAction {
    pub fn new(entity: Entity) -> Self {
        Self { entity, duration: ActionType::Wait.get_base_time_to_perform() }
    }

    pub fn new_with_duration(entity: Entity, duration: u64) -> Self { Self { entity, duration } }
}

impl GameAction for WaitAction {
    fn action_type(&self) -> ActionType { ActionType::Wait }

    fn execute(&self, _world: &mut World) -> Result<u64, GameError> {
        info!("Entity {} is waiting", self.entity);
        Ok(self.duration)
    }

    fn duration(&self) -> u64 { self.duration }
}
