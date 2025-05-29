use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    model::components::{TurnActor, ViewShed},
    view::{ViewConstants, components::TileSprite},
};

/// Data representation of TurnActor component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct TurnActorData {
    /// Speed value for turn timing
    pub speed: u64,
    /// Maximum number of actions that can be queued
    pub action_queue_size: Option<usize>,
}

impl TurnActorData {
    pub fn new(speed: u64) -> Self { Self { speed, action_queue_size: None } }

    pub fn with_queue_size(mut self, size: usize) -> Self {
        self.action_queue_size = Some(size);
        self
    }
}

impl From<TurnActorData> for TurnActor {
    fn from(data: TurnActorData) -> Self { TurnActor::new(data.speed) }
}

impl From<&TurnActorData> for TurnActor {
    fn from(data: &TurnActorData) -> Self { TurnActor::new(data.speed) }
}

/// Data representation of ViewShed component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct ViewShedData {
    /// Vision radius in tiles
    pub radius: i32,
}

impl ViewShedData {
    pub fn new(radius: i32) -> Self { Self { radius } }
}

impl From<ViewShedData> for ViewShed {
    fn from(data: ViewShedData) -> Self { ViewShed { radius: data.radius } }
}

impl From<&ViewShedData> for ViewShed {
    fn from(data: &ViewShedData) -> Self { ViewShed { radius: data.radius } }
}

/// Data representation of TileSprite component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct TileSpriteData {
    /// Tile coordinates in the tilemap (x, y)
    pub tile_coords: (u32, u32),
    /// Size of each tile (defaults to ViewConstants::TILE_SIZE if not specified)
    pub tile_size: Option<(f32, f32)>,
    /// Optional tint color
    pub tint: Option<(f32, f32, f32, f32)>, // RGBA
}

impl TileSpriteData {
    pub fn new(tile_coords: (u32, u32)) -> Self { Self { tile_coords, tile_size: None, tint: None } }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.tile_size = Some((width, height));
        self
    }

    pub fn with_tint(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.tint = Some((r, g, b, a));
        self
    }
}

impl From<TileSpriteData> for TileSprite {
    fn from(data: TileSpriteData) -> Self {
        let tile_size = data
            .tile_size
            .map(|(w, h)| Vec2::new(w, h))
            .unwrap_or_else(|| Vec2::splat(ViewConstants::TILE_SIZE));

        let tint = data.tint.map(|(r, g, b, a)| Color::srgba(r, g, b, a));

        TileSprite { tile_coords: data.tile_coords, tile_size, tint }
    }
}

impl From<&TileSpriteData> for TileSprite {
    fn from(data: &TileSpriteData) -> Self {
        let tile_size = data
            .tile_size
            .map(|(w, h)| Vec2::new(w, h))
            .unwrap_or_else(|| Vec2::splat(ViewConstants::TILE_SIZE));

        let tint = data.tint.map(|(r, g, b, a)| Color::srgba(r, g, b, a));

        TileSprite { tile_coords: data.tile_coords, tile_size, tint }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::entities::EntityDefinition;

    #[test]
    fn test_turn_actor_conversion() {
        let data = TurnActorData::new(100);
        let component: TurnActor = data.into();
        assert_eq!(component.speed, 100);
        assert!(component.is_alive());
    }

    #[test]
    fn test_view_shed_conversion() {
        let data = ViewShedData::new(8);
        let component: ViewShed = data.into();
        assert_eq!(component.radius, 8);
    }

    #[test]
    fn test_tile_sprite_conversion() {
        let data = TileSpriteData::new((10, 18));
        let component: TileSprite = data.into();
        assert_eq!(component.tile_coords, (10, 18));
        assert_eq!(component.tile_size, Vec2::splat(ViewConstants::TILE_SIZE));
        assert_eq!(component.tint, None);
    }

    #[test]
    fn test_tile_sprite_with_custom_size() {
        let data = TileSpriteData::new((5, 10)).with_size(16.0, 16.0);
        let component: TileSprite = data.into();
        assert_eq!(component.tile_coords, (5, 10));
        assert_eq!(component.tile_size, Vec2::new(16.0, 16.0));
    }

    #[test]
    fn test_tile_sprite_with_tint() {
        let data = TileSpriteData::new((1, 2)).with_tint(1.0, 0.5, 0.0, 0.8);
        let component: TileSprite = data.into();
        assert_eq!(component.tint, Some(Color::srgba(1.0, 0.5, 0.0, 0.8)));
    }

    #[test]
    fn test_serialization_round_trip() {
        let original = TurnActorData::new(150).with_queue_size(10);
        let serialized = ron::to_string(&original).expect("Failed to serialize");
        let deserialized: TurnActorData = ron::from_str(&serialized).expect("Failed to deserialize");

        assert_eq!(original.speed, deserialized.speed);
        assert_eq!(original.action_queue_size, deserialized.action_queue_size);
    }

    #[test]
    fn test_player_ron_file_parsing() {
        let player_ron = include_str!("../../../assets/entities/player.definition.ron");
        let entity_def: EntityDefinition = ron::from_str(player_ron).expect("Failed to parse player.ron");

        assert_eq!(entity_def.name, "Player");
        assert!(entity_def.is_player());
        assert!(!entity_def.is_ai());

        // Verify component data matches hardcoded values
        let components = &entity_def.components;
        assert_eq!(components.turn_actor.as_ref().unwrap().speed, 100);
        assert_eq!(components.view_shed.as_ref().unwrap().radius, 8);
        assert_eq!(components.tile_sprite.as_ref().unwrap().tile_coords, (10, 18));
    }

    #[test]
    fn test_whale_ron_file_parsing() {
        let whale_ron = include_str!("../../../assets/entities/enemies/whale.definition.ron");
        let entity_def: EntityDefinition = ron::from_str(whale_ron).expect("Failed to parse whale.ron");

        assert_eq!(entity_def.name, "Whale");
        assert!(!entity_def.is_player());
        assert!(entity_def.is_ai());

        // Verify component data matches hardcoded values
        let components = &entity_def.components;
        assert_eq!(components.turn_actor.as_ref().unwrap().speed, 120);
        assert_eq!(components.tile_sprite.as_ref().unwrap().tile_coords, (0, 16));
        assert!(components.view_shed.is_none()); // Enemies don't have view sheds
    }
}
