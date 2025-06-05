use bevy::prelude::*;
use echos_assets::entities::TurnActorData;
use std::collections::VecDeque;

use crate::core::{
    actions::WaitAction,
    types::{ActionCategory, GameAction},
};

/// Component for entities that participate in the turn-based system
#[derive(Component, Debug)]
pub struct TurnActor {
    /// Base speed for calculating action times
    pub speed: u32,
    /// Whether this actor is alive and should participate in turns
    pub alive: bool,
    /// Queue of actions to perform
    pub actions: VecDeque<Box<dyn GameAction>>,
}

impl TurnActor {
    /// Create a new TurnActor with the given speed
    pub fn new(speed: u32) -> Self { Self { speed, alive: true, actions: VecDeque::new() } }

    /// Get the next action
    pub fn next_action(&mut self) -> Option<Box<dyn GameAction>> { self.actions.pop_front() }

    /// Peek at the next action without consuming it
    pub fn peek_next_action(&self) -> Option<&dyn GameAction> { self.actions.front().map(|a| a.as_ref()) }

    /// Queue an action
    pub fn queue_action(&mut self, action: Box<dyn GameAction>) { self.actions.push_back(action); }

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
    pub fn queue_wait(&mut self, entity: Entity, duration: u64) {
        self.queue_action(Box::new(WaitAction::new(entity, duration)));
    }

    /// Check if the next action is of a specific category
    pub fn next_action_is_category(&self, category: ActionCategory) -> bool {
        self.peek_next_action().map(|action| action.category()) == Some(category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_actor_creation() {
        let actor = TurnActor::new(100);
        assert_eq!(actor.speed(), 100);
        assert!(actor.is_alive());
        assert!(!actor.has_action());
    }

    #[test]
    fn test_turn_actor_default() {
        let actor = TurnActor::default();
        assert_eq!(actor.speed(), 100);
        assert!(actor.is_alive());
        assert!(!actor.has_action());
    }
}
