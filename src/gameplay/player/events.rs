use crate::core::components::Position;
use bevy::prelude::*;

/// Event fired when the player moves
#[derive(Event, Debug)]
pub struct PlayerMoved {
    pub from: Position,
    pub to: Position,
}

/// Event fired when the player dies
#[derive(Event, Debug)]
pub struct PlayerDied {
    pub cause: String,
}
