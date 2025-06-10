use bevy::prelude::*;
use echos_in_the_dark::{
    core::{
        actions::AttackAction,
        components::{Health, Position, Stats},
        resources::CurrentMap,
        types::GameAction,
    },
    gameplay::world::components::TerrainType,
};

#[cfg(test)]
mod tests {
    use super::*;

    use echos_in_the_dark::{Grid, prelude::core::Tile};

    fn setup_test_world() -> (World, Entity, Entity) {
        let mut world = World::new();

        // Initialize map resource
        let map = CurrentMap::from_world(&mut world);
        world.insert_resource(map);

        // Create attacker entity
        let attacker = world
            .spawn((
                Position::new(0, 0),
                Stats::warrior(), // High strength for testing
                Health::new(100),
            ))
            .id();

        // Create target entity
        let target = world
            .spawn((
                Position::new(1, 0),
                Stats::balanced(8), // Lower stats for easier testing
                Health::new(50),
            ))
            .id();

        // Set up walkable terrain and place entities on the map
        let mut map = world.resource_mut::<CurrentMap>();
        map.tiles = Grid::new_fill(map.size, Tile { terrain: TerrainType::Floor, ..Default::default() });
        map.place_actor(Position::new(0, 0), attacker).unwrap();
        map.place_actor(Position::new(1, 0), target).unwrap();

        (world, attacker, target)
    }

    #[test]
    fn test_attack_action_hits_and_deals_damage() {
        let (mut world, attacker, target) = setup_test_world();

        // Get initial health
        let initial_health = world.get::<Health>(target).unwrap().current;

        // Create and execute attack action
        let mut attack = AttackAction::new(attacker, Position::new(1, 0));
        let result = attack.execute(&mut world);

        // Attack should succeed
        assert!(result.is_ok());

        // Target should have taken damage
        let final_health = world.get::<Health>(target).unwrap().current;
        assert!(final_health < initial_health, "Target should have taken damage");
    }

    #[test]
    fn test_attack_action_can_kill_target() {
        let (mut world, attacker, target) = setup_test_world();

        // Set target to very low health
        world.get_mut::<Health>(target).unwrap().set_current(1);

        // Create and execute attack action
        let mut attack = AttackAction::new(attacker, Position::new(1, 0));
        let result = attack.execute(&mut world);

        // Attack should succeed
        assert!(result.is_ok());

        // Target should be dead
        let target_health = world.get::<Health>(target).unwrap();
        assert!(target_health.is_dead(), "Target should be dead");
    }

    #[test]
    fn test_attack_action_on_empty_position() {
        let (mut world, attacker, _target) = setup_test_world();

        // Attack empty position
        let mut attack = AttackAction::new(attacker, Position::new(5, 5));
        let result = attack.execute(&mut world);

        // Attack should succeed but do nothing
        assert!(result.is_ok());
    }

    #[test]
    fn test_damage_calculation_with_stats() {
        let (mut world, attacker, target) = setup_test_world();

        // Give attacker high strength
        world.get_mut::<Stats>(attacker).unwrap().strength = 20;

        // Give target high defense
        world.get_mut::<Stats>(target).unwrap().defense = 15;

        let initial_health = world.get::<Health>(target).unwrap().current;

        // Execute attack
        let mut attack = AttackAction::new(attacker, Position::new(1, 0));
        let result = attack.execute(&mut world);

        assert!(result.is_ok());

        // Verify damage was calculated with stats
        let final_health = world.get::<Health>(target).unwrap().current;
        let damage_dealt = initial_health - final_health;

        // With high strength and defense, damage should still be at least 1
        assert!(damage_dealt >= 1, "Minimum damage should be 1");
    }
}
