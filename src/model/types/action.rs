use bevy::prelude::*;

use crate::model::types::{direction::MoveDirection, error::GameError};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ActionType {
    Move(MoveDirection),
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

#[macro_export]
macro_rules! impl_game_action {
    ($action:ident, $builder:ident, $( $field:ident ),+ ) => {
        impl $crate::model::types::GameActionBuilder for $builder {
            type Action = $action;
            fn build(self) -> $action {
                $action {
                    $(
                        $field: self.$field.expect(concat!(stringify!($field), " must be set")),
                    )+
                }
            }
        }

        impl $crate::model::types::BuildableGameAction for $action {
            type Builder = $builder;
            fn builder() -> Self::Builder {
                $builder::new()
            }
        }
    };
}
