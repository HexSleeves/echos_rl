use std::path::Path;

use bevy::prelude::*;

use crate::{grid::Grid, resources::Folders, systems::remove_resource};

#[cfg(feature = "icon")]
use crate::{resources::WindowIcon, systems::set_window_icon};

pub struct BrtkPlugin {
    pub folders: Folders,

    #[cfg(feature = "icon")]
    pub icon: Option<&'static [u8]>,
}

impl BrtkPlugin {
    pub fn new(
        base: impl AsRef<Path>,
        qualifier: impl ToString,
        orginization: impl ToString,
        application: impl ToString,
    ) -> Self {
        let folders = Folders::new(base, qualifier, orginization, application);
        Self {
            folders,

            #[cfg(feature = "icon")]
            icon: None,
        }
    }

    #[cfg(feature = "icon")]
    pub fn with_icon(mut self, icon: &'static [u8]) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn folders(&self) -> &Folders { &self.folders }
}

impl Plugin for BrtkPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        // Grid
        app.register_type::<Grid<Entity>>();
        app.register_type::<Grid<Option<Entity>>>();
        app.register_type::<Grid<Vec<Entity>>>();
        app.register_type::<Grid<bool>>();

        app.register_type::<Folders>();
        app.insert_resource(self.folders.clone());

        #[cfg(feature = "icon")]
        if let Some(icon) = self.icon {
            app.insert_resource(WindowIcon(icon));
            app.add_systems(Startup, (set_window_icon, remove_resource::<WindowIcon>).chain());
        }
    }
}
