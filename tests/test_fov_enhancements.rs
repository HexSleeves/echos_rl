use brtk::fov::{
    FovAlgorithmType,
    implementations::{map_provider::GridMapProvider, visibility_map::VisibilityMap},
    traits::FovReceiver,
};

#[test]
fn test_fog_of_war_functionality() {
    let mut visibility_map = VisibilityMap::new();

    // Initially nothing is visible or explored
    assert!(!visibility_map.get_visible((5, 5)));
    assert!(!visibility_map.get_explored((5, 5)));

    // Set a position as visible (should also mark as explored)
    visibility_map.set_visible((5, 5));
    assert!(visibility_map.get_visible((5, 5)));
    assert!(visibility_map.get_explored((5, 5)));

    // Clear visible but explored should remain
    visibility_map.clear_visible();
    assert!(!visibility_map.get_visible((5, 5)));
    assert!(visibility_map.get_explored((5, 5))); // Still explored (fog of war)

    // Can manually set explored areas
    visibility_map.set_explored((10, 10));
    assert!(!visibility_map.get_visible((10, 10)));
    assert!(visibility_map.get_explored((10, 10)));
}

#[test]
fn test_visibility_map_performance_optimization() {
    // Test range-based capacity optimization
    let mut visibility_map = VisibilityMap::with_range_capacity(10);

    // Should have reasonable capacity for range 10
    // π * 10² ≈ 314, so capacity should be at least 314
    assert!(visibility_map.get_visible_set().capacity() >= 314);

    // Test dynamic optimization
    visibility_map.optimize_for_range(5);
    visibility_map.optimize_for_range(20);

    // Should handle range changes efficiently
    assert!(visibility_map.get_visible_set().capacity() > 0);
}

#[test]
fn test_fov_algorithm_with_large_map() {
    // Create a large map to test performance
    let mut map_provider = GridMapProvider::new(100, 100, false);

    // Add some walls
    for i in 20..80 {
        map_provider.set_opaque(i, 50, true);
        map_provider.set_opaque(50, i, true);
    }

    let mut visibility_map = VisibilityMap::with_range_capacity(30);

    // Compute FOV from center with large range
    FovAlgorithmType::Shadowcast.compute((50, 50), 0, 30, &mut map_provider, &mut visibility_map);

    // Should have computed visibility for a reasonable number of tiles
    assert!(visibility_map.visible_count() > 100);
    assert!(visibility_map.visible_count() < 3000); // Shouldn't be too many due to walls

    // Center should be visible
    assert!(visibility_map.get_visible((50, 50)));
}

#[test]
fn test_directional_fov() {
    let mut map_provider = GridMapProvider::new(20, 20, false);
    let mut visibility_map = VisibilityMap::new();

    // Test directional FOV (cone vision)
    FovAlgorithmType::ShadowcastDirection(brtk::direction::Direction::NORTH).compute(
        (10, 10),
        0,
        8,
        &mut map_provider,
        &mut visibility_map,
    );

    // Should have some visible tiles
    assert!(visibility_map.visible_count() > 0);

    // Center should be visible
    assert!(visibility_map.get_visible((10, 10)));
}

#[test]
fn test_explored_areas_persistence() {
    let mut visibility_map = VisibilityMap::new();

    // Simulate multiple FOV computations (like a player moving)
    let positions = [(5, 5), (6, 6), (7, 7)];

    for &pos in &positions {
        visibility_map.set_visible(pos);
    }

    // All should be explored
    for &pos in &positions {
        assert!(visibility_map.get_explored(pos));
    }

    // Clear visible (simulate moving away)
    visibility_map.clear_visible();

    // Nothing should be visible now
    for &pos in &positions {
        assert!(!visibility_map.get_visible(pos));
    }

    // But all should still be explored (fog of war)
    for &pos in &positions {
        assert!(visibility_map.get_explored(pos));
    }

    // Get all explored areas
    let explored = visibility_map.get_all_explored();
    assert_eq!(explored.len(), 3);

    // Clear explored areas
    visibility_map.clear_explored();
    assert_eq!(visibility_map.get_all_explored().len(), 0);
}
