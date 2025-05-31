use bevy::prelude::*;

/// Core game states that control the main game loop
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    /// Gather actions from all entities (player + monsters) without processing them
    GatherActions,
    /// Process all queued actions in turn queue order
    #[default]
    ProcessTurns,
}

/// Screen states for different UI screens
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[states(scoped_entities)]
pub enum ScreenState {
    #[default]
    Loading,
    Gameplay,
}
