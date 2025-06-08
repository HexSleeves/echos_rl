use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use super::DebugCategory;

/// Debug configuration resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// Which debug categories are enabled
    pub categories: HashMap<DebugCategory, bool>,
    /// Whether file logging is enabled
    pub file_logging_enabled: bool,
    /// Path where log files are stored
    pub log_file_path: PathBuf,
    /// Maximum size of a single log file in bytes (default: 10MB)
    pub max_log_file_size: u64,
    /// Maximum number of log files to keep (default: 5)
    pub max_log_files: usize,
    /// Whether to include timestamps in console output
    pub console_timestamps: bool,
    /// Whether to use colored output in console
    pub console_colors: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        let mut categories = HashMap::new();

        // Initialize all categories as disabled by default
        for &category in DebugCategory::all() {
            categories.insert(category, false);
        }

        // Check environment variables for initial state
        for &category in DebugCategory::all() {
            if category.is_env_enabled() {
                categories.insert(category, true);
            }
        }

        Self {
            categories,
            file_logging_enabled: std::env::var("DEBUG_FILE_LOGGING").is_ok(),
            log_file_path: PathBuf::from("logs"),
            max_log_file_size: 10 * 1024 * 1024, // 10MB
            max_log_files: 5,
            console_timestamps: true,
            console_colors: true,
        }
    }
}

impl DebugConfig {
    /// Check if a specific debug category is enabled
    pub fn is_category_enabled(&self, category: DebugCategory) -> bool {
        self.categories.get(&category).copied().unwrap_or(false)
    }

    /// Enable or disable a debug category
    pub fn set_category_enabled(&mut self, category: DebugCategory, enabled: bool) {
        self.categories.insert(category, enabled);
    }

    /// Toggle a debug category
    pub fn toggle_category(&mut self, category: DebugCategory) {
        let current = self.is_category_enabled(category);
        self.set_category_enabled(category, !current);
    }

    /// Enable all debug categories
    pub fn enable_all(&mut self) {
        for &category in DebugCategory::all() {
            self.set_category_enabled(category, true);
        }
    }

    /// Disable all debug categories
    pub fn disable_all(&mut self) {
        for &category in DebugCategory::all() {
            self.set_category_enabled(category, false);
        }
    }

    /// Get a list of enabled categories
    pub fn enabled_categories(&self) -> Vec<DebugCategory> {
        self.categories
            .iter()
            .filter_map(|(&category, &enabled)| if enabled { Some(category) } else { None })
            .collect()
    }

    /// Load configuration from file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: DebugConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get the default config file path
    pub fn default_config_path() -> PathBuf { PathBuf::from("settings/debug_config.toml") }

    /// Load configuration with fallback to default
    pub fn load_or_default() -> Self {
        let config_path = Self::default_config_path();

        match Self::load_from_file(&config_path) {
            Ok(config) => {
                info!("Loaded debug configuration from {:?}", config_path);
                config
            }
            Err(e) => {
                info!("Failed to load debug config ({}), using defaults", e);
                let default_config = Self::default();

                // Try to save the default config for next time
                if let Err(save_err) = default_config.save_to_file(&config_path) {
                    warn!("Failed to save default debug config: {}", save_err);
                }

                default_config
            }
        }
    }
}

/// System to save debug configuration periodically or on changes
pub fn save_debug_config_system(config: Res<DebugConfig>) {
    if config.is_changed() {
        let config_path = DebugConfig::default_config_path();
        if let Err(e) = config.save_to_file(&config_path) {
            warn!("Failed to save debug configuration: {}", e);
        } else {
            debug!("Debug configuration saved to {:?}", config_path);
        }
    }
}
