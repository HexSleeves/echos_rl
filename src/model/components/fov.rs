use bevy::prelude::*;
use echos_assets::entities::ViewShedData;

/// Add this as a component to entities that can see
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ViewShed {
    pub radius: u8,
}

impl ViewShed {
    pub fn new(radius: u8) -> Self {
        debug_assert!(radius > 0, "ViewShed radius must be greater than 0");
        Self { radius }
    }
}

impl From<u8> for ViewShed {
    fn from(radius: u8) -> Self { Self::new(radius) }
}

impl From<&u8> for ViewShed {
    fn from(radius: &u8) -> Self { Self::new(*radius) }
}

impl From<ViewShedData> for ViewShed {
    fn from(data: ViewShedData) -> Self { Self::new(data.radius) }
}

impl From<&ViewShedData> for ViewShed {
    fn from(data: &ViewShedData) -> Self { Self::new(data.radius) }
}
