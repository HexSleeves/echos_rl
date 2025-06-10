use bevy::prelude::*;
use std::collections::VecDeque;

use brtk::prelude::Direction;
use echos_assets::entities::TurnActorData;

use crate::core::{
    components::Position,
    types::{ActionType, ActionTypeWrapper, GameAction},
};

/// Component for entities that participate in the turn-based system
#[derive(Component, Debug)]
pub struct TurnActor {
    /// Base speed for calculating action times
    pub speed: u32,
    /// Whether this actor is alive and should participate in turns
    pub alive: bool,
    /// Queue of actions to perform (using trait objects for flexibility)
    pub actions: VecDeque<Box<dyn GameAction>>,
}

impl TurnActor {
    /// Create a new TurnActor with the given speed
    pub fn new(speed: u32) -> Self { Self { speed, alive: true, actions: VecDeque::new() } }

    /// Get the next action
    pub fn next_action(&mut self) -> Option<Box<dyn GameAction>> { self.actions.pop_front() }

    /// Peek at the next action without consuming it
    pub fn peek_next_action(&self) -> Option<&dyn GameAction> {
        self.actions.front().map(|action| action.as_ref())
    }

    /// Queue an action trait object
    pub fn queue_action_trait(&mut self, action: Box<dyn GameAction>) { self.actions.push_back(action); }

    /// Queue an ActionType (for backward compatibility with AI systems)
    pub fn queue_action(&mut self, action: ActionType) {
        // Store the ActionType directly - it will be converted to proper actions in the turn system
        self.actions.push_back(Box::new(ActionTypeWrapper::new(action)));
    }

    /// Check if actor has any pending actions
    pub fn has_action(&self) -> bool { !self.actions.is_empty() }

    /// Clear all actions
    pub fn clear_all_actions(&mut self) { self.actions.clear(); }

    /// Mark actor as dead and clear all actions
    pub fn kill(&mut self) {
        self.alive = false;
        self.clear_all_actions();
    }

    /// Revive actor
    pub fn revive(&mut self) { self.alive = true; }

    /// Check if actor is alive
    pub fn is_alive(&self) -> bool { self.alive }

    /// Set alive status directly
    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
        if !alive {
            self.clear_all_actions();
        }
    }

    /// Get current base speed
    pub fn speed(&self) -> u32 { self.speed }

    /// Set new base speed
    pub fn set_speed(&mut self, speed: u32) { self.speed = speed; }

    /// Get number of queued actions
    pub fn action_count(&self) -> usize { self.actions.len() }

    /// Check if actor is ready for a turn (has actions and is alive)
    pub fn is_ready_for_turn(&self) -> bool { self.alive && self.has_action() }
}

impl From<TurnActorData> for TurnActor {
    fn from(data: TurnActorData) -> Self { Self::new(data.speed) }
}

impl From<&TurnActorData> for TurnActor {
    fn from(data: &TurnActorData) -> Self { Self::new(data.speed) }
}

impl Default for TurnActor {
    fn default() -> Self {
        Self::new(100) // Default speed of 100
    }
}

// Convenience methods for common action types
impl TurnActor {
    /// Quick method to queue a wait action
    pub fn queue_wait(&mut self) {
        info!("Queueing wait action");
        self.queue_action(ActionType::Wait);
    }

    /// Quick method to queue a teleport action
    pub fn queue_teleport(&mut self, position: Position) {
        info!("Queueing teleport action to {position:?}");
        self.queue_action(ActionType::Teleport(position))
    }

    /// Quick method to queue an attack action
    pub fn queue_attack(&mut self, position: Position) {
        info!("Queueing attack action to {position:?}");
        self.queue_action(ActionType::Attack(position))
    }

    /// Quick method to queue a move delta action
    pub fn queue_move_delta(&mut self, direction: Direction) {
        info!("Queueing move delta action to {direction:?}");
        self.queue_action(ActionType::MoveDelta(direction))
    }
}
