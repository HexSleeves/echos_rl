use bevy::prelude::*;

use crate::{
    core::{
        components::Position,
        types::{ActionType, GameAction, GameError},
    },
    debug_combat,
};

#[derive(Clone, Debug)]
pub struct AttackAction {
    entity: Entity,
    target_position: Position,
}

impl AttackAction {
    pub fn new(entity: Entity, target_position: Position) -> Self { Self { entity, target_position } }
}

impl GameAction for AttackAction {
    fn action_type(&self) -> ActionType { ActionType::Attack(self.target_position) }

    fn execute(&self, _world: &mut World) -> Result<u64, GameError> {
        // Placeholder implementation - just log the attack for now
        debug_combat!("Entity {} attacks position {:?}", self.entity, self.target_position);

        // TODO: Implement actual attack logic when combat system is added
        // - Check if target position has an entity
        // - Calculate damage
        // - Apply damage to target
        // - Handle death/destruction

        Ok(self.duration())
    }
}
