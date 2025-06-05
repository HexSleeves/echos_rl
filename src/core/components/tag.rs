use bevy::prelude::*;

use super::{Description, Position};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
#[require(Description, Position)]
pub struct Mob;

/// Component that marks an entity as the player
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Description, Position)]
pub struct PlayerTag;

/// Component that marks an entity as AI-controlled
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AITag;

/// Component that marks an entity as dead
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DeadTag;
