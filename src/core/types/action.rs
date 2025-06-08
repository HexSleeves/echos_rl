use bevy::prelude::*;
use brtk::prelude::Direction;

use crate::core::{components::Position, types::error::GameError};

/// Simplified trait for game actions - focused on execution only
pub trait GameAction: Send + Sync + 'static + std::fmt::Debug {
    /// Execute the action and return the time it took
    fn execute(&mut self, world: &mut World) -> Result<u64, GameError>;

    /// Get the action type for identification and time calculation
    fn action_type(&self) -> ActionType;

    /// Get the base duration for this action type
    fn duration(&self) -> u64 { self.action_type().get_base_time_to_perform() }
}

pub const WAIT_TIME: u64 = 1000;
pub const ATTACK_TIME: u64 = 1000;
pub const TELEPORT_TIME: u64 = 1000;
pub const MOVE_DELTA_TIME: u64 = 1000;

#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    Wait,
    MoveDelta(Direction),
    Teleport(Position),
    Attack(Position),
}

impl ActionType {
    pub const fn get_base_time_to_perform(&self) -> u64 {
        match self {
            Self::Wait => WAIT_TIME,
            Self::Attack(_) => ATTACK_TIME,
            Self::Teleport(_) => TELEPORT_TIME,
            Self::MoveDelta(_) => MOVE_DELTA_TIME,
        }
    }

    /// Convert ActionType to a boxed GameAction trait object
    pub fn to_action(self, entity: Entity) -> Box<dyn GameAction> {
        match self {
            ActionType::Wait => Box::new(crate::core::actions::WaitAction::new(entity)),
            ActionType::MoveDelta(direction) => {
                Box::new(crate::core::actions::MoveAction::new(entity, direction))
            }
            ActionType::Teleport(position) => {
                Box::new(crate::core::actions::TeleportAction::new(entity, position))
            }
            ActionType::Attack(position) => {
                Box::new(crate::core::actions::AttackAction::new(entity, position))
            }
        }
    }
}

/// Wrapper for ActionType to implement GameAction trait
/// This allows backward compatibility while we transition to the new system
#[derive(Debug)]
pub struct ActionTypeWrapper {
    action_type: ActionType,
}

impl ActionTypeWrapper {
    pub fn new(action_type: ActionType) -> Self { Self { action_type } }

    pub fn get_action_type(&self) -> ActionType { self.action_type }
}

impl GameAction for ActionTypeWrapper {
    fn action_type(&self) -> ActionType { self.action_type }

    fn execute(&mut self, _world: &mut World) -> Result<u64, crate::core::types::GameError> {
        // This is a placeholder - the actual execution will be handled by converting
        // to proper action types in the turn system
        Ok(self.action_type.get_base_time_to_perform())
    }
}
