use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core character statistics
#[derive(Component, Reflect, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Stats {
    /// Physical strength - affects melee damage and carrying capacity
    pub strength: i32,
    /// Physical defense - reduces incoming physical damage
    pub defense: i32,
    /// Mental intelligence - affects magic damage and mana
    pub intelligence: i32,
    /// Agility/dexterity - affects accuracy, evasion, and initiative
    pub agility: i32,
    /// Constitution/vitality - affects health and stamina
    pub vitality: i32,
    /// Luck - affects critical hits and random events
    pub luck: i32,
}

impl Stats {
    /// Create new stats with specified values
    pub fn new(
        strength: i32,
        defense: i32,
        intelligence: i32,
        agility: i32,
        vitality: i32,
        luck: i32,
    ) -> Self {
        Self { strength, defense, intelligence, agility, vitality, luck }
    }

    /// Create balanced stats with all attributes at the same value
    pub fn balanced(value: i32) -> Self { Self::new(value, value, value, value, value, value) }

    /// Create warrior-focused stats (high strength and defense)
    pub fn warrior() -> Self { Self::new(15, 12, 8, 10, 14, 8) }

    /// Create mage-focused stats (high intelligence and low physical stats)
    pub fn mage() -> Self { Self::new(8, 8, 15, 12, 10, 10) }

    /// Create rogue-focused stats (high agility and luck)
    pub fn rogue() -> Self { Self::new(10, 9, 11, 15, 10, 12) }

    /// Get the total of all stats
    pub fn total(&self) -> i32 {
        self.strength + self.defense + self.intelligence + self.agility + self.vitality + self.luck
    }

    /// Get the average of all stats
    pub fn average(&self) -> f32 { self.total() as f32 / 6.0 }

    /// Calculate melee damage bonus from strength
    pub fn melee_damage_bonus(&self) -> i32 { (self.strength - 10).max(0) }

    /// Calculate magic damage bonus from intelligence
    pub fn magic_damage_bonus(&self) -> i32 { (self.intelligence - 10).max(0) }

    /// Calculate damage reduction from defense
    pub fn damage_reduction(&self) -> i32 { (self.defense - 10).max(0) }

    /// Calculate accuracy bonus from agility
    pub fn accuracy_bonus(&self) -> i32 { (self.agility - 10).max(0) }

    /// Calculate evasion bonus from agility
    pub fn evasion_bonus(&self) -> i32 { (self.agility - 10).max(0) }

    /// Calculate health bonus from vitality
    pub fn health_bonus(&self) -> i32 { (self.vitality - 10) * 5 }

    /// Calculate critical hit chance from luck (as percentage)
    pub fn critical_chance(&self) -> f32 {
        (self.luck as f32 * 0.5).max(0.0).min(25.0) // Cap at 25 %
    }

    /// Calculate initiative bonus for turn order
    pub fn initiative_bonus(&self) -> i32 { self.agility + (self.luck / 2) }

    /// Modify a specific stat by a given amount
    pub fn modify_stat(&mut self, stat: StatType, amount: i32) {
        match stat {
            StatType::Strength => self.strength = self.strength.saturating_add(amount),
            StatType::Defense => self.defense = self.defense.saturating_add(amount),
            StatType::Intelligence => self.intelligence = self.intelligence.saturating_add(amount),
            StatType::Agility => self.agility = self.agility.saturating_add(amount),
            StatType::Vitality => self.vitality = self.vitality.saturating_add(amount),
            StatType::Luck => self.luck = self.luck.saturating_add(amount),
        }
    }

    /// Get a stat value by type
    pub fn get_stat(&self, stat: StatType) -> i32 {
        match stat {
            StatType::Strength => self.strength,
            StatType::Defense => self.defense,
            StatType::Intelligence => self.intelligence,
            StatType::Agility => self.agility,
            StatType::Vitality => self.vitality,
            StatType::Luck => self.luck,
        }
    }

    /// Set a stat value by type
    pub fn set_stat(&mut self, stat: StatType, value: i32) {
        match stat {
            StatType::Strength => self.strength = value,
            StatType::Defense => self.defense = value,
            StatType::Intelligence => self.intelligence = value,
            StatType::Agility => self.agility = value,
            StatType::Vitality => self.vitality = value,
            StatType::Luck => self.luck = value,
        }
    }

    /// Apply temporary stat modifiers (returns modified stats without changing original)
    pub fn with_modifiers(&self, modifiers: &StatModifiers) -> Self {
        let mut modified = self.clone();
        for (stat_type, modifier) in &modifiers.modifiers {
            modified.modify_stat(*stat_type, *modifier);
        }
        modified
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::balanced(10) // Default to 10 in all stats
    }
}

/// Enumeration of different stat types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum StatType {
    Strength,
    Defense,
    Intelligence,
    Agility,
    Vitality,
    Luck,
}

impl StatType {
    /// Get all stat types as an array
    pub fn all() -> [StatType; 6] {
        [
            StatType::Strength,
            StatType::Defense,
            StatType::Intelligence,
            StatType::Agility,
            StatType::Vitality,
            StatType::Luck,
        ]
    }

    /// Get the display name for the stat
    pub fn display_name(&self) -> &'static str {
        match self {
            StatType::Strength => "Strength",
            StatType::Defense => "Defense",
            StatType::Intelligence => "Intelligence",
            StatType::Agility => "Agility",
            StatType::Vitality => "Vitality",
            StatType::Luck => "Luck",
        }
    }

    /// Get a short abbreviation for the stat
    pub fn abbreviation(&self) -> &'static str {
        match self {
            StatType::Strength => "STR",
            StatType::Defense => "DEF",
            StatType::Intelligence => "INT",
            StatType::Agility => "AGI",
            StatType::Vitality => "VIT",
            StatType::Luck => "LUK",
        }
    }
}

/// Component for temporary stat modifications (buffs/debuffs)
#[derive(Component, Reflect, Debug, Clone, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct StatModifiers {
    /// Map of stat type to modifier value
    pub modifiers: HashMap<StatType, i32>,
}

impl StatModifiers {
    /// Create new empty stat modifiers
    pub fn new() -> Self { Self { modifiers: HashMap::new() } }

    /// Add a modifier for a specific stat
    pub fn add_modifier(&mut self, stat: StatType, value: i32) {
        *self.modifiers.entry(stat).or_insert(0) += value;
    }

    /// Remove a modifier for a specific stat
    pub fn remove_modifier(&mut self, stat: StatType) { self.modifiers.remove(&stat); }

    /// Clear all modifiers
    pub fn clear(&mut self) { self.modifiers.clear(); }

    /// Check if there are any active modifiers
    pub fn is_empty(&self) -> bool { self.modifiers.is_empty() }

    /// Get the modifier value for a specific stat
    pub fn get_modifier(&self, stat: StatType) -> i32 { self.modifiers.get(&stat).copied().unwrap_or(0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_creation() {
        let stats = Stats::new(15, 12, 10, 14, 13, 8);
        assert_eq!(stats.strength, 15);
        assert_eq!(stats.defense, 12);
        assert_eq!(stats.intelligence, 10);
        assert_eq!(stats.agility, 14);
        assert_eq!(stats.vitality, 13);
        assert_eq!(stats.luck, 8);
    }

    #[test]
    fn test_balanced_stats() {
        let stats = Stats::balanced(12);
        assert_eq!(stats.total(), 72);
        assert_eq!(stats.average(), 12.0);
    }

    #[test]
    fn test_preset_builds() {
        let warrior = Stats::warrior();
        assert!(warrior.strength >= warrior.intelligence);
        assert!(warrior.defense >= warrior.intelligence);

        let mage = Stats::mage();
        assert!(mage.intelligence >= mage.strength);
        assert!(mage.intelligence >= mage.defense);

        let rogue = Stats::rogue();
        assert!(rogue.agility >= rogue.strength);
        assert!(rogue.luck >= rogue.defense);
    }

    #[test]
    fn test_damage_calculations() {
        let stats = Stats::new(15, 8, 12, 10, 14, 6);
        assert_eq!(stats.melee_damage_bonus(), 5); // 15 - 10
        assert_eq!(stats.magic_damage_bonus(), 2); // 12 - 10
        assert_eq!(stats.damage_reduction(), 0); // 8 - 10, but max(0)
    }

    #[test]
    fn test_stat_modification() {
        let mut stats = Stats::balanced(10);
        stats.modify_stat(StatType::Strength, 5);
        assert_eq!(stats.strength, 15);

        stats.set_stat(StatType::Defense, 20);
        assert_eq!(stats.defense, 20);
    }

    #[test]
    fn test_stat_modifiers() {
        let base_stats = Stats::balanced(10);
        let mut modifiers = StatModifiers::new();
        modifiers.add_modifier(StatType::Strength, 5);
        modifiers.add_modifier(StatType::Agility, -2);

        let modified_stats = base_stats.with_modifiers(&modifiers);
        assert_eq!(modified_stats.strength, 15);
        assert_eq!(modified_stats.agility, 8);
        assert_eq!(modified_stats.defense, 10); // Unchanged
    }

    #[test]
    fn test_critical_chance() {
        let stats = Stats::new(10, 10, 10, 10, 10, 20);
        assert_eq!(stats.critical_chance(), 10.0); // 20 * 0.5

        let high_luck_stats = Stats::new(10, 10, 10, 10, 10, 60);
        assert_eq!(high_luck_stats.critical_chance(), 25.0); // Capped at 25%
    }

    #[test]
    fn test_health_bonus() {
        let stats = Stats::new(10, 10, 10, 10, 15, 10);
        assert_eq!(stats.health_bonus(), 25); // (15 - 10) * 5

        let low_vitality_stats = Stats::new(10, 10, 10, 10, 8, 10);
        assert_eq!(low_vitality_stats.health_bonus(), -10); // (8 - 10) * 5
    }

    #[test]
    fn test_stat_type_utilities() {
        assert_eq!(StatType::Strength.display_name(), "Strength");
        assert_eq!(StatType::Agility.abbreviation(), "AGI");
        assert_eq!(StatType::all().len(), 6);
    }
}
