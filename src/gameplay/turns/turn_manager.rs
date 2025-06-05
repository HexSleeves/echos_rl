use bevy::prelude::*;
use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    core::{components::DeadTag, types::GameError},
    gameplay::turns::components::TurnActor,
};

/// Time units per turn (adjust based on your game's needs)
const TURN_TIME: u32 = 100;

/// Resource that manages the turn-based queue and timing
#[derive(Resource, Default)]
pub struct TurnManager {
    /// Current turn number
    turn_number: u32,
    /// Current time within the turn (0 to TURN_TIME-1)
    current_time: u32,
    /// Priority queue of (turn, time, entity) ordered by turn/time
    turn_queue: BinaryHeap<Reverse<(u32, u32, Entity)>>,

    /// Cleanup tracking to prevent performance issues
    operations_since_cleanup: u32,
    cleanup_threshold: u32,

    /// Metrics for monitoring and debugging
    total_cleanups: u64,
    total_entities_removed: u64,
    total_turns_processed: u64,

    /// Performance tracking
    last_cleanup_duration: std::time::Duration,

    /// Configuration
    max_queue_size: usize,
}

impl TurnManager {
    /// Create a new TurnManager with default settings
    pub fn new() -> Self {
        Self {
            turn_number: 0,
            current_time: 0,
            total_cleanups: 0,
            max_queue_size: 10000,
            cleanup_threshold: 100,
            total_turns_processed: 0,
            total_entities_removed: 0,
            operations_since_cleanup: 0,
            turn_queue: BinaryHeap::new(),
            last_cleanup_duration: std::time::Duration::ZERO,
        }
    }

    /// Create a TurnManager with custom settings
    pub fn with_config(cleanup_threshold: u32, max_queue_size: usize) -> Self {
        Self { cleanup_threshold, max_queue_size, ..Self::new() }
    }

    /// Add an entity to the turn queue at the current turn/time
    pub fn add_entity(&mut self, entity: Entity) -> Result<(), GameError> {
        if self.turn_queue.len() >= self.max_queue_size {
            return Err(GameError::QueueFull);
        }

        self.turn_queue.push(Reverse((self.turn_number, self.current_time, entity)));
        debug!(
            "Added entity {:?} to turn queue at turn {}, time {}",
            entity, self.turn_number, self.current_time
        );
        Ok(())
    }

    /// Remove a specific entity from the turn queue (O(n) operation)
    pub fn remove_entity(&mut self, target_entity: Entity) -> bool {
        let original_size = self.turn_queue.len();
        let mut temp_queue = BinaryHeap::with_capacity(original_size);
        let mut found = false;

        while let Some(entry @ Reverse((turn, time, entity))) = self.turn_queue.pop() {
            if entity == target_entity && !found {
                found = true;
                debug!("Removed entity {:?} from turn queue (was at turn {}, time {})", entity, turn, time);
                continue;
            }
            temp_queue.push(entry);
        }

        self.turn_queue = temp_queue;
        found
    }

    /// Clear all entities from the queue, optionally reset turn counter
    pub fn clear_entities(&mut self, reset_turn_number: bool) {
        let entities_cleared = self.turn_queue.len();

        if reset_turn_number {
            self.turn_number = 0;
            info!("Reset turn number to 0");
        }

        self.current_time = 0;
        self.turn_queue.clear();

        info!("Cleared {} entities from turn queue", entities_cleared);
    }

    /// Get the next entity ready for their turn
    pub fn start_entity_turn(&mut self) -> Option<Entity> {
        if let Some(Reverse((turn_number, current_time, entity))) = self.turn_queue.pop() {
            // Update current game time to this entity's turn time
            self.turn_number = turn_number;
            self.current_time = current_time;
            self.total_turns_processed += 1;

            debug!("Starting turn for entity {:?} at turn {}, time {}", entity, turn_number, current_time);

            Some(entity)
        } else {
            None
        }
    }

    /// Schedule an entity's next turn after they spend time performing an action
    pub fn end_entity_turn(&mut self, entity: Entity, time_spent: u32) -> Result<(), GameError> {
        if self.turn_queue.len() >= self.max_queue_size {
            return Err(GameError::QueueFull);
        }

        // Handle zero time (immediate re-queue)
        if time_spent == 0 {
            self.turn_queue.push(Reverse((self.turn_number, self.current_time, entity)));
            debug!("Re-queued entity {:?} immediately", entity);
            return Ok(());
        }

        let (next_turn, next_time) = self.calculate_next_turn_time(time_spent);

        // Find the correct position in the queue and insert
        self.turn_queue.push(Reverse((next_turn, next_time, entity)));

        debug!(
            "Scheduled entity {:?} for turn {}, time {} (spent {} time)",
            entity, next_turn, next_time, time_spent
        );

        Ok(())
    }

    /// Schedule an entity for a specific turn and time
    pub fn schedule_turn(&mut self, entity: Entity, turn: u32, time: u32) -> Result<(), GameError> {
        if self.turn_queue.len() >= self.max_queue_size {
            return Err(GameError::QueueFull);
        }

        let normalized_time = time % TURN_TIME;
        let actual_turn = turn + (time / TURN_TIME);

        self.turn_queue.push(Reverse((actual_turn, normalized_time, entity)));

        debug!("Scheduled entity {:?} for specific turn {}, time {}", entity, actual_turn, normalized_time);

        Ok(())
    }

    /// Schedule an entity for immediate action (current turn/time)
    pub fn schedule_now(&mut self, entity: Entity) -> Result<(), GameError> {
        self.schedule_turn(entity, self.turn_number, self.current_time)
    }

    /// Calculate the next turn and time given time spent
    fn calculate_next_turn_time(&self, time_spent: u32) -> (u32, u32) {
        let mut next_turn = self.turn_number;
        let mut next_time = self.current_time + time_spent;

        // Handle turn overflow
        while next_time >= TURN_TIME {
            next_turn += 1;
            next_time -= TURN_TIME;
        }

        (next_turn, next_time)
    }

    /// Check if the turn queue is empty
    pub fn is_empty(&self) -> bool { self.turn_queue.is_empty() }

    /// Get the current turn number
    pub fn current_turn(&self) -> u32 { self.turn_number }

    /// Get the current time within the turn
    pub fn current_time(&self) -> u32 { self.current_time }

    /// Get the total number of actions in the queue
    pub fn total_action_count(&self) -> usize { self.turn_queue.len() }

    /// Peek at the next entity without removing them from the queue
    pub fn peek_next(&self) -> Option<(u32, u32, Entity)> {
        self.turn_queue.peek().map(|Reverse((turn, time, entity))| (*turn, *time, *entity))
    }

    /// Check if a specific entity is scheduled in the queue
    pub fn is_scheduled(&self, target_entity: Entity) -> bool {
        self.turn_queue.iter().any(|Reverse((_, _, entity))| *entity == target_entity)
    }

    /// Get the time until a specific turn/time
    pub fn time_until(&self, target_turn: u32, target_time: u32) -> u32 {
        if target_turn < self.turn_number
            || (target_turn == self.turn_number && target_time < self.current_time)
        {
            return 0; // Time has already passed
        }

        let turn_diff = target_turn - self.turn_number;
        

        if target_turn == self.turn_number {
            target_time - self.current_time
        } else {
            (TURN_TIME - self.current_time) + target_time + ((turn_diff - 1) * TURN_TIME)
        }
    }

    /// Get entities scheduled for a specific turn
    pub fn get_entities_for_turn(&self, target_turn: u32) -> Vec<(u32, Entity)> {
        self.turn_queue
            .iter()
            .filter_map(
                |Reverse((turn, time, entity))| {
                    if *turn == target_turn { Some((*time, *entity)) } else { None }
                },
            )
            .collect()
    }

    /// Get the next N entities in the queue without removing them
    pub fn peek_next_entities(&self, count: usize) -> Vec<(u32, u32, Entity)> {
        let mut entities = Vec::with_capacity(count);
        let mut temp_queue = self.turn_queue.clone();

        for _ in 0..count {
            if let Some(Reverse((turn, time, entity))) = temp_queue.pop() {
                entities.push((turn, time, entity));
            } else {
                break;
            }
        }

        entities
    }

    /// Force advance to a specific turn/time (useful for testing or time skips)
    pub fn advance_to(&mut self, target_turn: u32, target_time: u32) {
        let normalized_time = target_time % TURN_TIME;
        let actual_turn = target_turn + (target_time / TURN_TIME);

        if actual_turn > self.turn_number
            || (actual_turn == self.turn_number && normalized_time > self.current_time)
        {
            self.turn_number = actual_turn;
            self.current_time = normalized_time;

            info!("Advanced time to turn {}, time {}", self.turn_number, self.current_time);
        }
    }

    /// Clean up dead/invalid entities with adaptive frequency
    pub fn cleanup_dead_entities(&mut self, world: &World) -> CleanupMetrics {
        // Check if cleanup is needed
        if self.operations_since_cleanup < self.get_cleanup_threshold(world) {
            self.operations_since_cleanup += 1;
            return CleanupMetrics::default();
        }

        let start_time = std::time::Instant::now();
        let queue_size_before = self.turn_queue.len();

        debug!("Starting turn queue cleanup...");

        // Create new queue with only valid entities
        let mut new_queue = BinaryHeap::with_capacity(self.turn_queue.len());
        let mut removed_count = 0;

        while let Some(entry @ Reverse((turn, time, entity))) = self.turn_queue.pop() {
            if self.is_valid_turn_actor(world, entity) {
                new_queue.push(entry);
            } else {
                removed_count += 1;
                debug!("Removed invalid entity {:?} from turn queue", entity);
            }
        }

        // Replace the queue
        self.turn_queue = new_queue;

        // Update metrics
        let processing_time = start_time.elapsed();
        self.operations_since_cleanup = 0;
        self.total_cleanups += 1;
        self.total_entities_removed += removed_count as u64;
        self.last_cleanup_duration = processing_time;

        let metrics = CleanupMetrics {
            entities_removed: removed_count,
            queue_size_before,
            queue_size_after: self.turn_queue.len(),
            processing_time,
        };

        if removed_count > 0 {
            info!("Cleanup completed: removed {} entities in {:?}", removed_count, processing_time);
        }

        metrics
    }

    /// Determine cleanup frequency based on game state
    fn get_cleanup_threshold(&self, world: &World) -> u32 {
        let entity_count = world.entities().len();
        let queue_size = self.turn_queue.len();

        // More frequent cleanup with larger entity counts or queue sizes
        if entity_count > 1000 || queue_size > 500 {
            self.cleanup_threshold / 2
        } else if entity_count < 100 && queue_size < 50 {
            self.cleanup_threshold * 2
        } else {
            self.cleanup_threshold
        }
    }

    /// Check if an entity is valid for the turn queue
    fn is_valid_turn_actor(&self, world: &World, entity: Entity) -> bool {
        // Check if entity exists
        if !world.entities().contains(entity) {
            return false;
        }

        let entity_ref = world.entity(entity);

        // Check if it has required TurnActor component
        if !entity_ref.contains::<TurnActor>() {
            return false;
        }

        // Check for dead tag
        if entity_ref.contains::<DeadTag>() {
            return false;
        }

        // Check if TurnActor is alive
        if let Some(actor) = entity_ref.get::<TurnActor>()
            && !actor.is_alive() {
                return false;
            }

        true
    }

    /// Get comprehensive statistics about the turn manager
    pub fn get_statistics(&self) -> TurnManagerStats {
        TurnManagerStats {
            current_turn: self.turn_number,
            current_time: self.current_time,
            queue_size: self.turn_queue.len(),
            total_cleanups: self.total_cleanups,
            total_entities_removed: self.total_entities_removed,
            total_turns_processed: self.total_turns_processed,
            last_cleanup_duration: self.last_cleanup_duration,
            operations_since_cleanup: self.operations_since_cleanup,
            cleanup_threshold: self.cleanup_threshold,
        }
    }

    /// Set the cleanup threshold
    pub fn set_cleanup_threshold(&mut self, threshold: u32) { self.cleanup_threshold = threshold; }

    /// Set the maximum queue size
    pub fn set_max_queue_size(&mut self, max_size: usize) { self.max_queue_size = max_size; }

    /// Get current queue utilization as a percentage
    pub fn queue_utilization(&self) -> f32 {
        (self.turn_queue.len() as f32 / self.max_queue_size as f32) * 100.0
    }

    /// Check if the queue is near capacity
    pub fn is_queue_near_capacity(&self, threshold_percent: f32) -> bool {
        self.queue_utilization() > threshold_percent
    }

    /// Validate queue integrity (for debugging)
    pub fn validate_queue_integrity(&self) -> QueueValidationResult {
        let mut issues = Vec::new();
        let mut last_turn = 0;
        let mut last_time = 0;
        let mut is_first = true;

        // Check if queue is properly sorted
        for Reverse((turn, time, entity)) in &self.turn_queue {
            if !is_first
                && (*turn < last_turn || (*turn == last_turn && *time < last_time)) {
                    issues.push(format!(
                        "Queue order violation: ({turn}, {time}, {entity:?}) comes after ({last_turn}, {last_time}, _)"
                    ));
                }

            last_turn = *turn;
            last_time = *time;
            is_first = false;
        }

        // Check for time values >= TURN_TIME
        for Reverse((turn, time, entity)) in &self.turn_queue {
            if *time >= TURN_TIME {
                issues.push(format!(
                    "Invalid time value: entity {entity:?} has time {time} >= TURN_TIME ({TURN_TIME})"
                ));
            }
        }

        QueueValidationResult { is_valid: issues.is_empty(), issues, queue_size: self.turn_queue.len() }
    }

    /// Print debug information about the queue state
    pub fn print_debug_info(&self) {
        info!("=== Turn Manager Debug Info ===");
        info!("Current Turn: {}, Current Time: {}", self.turn_number, self.current_time);
        info!("Queue Size: {}/{}", self.turn_queue.len(), self.max_queue_size);
        info!("Queue Utilization: {:.1}%", self.queue_utilization());
        info!("Total Turns Processed: {}", self.total_turns_processed);
        info!("Total Cleanups: {}", self.total_cleanups);
        info!("Total Entities Removed: {}", self.total_entities_removed);

        // Show next few entities
        let next_entities = self.peek_next_entities(5);
        info!("Next {} entities:", next_entities.len());
        for (i, (turn, time, entity)) in next_entities.iter().enumerate() {
            info!("  {}: Turn {}, Time {}, Entity {:?}", i + 1, turn, time, entity);
        }
    }
}

/// Metrics returned by cleanup operations
#[derive(Default, Debug, Clone)]
pub struct CleanupMetrics {
    pub entities_removed: usize,
    pub queue_size_before: usize,
    pub queue_size_after: usize,
    pub processing_time: std::time::Duration,
}

/// Comprehensive statistics about the turn manager
#[derive(Debug, Clone)]
pub struct TurnManagerStats {
    pub current_turn: u32,
    pub current_time: u32,
    pub queue_size: usize,
    pub total_cleanups: u64,
    pub total_entities_removed: u64,
    pub total_turns_processed: u64,
    pub last_cleanup_duration: std::time::Duration,
    pub operations_since_cleanup: u32,
    pub cleanup_threshold: u32,
}

/// Result of queue validation
#[derive(Debug)]
pub struct QueueValidationResult {
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub queue_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_manager_creation() {
        let tm = TurnManager::new();
        assert_eq!(tm.current_turn(), 0);
        assert_eq!(tm.current_time(), 0);
        assert!(tm.is_empty());
    }

    #[test]
    fn test_time_calculation() {
        let tm = TurnManager::new();

        // Test normal time progression
        let (turn, time) = tm.calculate_next_turn_time(50);
        assert_eq!(turn, 0);
        assert_eq!(time, 50);

        // Test turn overflow
        let (turn, time) = tm.calculate_next_turn_time(150);
        assert_eq!(turn, 1);
        assert_eq!(time, 50);

        // Test multiple turn overflow
        let (turn, time) = tm.calculate_next_turn_time(250);
        assert_eq!(turn, 2);
        assert_eq!(time, 50);
    }

    #[test]
    fn test_time_until() {
        let tm = TurnManager::new();

        // Same turn, future time
        assert_eq!(tm.time_until(0, 50), 50);

        // Next turn, same time
        assert_eq!(tm.time_until(1, 0), TURN_TIME);

        // Next turn, future time
        assert_eq!(tm.time_until(1, 50), TURN_TIME + 50);

        // Past time should return 0
        assert_eq!(tm.time_until(0, 0), 0);
    }

    #[test]
    fn test_queue_operations() {
        let mut tm = TurnManager::new();
        let entity1 = Entity::from_raw(1);
        let entity2 = Entity::from_raw(2);

        // Add entities
        assert!(tm.add_entity(entity1).is_ok());
        assert!(tm.add_entity(entity2).is_ok());
        assert_eq!(tm.total_action_count(), 2);

        // Remove entity
        assert!(tm.remove_entity(entity1));
        assert_eq!(tm.total_action_count(), 1);

        // Remove non-existent entity
        assert!(!tm.remove_entity(entity1));
        assert_eq!(tm.total_action_count(), 1);
    }

    #[test]
    fn test_queue_validation() {
        let tm = TurnManager::new();
        let result = tm.validate_queue_integrity();
        assert!(result.is_valid);
        assert!(result.issues.is_empty());
    }
}
