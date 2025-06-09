/// Debug macros for category-specific logging
/// These macros check if debugging is enabled for a specific category before logging
/// and provide zero-cost abstraction when debug features are disabled.
///
/// Internal macro for debug logging that handles the common logic
#[macro_export]
macro_rules! debug_log_internal {
    ($category:expr, $level:expr, $($arg:tt)*) => {
        // Check environment variable first for quick enable/disable
        // if $category.is_env_enabled() || $crate::debug::DebugConfig::load_or_default().is_category_enabled($category) {
        if $category.is_env_enabled() {
            let message = format!($($arg)*);

            // Log to console with category prefix
            match $level {
                "ERROR" => log::error!(target: $category.short_name(), "{}", message),
                "WARN" => log::warn!(target: $category.short_name(), "{}", message),
                "INFO" => log::info!(target: $category.short_name(), "{}", message),
                "DEBUG" => log::debug!(target: $category.short_name(), "{}", message),
                _ => log::info!(target: $category.short_name(), "{}", message),
            }

            // Log to file if file logging is enabled
            $crate::debug::log_to_file($category, $level, message);
        }
    };
}

/// AI system debugging
#[macro_export]
macro_rules! debug_ai {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::AI, "INFO", $($arg)*);
    };
}

/// Turn system debugging
#[macro_export]
macro_rules! debug_turns {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::Turns, "INFO", $($arg)*);
    };
}

/// Combat system debugging
#[macro_export]
macro_rules! debug_combat {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::Combat, "INFO", $($arg)*);
    };
}

/// World/map system debugging
#[macro_export]
macro_rules! debug_world {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::World, "INFO", $($arg)*);
    };
}

/// Input system debugging
#[macro_export]
macro_rules! debug_input {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::Input, "INFO", $($arg)*);
    };
}

/// Rendering system debugging
#[macro_export]
macro_rules! debug_rendering {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::Rendering, "INFO", $($arg)*);
    };
}

/// Performance debugging
#[macro_export]
macro_rules! debug_perf {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::Performance, "INFO", $($arg)*);
    };
}

/// General debugging
#[macro_export]
macro_rules! debug_general {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::General, "INFO", $($arg)*);
    };
}

/// State transitions debugging
#[macro_export]
macro_rules! debug_state_transitions {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::StateTransitions, "INFO", $($arg)*);
    };
}

// Warning level macros
#[macro_export]
macro_rules! warn_ai {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::AI, "WARN", $($arg)*);
    };
}

#[macro_export]
macro_rules! warn_turns {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::Turns, "WARN", $($arg)*);
    };
}

#[macro_export]
macro_rules! warn_combat {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::Combat, "WARN", $($arg)*);
    };
}

#[macro_export]
macro_rules! warn_world {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::World, "WARN", $($arg)*);
    };
}

// Error level macros
#[macro_export]
macro_rules! error_ai {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::AI, "ERROR", $($arg)*);
    };
}

#[macro_export]
macro_rules! error_turns {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::Turns, "ERROR", $($arg)*);
    };
}

#[macro_export]
macro_rules! error_combat {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::Combat, "ERROR", $($arg)*);
    };
}

#[macro_export]
macro_rules! error_world {
    ($($arg:tt)*) => {
        $crate::debug_log_internal!($crate::debug::DebugCategory::World, "ERROR", $($arg)*);
    };
}

/// Convenience macro for timing operations
#[macro_export]
macro_rules! debug_time {
    ($category:expr, $operation:expr, $code:block) => {{
        #[cfg(feature = "debug")]
        {
            // Execute the code block exactly once by wrapping it in a closure
            let execute_code = || $code;

            if $category.is_env_enabled() {
                let start = std::time::Instant::now();
                let result = execute_code();
                let duration = start.elapsed();
                $crate::debug_log_internal!($category, "INFO", "{} took {:?}", $operation, duration);
                result
            } else {
                execute_code()
            }
        }
        #[cfg(not(feature = "debug"))]
        {
            $code
        }
    }};
}

/// Convenience macro for AI timing
#[macro_export]
macro_rules! debug_ai_time {
    ($operation:expr, $code:block) => {
        $crate::debug_time!($crate::debug::DebugCategory::AI, $operation, $code)
    };
}

/// Convenience macro for turn timing
#[macro_export]
macro_rules! debug_turns_time {
    ($operation:expr, $code:block) => {
        $crate::debug_time!($crate::debug::DebugCategory::Turns, $operation, $code)
    };
}

/// Convenience macro for performance timing
#[macro_export]
macro_rules! debug_perf_time {
    ($operation:expr, $code:block) => {
        $crate::debug_time!($crate::debug::DebugCategory::Performance, $operation, $code)
    };
}
