use std::path::{Path, PathBuf};

use bevy::prelude::*;
use directories::ProjectDirs;

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct Folders {
    pub base: PathBuf,
    pub config: PathBuf,
    pub data: PathBuf,
}

impl Folders {
    pub fn new(
        base: impl AsRef<Path>,
        qualifier: impl ToString,
        organization: impl ToString,
        application: impl ToString,
    ) -> Self {
        let project_dirs =
            ProjectDirs::from(&qualifier.to_string(), &organization.to_string(), &application.to_string())
                .expect("Failed to get project directories");

        Self {
            base: base.as_ref().to_path_buf(),
            config: project_dirs.config_dir().to_path_buf(),
            data: project_dirs.data_dir().to_path_buf(),
        }
    }

    pub fn base(&self, file_path: impl AsRef<Path>) -> PathBuf { self.base.join(file_path) }

    pub fn config(&self, file_path: impl AsRef<Path>) -> PathBuf { self.config.join(file_path) }

    pub fn data(&self, file_path: impl AsRef<Path>) -> PathBuf { self.data.join(file_path) }
}

// Read
impl Folders {
    pub fn read_base(&self, file_path: impl AsRef<Path>) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self.base(file_path))
    }

    pub fn read_base_u8(&self, file_path: impl AsRef<Path>) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(self.base(file_path))
    }

    pub fn read_config(&self, file_path: impl AsRef<Path>) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self.config(file_path))
    }

    pub fn read_config_u8(&self, file_path: impl AsRef<Path>) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(self.config(file_path))
    }

    pub fn read_data(&self, file_path: impl AsRef<Path>) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self.data(file_path))
    }

    pub fn read_data_u8(&self, file_path: impl AsRef<Path>) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(self.data(file_path))
    }
}

// Write
impl Folders {
    pub fn write_base(
        &self,
        file_path: impl AsRef<Path>,
        contents: impl AsRef<[u8]>,
    ) -> Result<(), std::io::Error> {
        Self::write(self.base(file_path), contents)
    }

    pub fn write_config(
        &self,
        file_path: impl AsRef<Path>,
        contents: impl AsRef<[u8]>,
    ) -> Result<(), std::io::Error> {
        Self::write(self.config(file_path), contents)
    }

    pub fn write_data(
        &self,
        file_path: impl AsRef<Path>,
        contents: impl AsRef<[u8]>,
    ) -> Result<(), std::io::Error> {
        Self::write(self.data(file_path), contents)
    }

    fn write(path: PathBuf, contents: impl AsRef<[u8]>) -> Result<(), std::io::Error> {
        let mut dir = path.clone();
        dir.pop();
        std::fs::create_dir_all(&dir)?;
        std::fs::write(path, contents)
    }
}
