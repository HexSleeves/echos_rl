use bevy::prelude::*;
use echos_assets::entities::ViewShedData;

/// Add this as a component to entities that can see
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ViewShed {
    pub radius: i32,
}

impl ViewShed {
    pub fn new(radius: i32) -> Self {
        debug_assert!(radius >= 0, "ViewShed radius must be non-negative");
        Self { radius }
    }
}

impl From<i32> for ViewShed {
    fn from(radius: i32) -> Self { Self::new(radius) }
}

impl From<&i32> for ViewShed {
    fn from(radius: &i32) -> Self { Self::new(*radius) }
}

impl From<ViewShedData> for ViewShed {
    fn from(data: ViewShedData) -> Self { Self::new(data.radius) }
}

impl From<&ViewShedData> for ViewShed {
    fn from(data: &ViewShedData) -> Self { Self::new(data.radius) }
}
