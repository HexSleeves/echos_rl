//! Tilemap module for handling the game world representation.
//! This module uses bevy_ecs_tilemap to implement a roguelike world.

use crate::screens::loading::TextureAssets;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

enum TileType {
    Wall,
    Floor,
}

impl TileType {
    fn texture_index(self) -> u32 {
        match self {
            TileType::Wall => 0,
            TileType::Floor => 1,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bevy_ecs_tilemap::TilemapPlugin);

    app.add_systems(Update, camera_movement);
}

// A simple camera system for moving and zooming the camera.
pub fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera>>,
) {
    for (mut transform, mut projection) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        let Projection::Orthographic(ortho) = &mut *projection else {
            continue;
        };

        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_secs() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}

/// System to generate a new map.
pub fn generate_map(mut commands: Commands, texture_assets: Res<TextureAssets>, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    fill_tilemap(TileTextureIndex(0), map_size, TilemapId(tilemap_entity), &mut commands, &mut tile_storage);

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        tile_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });
}
