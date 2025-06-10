use bevy::{ecs::system::SystemState, prelude::*};
use brtk::prelude::Direction;

use crate::{
    core::{
        actions::AttackAction,
        components::Position,
        resources::CurrentMap,
        types::{ActionType, GameAction, GameError},
    },
    gameplay::world::components::TerrainType,
};

#[derive(Clone, Debug)]
pub struct MoveAction {
    entity: Entity,
    direction: Direction,
}

impl MoveAction {
    pub fn new(entity: Entity, direction: Direction) -> Self { Self { entity, direction } }
}

impl GameAction for MoveAction {
    fn action_type(&self) -> ActionType { ActionType::MoveDelta(self.direction) }

    fn execute(&mut self, world: &mut World) -> Result<u64, GameError> {
        let mut state: SystemState<(ResMut<CurrentMap>, Query<&mut Position>)> = SystemState::new(world);

        // Get references to the data
        let (mut current_map, mut q_position) = state.get_mut(world);

        // Get the entity's current position
        let current_pos = match q_position.get(self.entity).copied() {
            Ok(pos) => pos,
            Err(_) => return Err(GameError::EntityNotFound(self.entity)),
        };

        let new_pos = current_pos + self.direction.coord();

        let Some(terrain_type) = current_map.get_terrain(new_pos) else {
            log::error!("Failed to get terrain type for entity: {}", self.entity);
            return Err(GameError::MissingComponent { entity: self.entity, component: "TerrainType" });
        };

        // Check for wall collision first
        match terrain_type {
            TerrainType::Wall => {
                log::error!("Wall in the way");
                return Err(GameError::MovementBlocked {
                    from: current_pos,
                    to: new_pos,
                    reason: "Wall in the way".to_string(),
                });
            }
            _ => {
                // Check if target position is occupied by another entity (bump-to-attack)
                if let Some(_target_entity) = current_map.get_actor(new_pos) {
                    // Release the system state before creating attack action
                    state.apply(world);

                    // Convert movement to attack
                    log::info!("Movement into occupied space - converting to attack at {new_pos:?}");
                    let mut attack_action = AttackAction::new(self.entity, new_pos);
                    return attack_action.execute(world);
                }

                // Normal movement - update position
                if let Ok(mut current_pos_mut) = q_position.get_mut(self.entity) {
                    // Update the map's actor tracking
                    if let Err(e) = current_map.move_actor(self.entity, new_pos) {
                        log::error!("Failed to move actor on map: {e}");
                        return Err(GameError::MovementBlocked {
                            from: *current_pos_mut,
                            to: new_pos,
                            reason: e,
                        });
                    }

                    // Update the entity's position component
                    *current_pos_mut = new_pos;
                }
            }
        }

        // Return the system state to update the world
        state.apply(world);

        Ok(self.duration())
    }
}

// Keep the old Walk struct for backward compatibility if needed
#[derive(Clone, Debug)]
pub struct Walk {
    entity: Entity,
    direction: Direction,
}

impl Walk {
    pub fn new(entity: Entity, direction: Direction) -> Self { Self { entity, direction } }
}

impl GameAction for Walk {
    fn action_type(&self) -> ActionType { ActionType::MoveDelta(self.direction) }

    fn execute(&mut self, world: &mut World) -> Result<u64, GameError> {
        // Delegate to MoveAction for the actual implementation
        MoveAction::new(self.entity, self.direction).execute(world)
    }
}
