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
    /// AI's preferred action (set by AI systems)
    pub preferred_action: Option<Box<dyn GameAction>>,
    /// Forced actions (interrupts, knockbacks, etc.) - highest priority
    pub forced_actions: VecDeque<Box<dyn GameAction>>,
    /// Speed modifier for buffs/debuffs
    pub action_speed_modifier: f32,
    /// Maximum number of queued actions to prevent memory issues
    pub max_queued_actions: usize,
}

impl TurnActor {
    /// Create a new TurnActor with the given speed
    pub fn new(speed: u32) -> Self {
        Self {
            speed,
            alive: true,
            preferred_action: None,
            actions: VecDeque::new(),
            forced_actions: VecDeque::new(),
            action_speed_modifier: 1.0,
            max_queued_actions: 10,
        }
    }

    /// Create a new TurnActor with custom queue limit
    pub fn with_queue_limit(speed: u32, max_queued_actions: usize) -> Self {
        Self {
            speed,
            alive: true,
            actions: VecDeque::new(),
            preferred_action: None,
            forced_actions: VecDeque::new(),
            action_speed_modifier: 1.0,
            max_queued_actions,
        }
    }

    /// Get the next action with proper priority handling
    /// Priority: Forced > Queued > Preferred
    pub fn next_action(&mut self) -> Option<Box<dyn GameAction>> {
        // 1. Forced actions first (highest priority)
        if let Some(forced) = self.forced_actions.pop_front() {
            return Some(forced);
        }

        // 2. Regular queued actions
        if let Some(action) = self.actions.pop_front() {
            return Some(action);
        }

        // 3. AI preferred action (lowest priority)
        self.preferred_action.take()
    }

    /// Peek at the next action without consuming it
    pub fn peek_next_action(&self) -> Option<&dyn GameAction> {
        if let Some(forced) = self.forced_actions.front() {
            return Some(forced.as_ref());
        }

        if let Some(action) = self.actions.front() {
            return Some(action.as_ref());
        }

        self.preferred_action.as_ref().map(|a| a.as_ref())
    }

    /// Queue a regular action (respects queue limit)
    pub fn queue_action(&mut self, action: Box<dyn GameAction>) -> Result<(), Box<dyn GameAction>> {
        if self.actions.len() >= self.max_queued_actions {
            return Err(action);
        }
        self.actions.push_back(action);
        Ok(())
    }

    /// Add action to front of queue (higher priority than normal queue)
    pub fn queue_action_priority(&mut self, action: Box<dyn GameAction>) -> Result<(), Box<dyn GameAction>> {
        if self.actions.len() >= self.max_queued_actions {
            return Err(action);
        }
        self.actions.push_front(action);
        Ok(())
    }

    /// Force queue an action (ignores queue limit)
    pub fn force_queue_action(&mut self, action: Box<dyn GameAction>) { self.actions.push_back(action); }

    /// Set AI preferred action (will be overwritten if set again)
    pub fn set_preferred_action(&mut self, action: Box<dyn GameAction>) {
        self.preferred_action = Some(action);
    }

    /// Get the current preferred action without consuming it
    pub fn get_preferred_action(&self) -> Option<&dyn GameAction> {
        self.preferred_action.as_ref().map(|a| a.as_ref())
    }

    /// Clear the preferred action (called after player turns)
    pub fn clear_preferred_action(&mut self) { self.preferred_action = None; }

    /// Force an action (for interrupts, knockbacks, etc.)
    pub fn force_action(&mut self, action: Box<dyn GameAction>) {
        if action.can_interrupt() {
            // Interrupting actions go to the very front
            self.forced_actions.push_front(action);
        } else {
            // Non-interrupting forced actions go to the back of forced queue
            self.forced_actions.push_back(action);
        }
    }

    /// Check if actor has any pending actions
    pub fn has_action(&self) -> bool {
        !self.forced_actions.is_empty() || !self.actions.is_empty() || self.preferred_action.is_some()
    }

    /// Check if there are any forced actions pending
    pub fn has_forced_action(&self) -> bool { !self.forced_actions.is_empty() }

    /// Get the category of the next action without consuming it
    pub fn peek_next_action_category(&self) -> Option<ActionCategory> {
        self.peek_next_action().map(|action| action.category())
    }

    /// Clear all actions of a specific category
    pub fn clear_actions_by_category(&mut self, category: ActionCategory) {
        self.actions.retain(|action| action.category() != category);
        self.forced_actions.retain(|action| action.category() != category);

        if let Some(ref preferred) = self.preferred_action
            && preferred.category() == category
        {
            self.preferred_action = None;
        }
    }

    /// Clear all queued actions (preserves forced actions)
    pub fn clear_queued_actions(&mut self) {
        self.actions.clear();
        self.preferred_action = None;
    }

    /// Clear all actions (including forced actions)
    pub fn clear_all_actions(&mut self) {
        self.actions.clear();
        self.preferred_action = None;
        self.forced_actions.clear();
    }

    /// Calculate actual time cost for an action based on speed and modifiers
    pub fn calculate_action_time(&self, base_time: u32) -> u32 {
        let speed_factor = self.speed as f32 / 100.0; // Normalize speed (100 = normal)
        let modified_time = base_time as f32 / (speed_factor * self.action_speed_modifier);
        modified_time.max(1.0) as u32 // Minimum 1 time unit
    }

    /// Apply temporary speed modifier (for buffs/debuffs)
    pub fn set_speed_modifier(&mut self, modifier: f32) {
        self.action_speed_modifier = modifier.max(0.1); // Minimum 10% speed
    }

    /// Reset speed modifier to normal
    pub fn reset_speed_modifier(&mut self) { self.action_speed_modifier = 1.0; }

    /// Get current effective speed (base speed * modifier)
    pub fn effective_speed(&self) -> f32 { self.speed as f32 * self.action_speed_modifier }

    /// Mark actor as dead and clear all actions
    pub fn kill(&mut self) {
        self.alive = false;
        self.clear_all_actions();
    }

    /// Revive actor (for resurrection mechanics)
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

    /// Get number of queued actions (excludes forced and preferred)
    pub fn queued_action_count(&self) -> usize { self.actions.len() }

    /// Get total number of pending actions
    pub fn total_action_count(&self) -> usize {
        self.forced_actions.len() + self.actions.len() + if self.preferred_action.is_some() { 1 } else { 0 }
    }

    /// Check if the action queue is full
    pub fn is_queue_full(&self) -> bool { self.actions.len() >= self.max_queued_actions }

    /// Get remaining queue capacity
    pub fn queue_capacity_remaining(&self) -> usize {
        self.max_queued_actions.saturating_sub(self.actions.len())
    }

    /// Set maximum queue size
    pub fn set_max_queued_actions(&mut self, max: usize) {
        self.max_queued_actions = max;

        // Trim queue if it's now too large
        while self.actions.len() > max {
            self.actions.pop_back();
        }
    }

    /// Check if actor is ready for a turn (has actions and is alive)
    pub fn is_ready_for_turn(&self) -> bool { self.alive && self.has_action() }

    /// Check if actor can perform actions of a specific category
    pub fn can_perform_category(&self, category: ActionCategory) -> bool {
        // This could be extended with status effects, equipment restrictions, etc.
        match category {
            ActionCategory::Movement => self.alive,
            ActionCategory::Attack => self.alive,
            ActionCategory::Spell => self.alive, // Could check for silence, mana, etc.
            ActionCategory::Item => self.alive,
            ActionCategory::Wait => true, // Can always wait
            ActionCategory::Interact => self.alive,
            ActionCategory::Social => self.alive,
            ActionCategory::Craft => self.alive,
        }
    }

    /// Get debug information about the actor's state
    pub fn debug_info(&self) -> String {
        format!(
            "TurnActor {{ alive: {}, speed: {}, modifier: {:.2}, forced: {}, queued: {}, preferred: {} }}",
            self.alive,
            self.speed,
            self.action_speed_modifier,
            self.forced_actions.len(),
            self.actions.len(),
            self.preferred_action.is_some()
        )
    }
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
    pub fn queue_wait(&mut self, entity: Entity, duration: u64) -> Result<(), Box<dyn GameAction>> {
        self.queue_action(Box::new(WaitAction::new(entity, duration)))
    }

    /// Check if the next action is of a specific category
    pub fn next_action_is_category(&self, category: ActionCategory) -> bool {
        self.peek_next_action_category() == Some(category)
    }

    /// Count actions of a specific category
    pub fn count_actions_by_category(&self, category: ActionCategory) -> usize {
        let forced_count = self.forced_actions.iter().filter(|action| action.category() == category).count();

        let queued_count = self.actions.iter().filter(|action| action.category() == category).count();

        let preferred_count = self
            .preferred_action
            .as_ref()
            .map(|action| if action.category() == category { 1 } else { 0 })
            .unwrap_or(0);

        forced_count + queued_count + preferred_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::GameError;

    // Mock action for testing
    #[derive(Clone, Debug)]
    struct MockAction {
        category: ActionCategory,
    }

    impl MockAction {
        pub fn new(category: ActionCategory) -> Self { Self { category } }
    }

    impl GameAction for MockAction {
        fn entity(&self) -> Option<Entity> { None }
        fn perform(&self, _world: &mut World) -> Result<u64, GameError> { Ok(100) }
        fn category(&self) -> ActionCategory { self.category }
    }

    #[test]
    fn test_turn_actor_creation() {
        let actor = TurnActor::new(100);
        assert_eq!(actor.speed(), 100);
        assert!(actor.is_alive());
        assert!(!actor.has_action());
    }

    #[test]
    fn test_action_priority() {
        let mut actor = TurnActor::new(100);

        // Add actions in different priority levels
        // This test would need actual GameAction implementations
        let result = actor.queue_action(Box::new(MockAction::new(ActionCategory::Movement)));
        assert!(result.is_ok());

        actor.set_preferred_action(Box::new(MockAction::new(ActionCategory::Wait)));
        actor.force_action(Box::new(MockAction::new(ActionCategory::Attack)));

        // Forced action should come first
        assert_eq!(actor.peek_next_action_category(), Some(ActionCategory::Attack));
    }

    #[test]
    fn test_queue_limits() {
        let mut actor = TurnActor::with_queue_limit(100, 2);

        // This test would need actual GameAction implementations
        let action1 = Box::new(MockAction::new(ActionCategory::Movement));
        let action2 = Box::new(MockAction::new(ActionCategory::Attack));
        let action3 = Box::new(MockAction::new(ActionCategory::Wait));

        assert!(actor.queue_action(action1).is_ok());
        assert!(actor.queue_action(action2).is_ok());
        assert!(actor.queue_action(action3).is_err()); // Should fail due to limit
    }
}
