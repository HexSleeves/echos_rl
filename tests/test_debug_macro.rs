// Test file to demonstrate the debug_time! macro fix
// This file shows that the code block is executed exactly once

use std::sync::atomic::{AtomicU32, Ordering};

// Mock the debug category for testing
#[cfg(feature = "debug")]
#[derive(Debug, Clone, Copy)]
enum DebugCategory {
    AI,
}

#[cfg(feature = "debug")]
impl DebugCategory {
    fn is_env_enabled(&self) -> bool {
        // For testing, we'll simulate both enabled and disabled states
        std::env::var("TEST_DEBUG_ENABLED").is_ok()
    }
}

// Mock the debug_log_internal macro for testing
#[cfg(feature = "debug")]
macro_rules! debug_log_internal {
    ($category:expr, $level:expr, $($arg:tt)*) => {
        println!("DEBUG: {}", format!($($arg)*));
    };
}

// The fixed debug_time! macro
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
                debug_log_internal!($category, "INFO", "{} took {:?}", $operation, duration);
                result
            } else {
                execute_code()
            }
        }
        #[cfg(not(feature = "debug"))]
        {
            // Execute the code block exactly once by wrapping it in a closure
            let execute_code = || $code;
            execute_code()
        }
    }};
}

#[test]
fn test_debug_macro() {
    static COUNTER: AtomicU32 = AtomicU32::new(0);

    println!("Testing debug_time! macro to ensure single execution...");

    // Test with debug disabled (no env var set)
    let result1 = debug_time!(DebugCategory::AI, "test operation", {
        let count = COUNTER.fetch_add(1, Ordering::SeqCst);
        println!("Code block executed, counter: {}", count + 1);
        42
    });

    println!("Result: {}, Counter after first call: {}", result1, COUNTER.load(Ordering::SeqCst));

    // Test with debug enabled
    unsafe {
        std::env::set_var("TEST_DEBUG_ENABLED", "1");
    }

    let result2 = debug_time!(DebugCategory::AI, "test operation", {
        let count = COUNTER.fetch_add(1, Ordering::SeqCst);
        println!("Code block executed, counter: {}", count + 1);
        84
    });

    println!("Result: {}, Counter after second call: {}", result2, COUNTER.load(Ordering::SeqCst));

    // Verify the counter is exactly 2 (proving single execution each time)
    assert_eq!(COUNTER.load(Ordering::SeqCst), 2, "Code block should execute exactly once per macro call");
    println!("✅ Test passed! Code block executed exactly once per macro call.");
}
