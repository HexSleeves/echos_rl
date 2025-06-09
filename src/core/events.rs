use bevy::prelude::*;

use crate::core::components::Position;

/// Event fired when the game starts
#[derive(Event, Debug)]
pub struct GameStarted;

/// Event fired when the game ends
#[derive(Event, Debug)]
pub struct GameEnded {
    pub reason: GameEndReason,
}

#[derive(Debug)]
pub enum GameEndReason {
    PlayerDeath,
    Victory,
    Quit,
}

/// Event fired when an entity takes damage
#[derive(Event, Debug)]
pub struct DamageDealtEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: i32,
    pub position: Position,
}

/// Event fired when an entity dies
#[derive(Event, Debug)]
pub struct EntityDeathEvent {
    pub entity: Entity,
    pub position: Position,
    pub killer: Option<Entity>,
}

/// General combat-related events
#[derive(Event, Debug)]
pub enum CombatEvent {
    AttackHit { attacker: Entity, target: Entity, damage: i32 },
    AttackMissed { attacker: Entity, target: Entity },
    CriticalHit { attacker: Entity, target: Entity, damage: i32 },
}
