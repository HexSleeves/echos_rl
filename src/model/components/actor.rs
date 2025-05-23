use bevy::prelude::*;

use crate::model::components::{Description, Position};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Description, Position)]
pub struct PlayerTag;

#[derive(Component, Debug)]
pub struct AITag;

#[derive(Component, Debug)]
pub struct DeadTag;
