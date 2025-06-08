use bevy::{ecs::system::SystemState, prelude::*};
use brtk::prelude::Direction;

use crate::{
    core::{
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

    fn execute(&self, world: &mut World) -> Result<u64, GameError> {
        let mut state: SystemState<(ResMut<CurrentMap>, Query<&mut Position>)> = SystemState::new(world);

        // Get references to the data
        let (current_map, mut q_position) = state.get_mut(world);

        // Get the entity's current position
        if let Ok(mut current_pos) = q_position.get_mut(self.entity) {
            let new_pos = *current_pos + self.direction.coord();

            let Some(terrain_type) = current_map.get_terrain(new_pos) else {
                log::error!("Failed to get terrain type for entity: {}", self.entity);
                return Err(GameError::MissingComponent { entity: self.entity, component: "TerrainType" });
            };

            match terrain_type {
                TerrainType::Wall => {
                    log::error!("Wall in the way");
                    return Err(GameError::MovementBlocked {
                        from: *current_pos,
                        to: new_pos,
                        reason: "Wall in the way".to_string(),
                    });
                }
                _ => {
                    *current_pos = new_pos;
                }
            }
        } else {
            return Err(GameError::EntityNotFound(self.entity));
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

    fn execute(&self, world: &mut World) -> Result<u64, GameError> {
        // Delegate to MoveAction for the actual implementation
        let move_action = MoveAction::new(self.entity, self.direction);
        move_action.execute(world)
    }
}
