use std::ops::{Add, AddAssign};

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

/// Position component for entities in the game world
#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
#[reflect(Component)]
pub struct Position(pub IVec2);

impl Position {
    pub fn new(x: i32, y: i32) -> Self { Self(IVec2::new(x, y)) }
    pub fn x(&self) -> i32 { self.0.x }
    pub fn y(&self) -> i32 { self.0.y }

    pub fn distance_squared(&self, other: &Position) -> i32 { self.0.distance_squared(other.0) }

    pub fn distance(&self, other: &Position) -> f32 { self.0.as_vec2().distance(other.0.as_vec2()) }
}

impl From<IVec2> for Position {
    fn from(vec: IVec2) -> Self { Self(vec) }
}

impl From<Position> for (i32, i32) {
    fn from(value: Position) -> Self { (value.0.x, value.0.y) }
}

impl From<TilePos> for Position {
    fn from(tile_pos: TilePos) -> Self { Self(IVec2::new(tile_pos.x as i32, tile_pos.y as i32)) }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self { Self(IVec2::new(x, y)) }
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output { Self(self.0 + rhs.0) }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) { self.0 += rhs.0; }
}

impl Add<IVec2> for Position {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self::Output { Self(self.0 + rhs) }
}

impl AddAssign<IVec2> for Position {
    fn add_assign(&mut self, rhs: IVec2) { self.0 += rhs; }
}

impl Add<(i32, i32)> for Position {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output { Self(IVec2::new(self.0.x + rhs.0, self.0.y + rhs.1)) }
}

impl AddAssign<(i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0.x += rhs.0;
        self.0.y += rhs.1;
    }
}

/// Description component for entities
#[derive(Component, Reflect, Default, Debug, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct Description(pub String);

impl Description {
    pub fn new(description: impl ToString) -> Self { Self(description.to_string()) }
}

#[derive(Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct FieldOfView(pub u8);

impl Default for FieldOfView {
    fn default() -> Self { Self(4) }
}

impl FieldOfView {
    pub fn new(radius: u8) -> Self { Self(radius) }
}

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

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Mob;
