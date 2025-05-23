use bevy::{color::Color, render::view::RenderLayers};

pub struct UiConstants;

impl UiConstants {
    /// This is the default layer, entities are added to this layer implicitly
    pub const GAME_LAYER: RenderLayers = RenderLayers::layer(0);

    /// This is the UI layer, entities shouldn't be assigned to this layer use TargetCamera for root
    /// UI nodes
    pub const UI_LAYER: RenderLayers = RenderLayers::layer(1);
}

pub struct Palette;

impl Palette {
    pub const LABEL_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);

    /// #fcfbcc
    pub const HEADER_TEXT: Color = Color::srgb(0.988, 0.984, 0.800);

    /// #ececec
    pub const BUTTON_TEXT: Color = Color::srgb(0.925, 0.925, 0.925);

    /// #4666bf
    pub const BUTTON_BACKGROUND: Color = Color::srgb(0.275, 0.400, 0.750);

    /// #6299d1
    pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.384, 0.600, 0.820);

    /// #3d4999
    pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.239, 0.286, 0.600);
}
