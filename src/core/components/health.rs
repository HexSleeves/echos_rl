use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Health component for entities that can take damage or be healed
#[derive(Component, Reflect, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Health {
    /// Current health points
    pub current: i32,
    /// Maximum health points
    pub max: i32,
}

impl Health {
    /// Create a new Health component with the specified max health
    pub fn new(max_health: i32) -> Self { Self { current: max_health, max: max_health } }

    /// Create a new Health component with current and max health
    pub fn new_with_current(current: i32, max: i32) -> Self { Self { current: current.min(max), max } }

    /// Check if the entity is alive (health > 0)
    pub fn is_alive(&self) -> bool { self.current > 0 }

    /// Check if the entity is dead (health <= 0)
    pub fn is_dead(&self) -> bool { self.current <= 0 }

    /// Check if the entity is at full health
    pub fn is_full(&self) -> bool { self.current >= self.max }

    /// Get the current health percentage (0.0 to 1.0)
    pub fn percentage(&self) -> f32 {
        if self.max <= 0 { 0.0 } else { (self.current as f32 / self.max as f32).clamp(0.0, 1.0) }
    }

    /// Get the missing health points
    pub fn missing(&self) -> i32 { (self.max - self.current).max(0) }

    /// Apply damage to the entity
    /// Returns the actual damage dealt (may be less than requested if health goes below 0)
    pub fn take_damage(&mut self, damage: i32) -> i32 {
        let old_health = self.current;
        self.current = (self.current - damage).max(0);
        old_health - self.current
    }

    /// Heal the entity
    /// Returns the actual healing done (may be less than requested if at max health)
    pub fn heal(&mut self, healing: i32) -> i32 {
        let old_health = self.current;
        self.current = (self.current + healing).min(self.max);
        self.current - old_health
    }

    /// Set current health directly (clamped to valid range)
    pub fn set_current(&mut self, health: i32) { self.current = health.clamp(0, self.max); }

    /// Set max health and adjust current if necessary
    pub fn set_max(&mut self, max_health: i32) {
        self.max = max_health.max(1); // Ensure max health is at least 1
        self.current = self.current.min(self.max);
    }

    /// Increase max health and optionally heal to the new max
    pub fn increase_max(&mut self, increase: i32, heal_to_new_max: bool) {
        self.max += increase;
        if heal_to_new_max {
            self.current = self.max;
        }
    }

    /// Fully heal the entity to max health
    pub fn full_heal(&mut self) { self.current = self.max; }

    /// Kill the entity (set health to 0)
    pub fn kill(&mut self) { self.current = 0; }

    /// Revive the entity with specified health (defaults to max if not specified)
    pub fn revive(&mut self, health: Option<i32>) {
        self.current = health.unwrap_or(self.max).clamp(1, self.max);
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::new(100) // Default to 100 max health
    }
}

impl From<i32> for Health {
    fn from(max_health: i32) -> Self { Self::new(max_health) }
}

impl From<(i32, i32)> for Health {
    fn from((current, max): (i32, i32)) -> Self { Self::new_with_current(current, max) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_creation() {
        let health = Health::new(100);
        assert_eq!(health.current, 100);
        assert_eq!(health.max, 100);
        assert!(health.is_alive());
        assert!(!health.is_dead());
        assert!(health.is_full());
    }

    #[test]
    fn test_health_with_current() {
        let health = Health::new_with_current(50, 100);
        assert_eq!(health.current, 50);
        assert_eq!(health.max, 100);
        assert_eq!(health.percentage(), 0.5);
        assert_eq!(health.missing(), 50);
    }

    #[test]
    fn test_damage() {
        let mut health = Health::new(100);
        let damage_dealt = health.take_damage(30);
        assert_eq!(damage_dealt, 30);
        assert_eq!(health.current, 70);
        assert!(health.is_alive());
    }

    #[test]
    fn test_overkill_damage() {
        let mut health = Health::new(100);
        let damage_dealt = health.take_damage(150);
        assert_eq!(damage_dealt, 100); // Only dealt 100 damage, not 150
        assert_eq!(health.current, 0);
        assert!(health.is_dead());
    }

    #[test]
    fn test_healing() {
        let mut health = Health::new_with_current(50, 100);
        let healing_done = health.heal(30);
        assert_eq!(healing_done, 30);
        assert_eq!(health.current, 80);
    }

    #[test]
    fn test_overheal() {
        let mut health = Health::new_with_current(90, 100);
        let healing_done = health.heal(20);
        assert_eq!(healing_done, 10); // Only healed 10, not 20
        assert_eq!(health.current, 100);
        assert!(health.is_full());
    }

    #[test]
    fn test_max_health_changes() {
        let mut health = Health::new(100);
        health.set_max(150);
        assert_eq!(health.max, 150);
        assert_eq!(health.current, 100);

        health.increase_max(50, true);
        assert_eq!(health.max, 200);
        assert_eq!(health.current, 200);
    }

    #[test]
    fn test_revive() {
        let mut health = Health::new(100);
        health.kill();
        assert!(health.is_dead());

        health.revive(Some(50));
        assert_eq!(health.current, 50);
        assert!(health.is_alive());

        health.kill();
        health.revive(None);
        assert_eq!(health.current, 100);
    }

    #[test]
    fn test_percentage_calculation() {
        let health = Health::new_with_current(25, 100);
        assert_eq!(health.percentage(), 0.25);

        let health = Health::new_with_current(0, 100);
        assert_eq!(health.percentage(), 0.0);

        let health = Health::new_with_current(100, 100);
        assert_eq!(health.percentage(), 1.0);
    }
}
