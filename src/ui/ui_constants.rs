use bevy::render::view::RenderLayers;

pub struct UiConstants;

impl UiConstants {
    /// This is the default layer, entities are added to this layer implicitly
    pub const GAME_LAYER: RenderLayers = RenderLayers::layer(0);
    /// This is the UI layer, entities shouldn't be assigned to this layer use TargetCamera for root
    /// UI nodes
    pub const UI_LAYER: RenderLayers = RenderLayers::layer(1);
}
