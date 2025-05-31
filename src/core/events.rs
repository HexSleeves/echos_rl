use bevy::prelude::*;

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
