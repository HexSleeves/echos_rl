use bevy::prelude::*;
use brtk::prelude::Direction;

use crate::core::types::error::GameError;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ActionType {
    Move(Direction),
    Attack(Entity),
    Wait,
    // Other actions
}

pub trait GameAction: Send + Sync + 'static + std::fmt::Debug {
    fn entity(&self) -> Option<Entity>;
    fn perform(&self, world: &mut World) -> Result<u64, GameError>;
}

/// Builder trait for GameAction implementations
pub trait GameActionBuilder: Send + Sync + 'static {
    /// The action type this builder creates
    type Action: GameAction;

    /// Build the final action
    fn build(self) -> Self::Action;
}

pub trait BuildableGameAction: GameAction {
    type Builder: GameActionBuilder<Action = Self>;

    fn builder() -> Self::Builder
    where
        Self: Sized;
}

// Macro moved to core::types::action - use that version instead
