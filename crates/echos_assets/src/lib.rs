use bevy::prelude::*;

pub mod entities;

pub struct EchosAssetsPlugin;

impl Plugin for EchosAssetsPlugin {
    fn build(&self, app: &mut App) { app.add_plugins((entities::plugin,)); }
}
