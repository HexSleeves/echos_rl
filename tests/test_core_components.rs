use bevy::prelude::*;
use echos_in_the_dark::core::components::*;

#[test]
fn test_base_entity_bundle() {
    let position = Position::new(5, 10);
    let bundle = BaseEntityBundle::new(position);

    assert_eq!(bundle.position, position);
    assert_eq!(bundle.health.current, 100); // Default health
    assert_eq!(bundle.health.max, 100);
    assert_eq!(bundle.stats.total(), 60); // Default balanced stats (10 * 6)
}

#[test]
fn test_base_entity_bundle_with_custom_stats() {
    let position = Position::new(3, 7);
    let health = Health::new(150);
    let stats = Stats::warrior();

    let bundle = BaseEntityBundle::new_with_stats(position, health, stats.clone());

    assert_eq!(bundle.position, position);
    assert_eq!(bundle.health.max, 150);
    assert_eq!(bundle.stats, stats);
}

#[test]
fn test_player_bundle() {
    let position = Position::new(0, 0);
    let bundle = PlayerBundle::new(position);

    assert_eq!(bundle.base.position, position);
    assert_eq!(bundle.base.health.max, 100);
    assert_eq!(bundle.base.stats.total(), 72); // Balanced 12 * 6
    assert_eq!(bundle.inventory.max_slots, 30);
    assert_eq!(bundle.inventory.max_weight, 150.0);
    assert!(bundle.inventory.is_empty());
}

#[test]
fn test_player_bundle_with_custom_stats() {
    let position = Position::new(1, 1);
    let health = Health::new(120);
    let stats = Stats::mage();

    let bundle = PlayerBundle::new_with_stats(position, health, stats.clone());

    assert_eq!(bundle.base.position, position);
    assert_eq!(bundle.base.health.max, 120);
    assert_eq!(bundle.base.stats, stats);
    assert_eq!(bundle.inventory.max_slots, 30);
}

#[test]
fn test_enemy_bundle() {
    let position = Position::new(10, 15);
    let bundle = EnemyBundle::new(position);

    assert_eq!(bundle.base.position, position);
    assert_eq!(bundle.base.health.current, 100); // Default health
    assert_eq!(bundle.base.stats.total(), 60); // Default balanced stats
}

#[test]
fn test_enemy_bundle_presets() {
    let position = Position::new(5, 5);

    // Test warrior enemy
    let warrior = EnemyBundle::warrior(position);
    assert_eq!(warrior.base.position, position);
    assert_eq!(warrior.base.health.max, 80);
    assert!(warrior.base.stats.strength >= warrior.base.stats.intelligence);

    // Test mage enemy
    let mage = EnemyBundle::mage(position);
    assert_eq!(mage.base.health.max, 60);
    assert!(mage.base.stats.intelligence >= mage.base.stats.strength);

    // Test rogue enemy
    let rogue = EnemyBundle::rogue(position);
    assert_eq!(rogue.base.health.max, 70);
    assert!(rogue.base.stats.agility >= rogue.base.stats.strength);
}

#[test]
fn test_health_stats_integration() {
    let stats = Stats::new(10, 10, 10, 10, 15, 10); // High vitality
    let health_bonus = stats.health_bonus();

    // Health bonus should be (15 - 10) * 5 = 25
    assert_eq!(health_bonus, 25);

    // Create health with bonus applied
    let base_health = 100;
    let total_health = base_health + health_bonus;
    let health = Health::new(total_health);

    assert_eq!(health.max, 125);
}

#[test]
fn test_inventory_with_stats() {
    let stats = Stats::new(15, 10, 10, 10, 10, 10); // High strength
    let strength_bonus = stats.melee_damage_bonus();

    // Strength bonus should be 15 - 10 = 5
    assert_eq!(strength_bonus, 5);

    // Test inventory capacity (could be affected by strength in the future)
    let mut inventory = Inventory::new(20, 100.0);

    // Add a heavy item
    let heavy_item = InventoryItem::new(
        "sword".to_string(),
        "Heavy Sword".to_string(),
        1,
        1,
        15.0,
        "A very heavy sword".to_string(),
    );

    let result = inventory.add_item(heavy_item);
    assert!(result.is_ok());
    assert_eq!(inventory.current_weight, 15.0);
}

#[test]
fn test_component_cloning() {
    // Test that components can be cloned properly
    let health = Health::new_with_current(75, 100);
    let cloned_health = health.clone();

    assert_eq!(health, cloned_health);
    assert_eq!(health.current, cloned_health.current);
    assert_eq!(health.max, cloned_health.max);

    let stats = Stats::warrior();
    let cloned_stats = stats.clone();

    assert_eq!(stats, cloned_stats);
    assert_eq!(stats.strength, cloned_stats.strength);
}

#[test]
fn test_stat_modifiers_integration() {
    let base_stats = Stats::balanced(10);
    let mut modifiers = StatModifiers::new();

    // Add some temporary modifiers
    modifiers.add_modifier(StatType::Strength, 5);
    modifiers.add_modifier(StatType::Defense, -2);

    let modified_stats = base_stats.with_modifiers(&modifiers);

    assert_eq!(modified_stats.strength, 15);
    assert_eq!(modified_stats.defense, 8);
    assert_eq!(modified_stats.intelligence, 10); // Unchanged

    // Test damage calculations with modifiers
    assert_eq!(modified_stats.melee_damage_bonus(), 5); // 15 - 10
    assert_eq!(modified_stats.damage_reduction(), 0); // 8 - 10, but max(0)
}

#[test]
fn test_health_percentage_with_stats() {
    let stats = Stats::new(10, 10, 10, 10, 20, 10); // Very high vitality
    let health_bonus = stats.health_bonus(); // (20 - 10) * 5 = 50

    let mut health = Health::new(100 + health_bonus); // 150 total health
    assert_eq!(health.max, 150);
    assert_eq!(health.percentage(), 1.0);

    // Take some damage
    health.take_damage(75);
    assert_eq!(health.current, 75);
    assert_eq!(health.percentage(), 0.5);
}

#[test]
fn test_inventory_item_operations() {
    let mut item = InventoryItem::new(
        "potion".to_string(),
        "Health Potion".to_string(),
        5,
        10,
        0.5,
        "Restores health".to_string(),
    );

    // Test stacking
    assert_eq!(item.total_weight(), 2.5); // 5 * 0.5

    let overflow = item.add_quantity(7);
    assert_eq!(item.quantity, 10); // Capped at max_stack
    assert_eq!(overflow, 2); // 7 - 5 (could only add 5)

    // Test splitting
    let split_item = item.split(3).unwrap();
    assert_eq!(item.quantity, 7);
    assert_eq!(split_item.quantity, 3);
    assert_eq!(split_item.item_id, "potion");
}

#[test]
fn test_field_of_view_component() {
    let fov = FieldOfView::default();
    assert_eq!(*fov, 4);

    let custom_fov = FieldOfView::new(8);
    assert_eq!(*custom_fov, 8);
}

#[test]
fn test_description_component() {
    let desc = Description::new("A brave warrior");
    assert_eq!(desc.as_str(), "A brave warrior");

    let default_desc = Description::default();
    assert_eq!(default_desc.as_str(), "");
}
