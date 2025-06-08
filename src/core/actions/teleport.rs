use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    core::{
        components::Position,
        resources::CurrentMap,
        types::{ActionType, GameAction, GameError},
    },
    gameplay::world::components::TerrainType,
};

#[derive(Clone, Debug)]
pub struct TeleportAction {
    entity: Entity,
    target_position: Position,
}

impl TeleportAction {
    pub fn new(entity: Entity, target_position: Position) -> Self { Self { entity, target_position } }
}

impl GameAction for TeleportAction {
    fn action_type(&self) -> ActionType { ActionType::Teleport(self.target_position) }

    fn execute(&self, world: &mut World) -> Result<u64, GameError> {
        let mut state: SystemState<(ResMut<CurrentMap>, Query<&mut Position>)> = SystemState::new(world);
        let (current_map, mut q_position) = state.get_mut(world);

        // Get the entity's current position
        if let Ok(mut current_pos) = q_position.get_mut(self.entity) {
            let Some(terrain_type) = current_map.get_terrain(self.target_position) else {
                log::error!("Failed to get terrain type for target position: {:?}", self.target_position);
                return Err(GameError::MissingComponent { entity: self.entity, component: "TerrainType" });
            };

            match terrain_type {
                TerrainType::Wall => {
                    log::error!("Target position is blocked by wall");
                    return Err(GameError::MovementBlocked {
                        from: *current_pos,
                        to: self.target_position,
                        reason: "Target position blocked by wall".to_string(),
                    });
                }
                _ => {
                    *current_pos = self.target_position;
                    log::info!("Entity {} teleported to {:?}", self.entity, self.target_position);
                }
            }
        } else {
            return Err(GameError::EntityNotFound(self.entity));
        }

        // Apply the system state changes
        state.apply(world);
        Ok(self.duration())
    }
}
