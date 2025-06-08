use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::SystemTime,
};

use chrono::TimeZone;

use super::{DebugCategory, DebugConfig};

/// File logger that handles writing debug messages to files
#[derive(Debug)]
pub struct FileLogger {
    writers: Arc<Mutex<std::collections::HashMap<DebugCategory, BufWriter<File>>>>,
    config: DebugConfig,
}

impl FileLogger {
    /// Create a new file logger with the given configuration
    pub fn new(config: DebugConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let writers = Arc::new(Mutex::new(std::collections::HashMap::new()));

        // Create log directory if it doesn't exist
        std::fs::create_dir_all(&config.log_file_path)?;

        Ok(Self { writers, config })
    }

    /// Get or create a writer for the given category
    fn get_writer(&self, category: DebugCategory) -> Result<(), Box<dyn std::error::Error>> {
        let mut writers = self.writers.lock().unwrap();

        if let std::collections::hash_map::Entry::Vacant(e) = writers.entry(category) {
            let log_file_path = self.get_log_file_path(category);

            // Check if we need to rotate the log file
            if let Err(err) = self.rotate_log_if_needed(&log_file_path) {
                eprintln!("Failed to rotate log file: {err}");
            }

            let file = OpenOptions::new().create(true).append(true).open(&log_file_path)?;

            let writer = BufWriter::new(file);
            e.insert(writer);
        }

        Ok(())
    }

    /// Get the log file path for a category
    fn get_log_file_path(&self, category: DebugCategory) -> PathBuf {
        let filename = format!("echos_rl_{}.log", category.short_name());
        self.config.log_file_path.join(filename)
    }

    /// Rotate log file if it exceeds the maximum size
    fn rotate_log_if_needed(&self, log_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !log_path.exists() {
            return Ok(());
        }

        let metadata = std::fs::metadata(log_path)?;
        if metadata.len() > self.config.max_log_file_size {
            self.rotate_log_files(log_path)?;
        }

        Ok(())
    }

    /// Rotate log files (move current to .1, .1 to .2, etc.)
    fn rotate_log_files(&self, log_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let base_path = log_path.with_extension("");
        let extension = log_path.extension().unwrap_or_default();

        // Move existing rotated files
        // include the current log (index 0) and create space for `.1`
        for i in (0..=self.config.max_log_files).rev() {
            let from_path = if i == 1 {
                log_path.to_path_buf()
            } else {
                base_path.with_extension(format!("{}.{}", extension.to_string_lossy(), i))
            };

            let to_path = base_path.with_extension(format!("{}.{}", extension.to_string_lossy(), i + 1));
            if from_path.exists() {
                if i + 1 > self.config.max_log_files {
                    // Delete the oldest file
                    std::fs::remove_file(&from_path)?;
                } else {
                    // Move to next rotation
                    std::fs::rename(&from_path, &to_path)?;
                }
            }
        }

        Ok(())
    }

    /// Write a log message to the appropriate file
    pub fn log(
        &self,
        category: DebugCategory,
        level: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.file_logging_enabled {
            return Ok(());
        }

        self.get_writer(category)?;

        let mut writers = self.writers.lock().unwrap();
        if let Some(writer) = writers.get_mut(&category) {
            let timestamp = format_timestamp(SystemTime::now());
            writeln!(writer, "[{}] [{}] [{}] {}", timestamp, level, category.short_name(), message)?;
            writer.flush()?;
        }

        Ok(())
    }

    /// Flush all writers
    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut writers = self.writers.lock().unwrap();
        for writer in writers.values_mut() {
            writer.flush()?;
        }
        Ok(())
    }
}

#[cfg(feature = "debug-logging")]
/// Global file logger instance
static FILE_LOGGER: once_cell::sync::OnceCell<FileLogger> = once_cell::sync::OnceCell::new();

/// Setup file logging with the current debug configuration
#[cfg(feature = "debug-logging")]
pub fn setup_file_logging() -> Result<(), Box<dyn std::error::Error>> {
    let config = DebugConfig::load_or_default();
    let logger = FileLogger::new(config)?;

    FILE_LOGGER.set(logger).map_err(|_| "File logger already initialized")?;

    Ok(())
}

/// Log a message to file if file logging is enabled
pub fn log_to_file(category: DebugCategory, level: &str, message: String) {
    #[cfg(feature = "debug-logging")]
    {
        if let Some(logger) = FILE_LOGGER.get() {
            if let Err(e) = logger.log(category, level, &message) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
    }

    #[cfg(not(feature = "debug-logging"))]
    {
        // Suppress unused variable warnings when debug-logging is disabled
        let _ = (category, level, message);
    }
}

/// Flush all log files
pub fn flush_logs() {
    #[cfg(feature = "debug-logging")]
    {
        if let Some(logger) = FILE_LOGGER.get() {
            if let Err(e) = logger.flush() {
                eprintln!("Failed to flush log files: {}", e);
            }
        }
    }
}

/// Format a timestamp for log entries
fn format_timestamp(time: SystemTime) -> String {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let nanos = duration.subsec_nanos();

            // Convert to a readable format (simplified)
            let datetime =
                chrono::Utc.timestamp_opt(secs as i64, nanos).single().unwrap_or_else(chrono::Utc::now);

            datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
        }
        Err(_) => "INVALID_TIME".to_string(),
    }
}

/// Generate a bug report with recent log entries
pub fn generate_bug_report() -> Result<String, Box<dyn std::error::Error>> {
    let config = DebugConfig::load_or_default();
    let mut report = String::new();

    report.push_str("=== ECHOS RL BUG REPORT ===\n");
    report.push_str(&format!("Generated: {}\n", format_timestamp(SystemTime::now())));
    report.push_str(&format!("Debug Config: {:?}\n", config.enabled_categories()));
    report.push_str("\n=== LOG ENTRIES ===\n");

    // Read recent entries from each log file
    for &category in DebugCategory::all() {
        if config.is_category_enabled(category) {
            let log_path = config.log_file_path.join(format!("echos_rl_{}.log", category.short_name()));

            if log_path.exists() {
                report.push_str(&format!("\n--- {} ---\n", category.display_name()));

                match read_last_lines(&log_path, 50) {
                    Ok(lines) => report.push_str(&lines),
                    Err(e) => report.push_str(&format!("Error reading log: {e}\n")),
                }
            }
        }
    }

    Ok(report)
}

/// Read the last N lines from a file
fn read_last_lines(path: &Path, n: usize) -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();

    let start = if lines.len() > n { lines.len() - n } else { 0 };
    let last_lines = lines[start..].join("\n");

    Ok(last_lines)
}
