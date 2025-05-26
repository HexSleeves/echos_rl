use crate::model::{
    components::{PlayerTag, Position, ViewShed},
    resources::{CurrentMap, FovMap, Map},
};
use bevy::prelude::*;

/// System set for organizing FOV-related systems
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FovSystemSet {
    /// Systems that compute field of view
    Compute,
    /// Systems that react to FOV changes (rendering, AI, etc.)
    React,
}

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
pub fn compute_fov(
    map: Res<Map>,
    mut fov_map: ResMut<FovMap>,
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
        fov_map.compute_fov(&map, *position, view_shed.radius);
    }
}

/// Alternative system that only computes FOV for the player
///
/// This is more efficient for single-player games where only the player's
/// FOV matters for rendering decisions.
pub fn compute_player_fov(
    map: Res<CurrentMap>,
    mut fov_map: ResMut<FovMap>,
    q_player: Query<
        (&Position, &ViewShed),
        (With<PlayerTag>, Or<(Changed<Position>, Changed<ViewShed>)>),
    >,
) {
    // Only compute if player has moved or ViewShed changed
    if let Ok((position, view_shed)) = q_player.single() {
        fov_map.compute_fov(&map, *position, view_shed.radius);
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
