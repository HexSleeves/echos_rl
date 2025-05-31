use std::{cmp::Reverse, collections::BinaryHeap};

use bevy::prelude::*;

use crate::{
    core::components::DeadTag,
    gameplay::turns::components::TurnActor,
};

/// Resource that manages the turn-based queue system
#[derive(Resource, Default)]
pub struct TurnQueue {
    current_time: u64,
    turn_queue: BinaryHeap<Reverse<(u64, Entity)>>,

    operations_since_cleanup: u32,
    // Optional cleanup metrics
    total_cleanups: u64,
    total_entities_removed: u64,
}

impl TurnQueue {
    pub fn print_queue(&self) {
        info!("Current time: {:?}, Turn queue: {:?}", self.current_time, self.turn_queue);
    }

    /// Check if the turn queue is empty
    pub fn is_empty(&self) -> bool {
        self.turn_queue.is_empty()
    }

    /// Add actor to the queue with wrapping time calculation
    pub fn schedule_turn(&mut self, entity: Entity, next_time: u64) {
        self.turn_queue.push(Reverse((next_time, entity)));
    }

    /// Get the next actor in the queue
    pub fn get_next_actor(&mut self) -> Option<(Entity, u64)> {
        if let Some(Reverse((time, entity))) = self.turn_queue.pop() {
            // Update current time with wrapping protection
            self.current_time = time;
            Some((entity, time))
        } else {
            None
        }
    }

    // Get current time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    // Peek at next actor without removing
    pub fn peek_next(&self) -> Option<(Entity, u64)> {
        self.turn_queue.peek().map(|Reverse((time, entity))| (*entity, *time))
    }

    // Check if an entity's turn is scheduled
    pub fn is_scheduled(&self, entity: Entity) -> bool {
        self.turn_queue.iter().any(|Reverse((_, e))| *e == entity)
    }

    // Properly handle time comparison with wrapping
    pub fn time_until(&self, time: u64) -> u64 {
        time.wrapping_sub(self.current_time)
    }

    // Compare two times accounting for wrapping
    pub fn is_before(&self, time_a: u64, time_b: u64) -> bool {
        // This handles wrapping correctly
        self.time_until(time_a) < self.time_until(time_b)
    }

    /// Clean up dead entities from the turn queue
    pub fn cleanup_dead_entities(&mut self, world: &World) -> CleanupMetrics {
        // Only run periodically to amortize cost
        if self.operations_since_cleanup < self.get_cleanup_threshold(world) {
            self.operations_since_cleanup += 1;
            return CleanupMetrics::default();
        }

        info!("Cleaning up dead entities...");

        let queue_size_before = self.turn_queue.len();
        let start_time = std::time::Instant::now();

        // Create a temporary vector to avoid modifying during iteration
        let mut new_queue = BinaryHeap::with_capacity(self.turn_queue.len());
        let mut removed_count = 0;

        // Process each entity in the turn queue
        while let Some(Reverse((time, entity))) = self.turn_queue.pop() {
            let entity_valid = self.is_valid_turn_actor(world, entity);

            if entity_valid {
                // Keep valid entities
                new_queue.push(Reverse((time, entity)));
            } else {
                // Count removed entities
                removed_count += 1;

                if let Some(entity_name) = get_entity_debug_name(world, entity) {
                    log::debug!("Removed dead entity from turn queue: {entity_name}");
                } else {
                    log::debug!("Removed dead entity from turn queue: {entity:?}");
                }
            }
        }

        // Replace the old queue
        self.turn_queue = new_queue;
        self.operations_since_cleanup = 0;
        self.total_cleanups += 1;
        self.total_entities_removed += removed_count as u64;

        let processing_time = start_time.elapsed();

        CleanupMetrics {
            entities_removed: removed_count,
            queue_size_before,
            queue_size_after: self.turn_queue.len(),
            processing_time,
        }
    }

    // Dynamically determine cleanup frequency based on game state
    fn get_cleanup_threshold(&self, world: &World) -> u32 {
        // Base threshold
        let base_threshold = 100;

        // Adjust based on entity count and queue size
        let entity_count = world.entities().len();
        let queue_size = self.turn_queue.len();

        // More frequent cleanup with larger entity counts or queue sizes
        if entity_count > 1000 || queue_size > 500 {
            return base_threshold / 2;
        } else if entity_count < 100 && queue_size < 50 {
            return base_threshold * 2;
        }

        base_threshold
    }

    // Check if an entity is valid for the turn queue
    fn is_valid_turn_actor(&self, world: &World, entity: Entity) -> bool {
        // First, check if entity exists at all
        if !world.entities().contains(entity) {
            return false;
        }

        // Check if it has required TurnActor component
        if !world.entity(entity).contains::<TurnActor>() {
            return false;
        }

        // Check for "dead" markers (game-specific logic)
        if world.entity(entity).contains::<DeadTag>() {
            return false;
        }

        true
    }
}

// Helper function to get entity name for debugging
fn get_entity_debug_name(world: &World, entity: Entity) -> Option<String> {
    world.entity(entity).get::<Name>().map(|name| name.as_str().to_owned())
}

// Metrics struct for monitoring cleanup performance
#[derive(Default, Debug)]
pub struct CleanupMetrics {
    pub entities_removed: usize,
    pub queue_size_before: usize,
    pub queue_size_after: usize,
    pub processing_time: std::time::Duration,
}
