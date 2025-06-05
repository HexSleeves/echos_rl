use bevy::{ecs::system::SystemState, prelude::*};
use brtk::prelude::Direction;

use crate::{
    core::{
        components::Position,
        resources::CurrentMap,
        types::{ActionCategory, GameAction, GameError},
    },
    gameplay::world::components::TerrainType,
    impl_debug_with_field, impl_game_action,
};

#[derive(Clone)]
pub struct Walk {
    entity: Entity,
    direction: Direction,
}

impl GameAction for Walk {
    fn category(&self) -> ActionCategory { ActionCategory::Movement }

    fn entity(&self) -> Option<Entity> { Some(self.entity) }

    fn perform(&self, world: &mut World) -> Result<u64, GameError> {
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

        Ok(1000)
    }
}

#[derive(Default, Clone)]
pub struct WalkBuilder {
    entity: Option<Entity>,
    direction: Option<Direction>,
}

impl WalkBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entity = Some(entity);
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }
}

impl_debug_with_field!(Walk, direction);
impl_game_action!(Walk, WalkBuilder, entity, direction);
