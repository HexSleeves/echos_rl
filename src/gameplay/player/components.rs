use crate::core::components::{Description, Position};
use bevy::prelude::*;

/// Component that marks an entity as the player
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Description, Position)]
pub struct PlayerTag;

/// Component that indicates the player is waiting for input
#[derive(Component, Debug)]
pub struct AwaitingInput;
