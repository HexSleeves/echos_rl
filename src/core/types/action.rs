use bevy::prelude::*;
use brtk::prelude::Direction;

use crate::core::{components::Position, types::error::GameError};

pub const TURN_TIME: u32 = 1000;
pub const WAIT_TIME: u32 = 1000;
pub const ATTACK_TIME: u32 = 1000;

#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    Wait,
    Move(Position),
    MoveDelta(Direction),
    Attack(Position),
}

impl ActionType {
    pub const fn get_base_time_to_perform(&self) -> u32 {
        match self {
            Self::Wait => WAIT_TIME,
            Self::Attack(_) => ATTACK_TIME,
            Self::Move(_) => TURN_TIME,
            Self::MoveDelta(_) => TURN_TIME,
        }
    }
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
        impl $crate::core::types::GameActionBuilder for $builder {
            type Action = $action;
            fn build(self) -> $action {
                $action {
                    $(
                        $field: self.$field.expect(concat!(stringify!($field), " must be set")),
                    )+
                }
            }
        }

        impl $crate::core::types::BuildableGameAction for $action {
            type Builder = $builder;
            fn builder() -> Self::Builder {
                $builder::new()
            }
        }
    };
}
