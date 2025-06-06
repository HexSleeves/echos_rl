use bevy::prelude::*;
use brtk::prelude::Direction;

use crate::core::{components::Position, types::error::GameError};

/// Your existing trait (which is excellent!)
pub trait GameAction: Send + Sync + 'static + std::fmt::Debug {
    fn action_type(&self) -> ActionType;
    fn perform(&self, world: &mut World) -> Result<u64, GameError>;

    fn priority(&self) -> u8 { 0 } // 0 = normal, higher = more important
    fn can_interrupt(&self) -> bool { false }
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
    // MoveTowards(Position),
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
            // Self::MoveTowards(_) => MOVE_DELTA_TIME,
        }
    }
}
