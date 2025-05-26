use crate::model::{
    components::{PlayerTag, Position, TerrainType, ViewShed},
    resources::{FovMap, Map},
};
use bevy::prelude::*;

/// System that computes field of view for all entities with ViewShed components
///
/// This system runs when entities with ViewShed components have moved (Position changed)
/// or when their ViewShed component has been modified. It updates the global FovMap
/// resource with the combined visibility of all viewing entities.
///
/// # Performance Notes
/// - Only processes entities whose Position or ViewShed has changed
/// - Uses caching within FovMap to avoid repeated terrain queries
/// - Clears and recomputes the entire FOV map each frame for simplicity
pub fn compute_fov_system(
    map: Res<Map>,
    mut fov_map: ResMut<FovMap>,
    q_terrain: Query<&TerrainType>,
    q_viewers: Query<(&Position, &ViewShed), Or<(Changed<Position>, Changed<ViewShed>)>>,
) {
    // If no viewers have changed, skip computation
    if q_viewers.is_empty() {
        return;
    }

    // Clear the FOV map at the start of computation
    fov_map.clear_visibility();

    // Compute FOV for each viewer
    for (position, view_shed) in q_viewers.iter() {
        fov_map.compute_fov(&q_terrain, &map, *position, view_shed.radius);
    }
}

/// Alternative system that only computes FOV for the player
///
/// This is more efficient for single-player games where only the player's
/// FOV matters for rendering decisions.
pub fn compute_player_fov_system(
    mut fov_map: ResMut<FovMap>,
    q_terrain: Query<&TerrainType>,
    map: Res<Map>,
    q_player: Query<(&Position, &ViewShed), (With<PlayerTag>, Or<(Changed<Position>, Changed<ViewShed>)>)>,
) {
    // Only compute if player has moved or ViewShed changed
    if let Ok((position, view_shed)) = q_player.single() {
        fov_map.compute_fov(&q_terrain, &map, *position, view_shed.radius);
    }
}

/// System that can be used to debug FOV computation
///
/// Prints information about the current FOV state to the console.
/// Only enable this for debugging purposes as it can be verbose.
#[allow(dead_code)]
pub fn debug_fov_system(fov_map: Res<FovMap>, q_viewers: Query<(&Position, &ViewShed)>) {
    if fov_map.is_changed() {
        info!(
            "FOV Update: {} visible tiles, {} revealed tiles",
            fov_map.visible_tile_count(),
            fov_map.revealed_tile_count()
        );

        for (position, view_shed) in q_viewers.iter() {
            info!("Viewer at {:?} with radius {}", position, view_shed.radius);
        }
    }
}

/// System set for organizing FOV-related systems
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FovSystemSet {
    /// Systems that compute field of view
    Compute,
    /// Systems that react to FOV changes (rendering, AI, etc.)
    React,
}

/// Plugin that adds FOV systems to the app
pub struct FovPlugin;

impl Plugin for FovPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (FovSystemSet::Compute, FovSystemSet::React).chain()).add_systems(
            Update,
            (
                compute_player_fov_system.in_set(FovSystemSet::Compute),
                // Add other FOV-related systems here
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fov_system_setup() {
        let mut app = App::new();
        app.add_plugins(FovPlugin);

        // Verify that the systems are registered
        // This is a basic test to ensure the plugin loads without errors
        assert!(app.world().contains_resource::<Schedules>());
    }

    #[test]
    fn test_system_sets() {
        // Test that system sets are properly configured
        let compute_set = FovSystemSet::Compute;
        let react_set = FovSystemSet::React;

        assert_ne!(compute_set, react_set);
    }
}
