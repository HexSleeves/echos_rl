use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TilePos};

use echos_in_the_dark::{
    core::{
        components::{Position, light::Light},
        resources::{FovMap, LightMap},
    },
    rendering::systems::{update_sprite_visibility, update_tilemap_visibility},
};

#[test]
fn test_light_map_integration() {
    let mut app = App::new();

    // Add required resources
    app.insert_resource(FovMap::new(50, 50)).insert_resource(LightMap::new());

    // Create a test entity with a light source
    let _light_entity = app
        .world_mut()
        .spawn((
            Position::new(10, 10),
            Light {
                range: 5,
                color: Color::srgb(1.0, 0.5, 0.2), // Orange light
                falloff: 1.5,
            },
        ))
        .id();

    // Create a test tile
    let tile_entity = app.world_mut().spawn((TileColor(Color::WHITE), TilePos { x: 10, y: 10 })).id();

    // Create a test sprite
    let sprite_entity =
        app.world_mut().spawn((Position::new(12, 12), Visibility::Visible, Sprite::default())).id();

    // Set up FOV to make positions visible
    {
        let mut fov_map = app.world_mut().resource_mut::<FovMap>();
        fov_map.set_visible(Position::new(10, 10), true);
        fov_map.set_visible(Position::new(12, 12), true);
    }

    // Manually populate light map (normally done by calculate_light_map system)
    {
        let mut light_map = app.world_mut().resource_mut::<LightMap>();
        light_map.set_light((10, 10), Color::srgb(1.0, 0.5, 0.2)); // Full intensity at source
        light_map.set_light((12, 12), Color::srgb(0.6, 0.3, 0.12)); // Dimmed at distance
    }

    // Add the lighting systems
    app.add_systems(Update, (update_tilemap_visibility, update_sprite_visibility));

    // Run one update cycle
    app.update();

    // Check that the tile color was affected by lighting
    let tile_color = app.world().entity(tile_entity).get::<TileColor>().unwrap();
    assert_ne!(tile_color.0, Color::WHITE); // Should not be pure white anymore
    assert_ne!(tile_color.0, Color::BLACK); // Should not be black either

    // Check that the sprite color was affected by lighting
    let sprite = app.world().entity(sprite_entity).get::<Sprite>().unwrap();
    assert_ne!(sprite.color, Color::WHITE); // Should not be pure white anymore
    assert_ne!(sprite.color, Color::BLACK); // Should not be black either

    // The colors should have some orange tint from the light
    let sprite_linear = sprite.color.to_linear();
    // Orange light (1.0, 0.5, 0.2) should result in red > green > blue
    assert!(
        sprite_linear.red > sprite_linear.green,
        "Expected red > green due to orange light, got red: {}, green: {}",
        sprite_linear.red,
        sprite_linear.green
    );
    assert!(
        sprite_linear.green > sprite_linear.blue,
        "Expected green > blue due to orange light, got green: {}, blue: {}",
        sprite_linear.green,
        sprite_linear.blue
    );
}

#[test]
fn test_minimum_light_levels() {
    let mut app = App::new();

    // Add required resources
    app.insert_resource(FovMap::new(50, 50)).insert_resource(LightMap::new()); // Empty light map (no lights)

    // Create a test tile in a dark area
    let tile_entity = app.world_mut().spawn((TileColor(Color::WHITE), TilePos { x: 20, y: 20 })).id();

    // Set up FOV to make position visible but no light
    {
        let mut fov_map = app.world_mut().resource_mut::<FovMap>();
        fov_map.set_visible(Position::new(20, 20), true);
    }

    // Add the lighting system
    app.add_systems(Update, update_tilemap_visibility);

    // Run one update cycle
    app.update();

    // Check that minimum light level is applied
    let tile_color = app.world().entity(tile_entity).get::<TileColor>().unwrap();
    assert_ne!(tile_color.0, Color::BLACK); // Should not be completely black

    // Should have minimum light level (0.2 as defined in the system)
    let tile_linear = tile_color.0.to_linear();
    assert!(tile_linear.red >= 0.19); // Allow for small floating point differences
    assert!(tile_linear.green >= 0.19);
    assert!(tile_linear.blue >= 0.19);
}

#[test]
fn test_fog_of_war_with_lighting() {
    let mut app = App::new();

    // Add required resources
    app.insert_resource(FovMap::new(50, 50)).insert_resource(LightMap::new());

    // Create a test tile
    let tile_entity = app.world_mut().spawn((TileColor(Color::WHITE), TilePos { x: 15, y: 15 })).id();

    // Set up FOV to make position revealed but not currently visible
    {
        let mut fov_map = app.world_mut().resource_mut::<FovMap>();
        fov_map.set_revealed(Position::new(15, 15), true);
        // Note: not setting as visible, only revealed
    }

    // Add the lighting system
    app.add_systems(Update, update_tilemap_visibility);

    // Run one update cycle
    app.update();

    // Check that fog of war color is applied
    let tile_color = app.world().entity(tile_entity).get::<TileColor>().unwrap();

    // Should be the fog of war color (0.4, 0.4, 0.4, 1.0)
    let expected_fog_color = Color::srgba(0.4, 0.4, 0.4, 1.0);
    assert_eq!(tile_color.0, expected_fog_color);
}

#[test]
fn test_unexplored_areas() {
    let mut app = App::new();

    // Add required resources
    app.insert_resource(FovMap::new(50, 50)).insert_resource(LightMap::new());

    // Create a test tile
    let tile_entity = app.world_mut().spawn((TileColor(Color::WHITE), TilePos { x: 25, y: 25 })).id();

    // Don't set up any FOV (position remains unexplored)

    // Add the lighting system
    app.add_systems(Update, update_tilemap_visibility);

    // Run one update cycle
    app.update();

    // Check that unexplored areas are black
    let tile_color = app.world().entity(tile_entity).get::<TileColor>().unwrap();
    assert_eq!(tile_color.0, Color::BLACK);
}
