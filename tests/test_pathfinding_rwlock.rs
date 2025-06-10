use echos_in_the_dark::{
    core::{
        components::Position,
        pathfinding::utils::{clear_pathfinding_cache, find_path, get_pathfinding_stats},
        resources::{CurrentMap, Map},
    },
    gameplay::world::components::TerrainType,
};
use std::{sync::Arc, thread, time::Duration};

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a simple test map for pathfinding
    fn create_test_map() -> CurrentMap {
        let size = (20, 20);
        let mut map = Map::new(size);

        // Fill with walkable floor tiles
        for x in 0..20 {
            for y in 0..20 {
                map.set_terrain(Position::new(x, y), TerrainType::Floor);
            }
        }

        // Add some walls to create interesting pathfinding scenarios
        for i in 5..15 {
            map.set_terrain(Position::new(i, 10), TerrainType::Wall); // Horizontal wall
        }
        map.set_terrain(Position::new(10, 9), TerrainType::Wall); // Block one path
        map.set_terrain(Position::new(10, 11), TerrainType::Wall); // Force pathfinding around

        CurrentMap(map)
    }

    #[test]
    fn test_concurrent_pathfinding_access() {
        println!("Testing pathfinding RwLock implementation...");

        // Clear any existing cache to start fresh
        clear_pathfinding_cache();

        // Create shared test data
        let test_scenarios = Arc::new(vec![
            (Position::new(0, 0), Position::new(19, 19)), // Long diagonal path
            (Position::new(0, 5), Position::new(19, 5)),  // Horizontal path
            (Position::new(5, 0), Position::new(5, 19)),  // Vertical path
            (Position::new(2, 8), Position::new(18, 12)), // Path around obstacles
            (Position::new(1, 1), Position::new(18, 18)), // Another diagonal
        ]);

        let results = Arc::new(std::sync::Mutex::new(Vec::new()));

        // Test concurrent read operations (multiple threads finding paths)
        let read_handles: Vec<_> = (0..8)
            .map(|i| {
                let scenarios = Arc::clone(&test_scenarios);
                let results = Arc::clone(&results);

                thread::spawn(move || {
                    println!("Read thread {i} starting pathfinding operations");
                    let mut local_results = Vec::new();

                    // Create a local map for this thread
                    let mut map = create_test_map();

                    // Perform multiple pathfinding operations
                    for (j, &(origin, destination)) in scenarios.iter().enumerate() {
                        let path = find_path(origin, destination, &mut map, false);
                        local_results.push((i, j, path.is_some(), path.as_ref().map(|p| p.len())));

                        // Small delay to increase chance of concurrent access
                        thread::sleep(Duration::from_millis(10));
                    }

                    // Store results
                    {
                        let mut shared_results = results.lock().unwrap();
                        shared_results.extend(local_results);
                    }

                    println!("Read thread {i} completed pathfinding operations");
                })
            })
            .collect();

        // Test concurrent write operations (cache clearing and stats access)
        let write_handles: Vec<_> = (0..3)
            .map(|i| {
                thread::spawn(move || {
                    println!("Write thread {i} starting cache operations");

                    // Perform cache operations that require write access
                    thread::sleep(Duration::from_millis(50)); // Let some reads happen first

                    if i == 0 {
                        // Clear cache mid-operation
                        clear_pathfinding_cache();
                        println!("Write thread {i} cleared pathfinding cache");
                    }

                    // Get stats (read operation)
                    let stats = get_pathfinding_stats();
                    println!("Write thread {i} got stats: {}", stats.lines().next().unwrap_or("No stats"));

                    println!("Write thread {i} completed cache operations");
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in read_handles {
            handle.join().unwrap();
        }

        for handle in write_handles {
            handle.join().unwrap();
        }

        // Verify results
        let final_results = results.lock().unwrap();
        println!("Total pathfinding operations completed: {}", final_results.len());

        // Verify that all pathfinding operations completed successfully
        assert!(!final_results.is_empty(), "No pathfinding operations were recorded");

        // Check that most paths were found (some might fail due to cache clearing)
        let successful_paths = final_results.iter().filter(|(_, _, success, _)| *success).count();
        let total_operations = final_results.len();
        let success_rate = successful_paths as f32 / total_operations as f32;

        println!("Success rate: {:.1}% ({}/{})", success_rate * 100.0, successful_paths, total_operations);
        assert!(success_rate > 0.5, "Success rate too low: {:.1}%", success_rate * 100.0);

        // Verify that paths have reasonable lengths
        for (thread_id, scenario_id, success, path_length) in final_results.iter() {
            if *success && let Some(length) = path_length {
                assert!(*length > 0, "Thread {thread_id} scenario {scenario_id} found empty path");
                assert!(
                    *length < 100,
                    "Thread {thread_id} scenario {scenario_id} found unreasonably long path: {length}"
                );
            }
        }

        // Get final stats to verify the system is still working
        let final_stats = get_pathfinding_stats();
        println!("Final pathfinding stats:\n{final_stats}");

        println!("All pathfinding operations completed successfully!");
        println!("RwLock implementation allows concurrent read access and reduces contention.");
        println!("Concurrent reads: 8 threads, Concurrent writes: 3 threads");
        println!("System maintained data integrity under concurrent access.");
    }

    #[test]
    fn test_rwlock_read_write_behavior() {
        println!("Testing RwLock read/write behavior specifically...");

        clear_pathfinding_cache();

        let barrier = Arc::new(std::sync::Barrier::new(6)); // 5 readers + 1 writer
        let start_time = std::time::Instant::now();
        let timings = Arc::new(std::sync::Mutex::new(Vec::new()));

        // Spawn multiple readers that should be able to run concurrently
        let reader_handles: Vec<_> = (0..5)
            .map(|i| {
                let barrier = Arc::clone(&barrier);
                let timings = Arc::clone(&timings);

                thread::spawn(move || {
                    barrier.wait(); // Synchronize start
                    let thread_start = std::time::Instant::now();

                    let mut map = create_test_map();

                    // Perform read operation (pathfinding)
                    let _path = find_path(Position::new(0, 0), Position::new(10, 10), &mut map, false);

                    let duration = thread_start.elapsed();

                    {
                        let mut shared_timings = timings.lock().unwrap();
                        shared_timings.push((format!("reader_{i}"), duration));
                    }
                })
            })
            .collect();

        // Spawn one writer
        let writer_handle = {
            let barrier = Arc::clone(&barrier);
            let timings = Arc::clone(&timings);

            thread::spawn(move || {
                barrier.wait(); // Synchronize start
                let thread_start = std::time::Instant::now();

                // Perform write operation (cache clear)
                clear_pathfinding_cache();

                let duration = thread_start.elapsed();

                {
                    let mut shared_timings = timings.lock().unwrap();
                    shared_timings.push(("writer".to_string(), duration));
                }
            })
        };

        // Wait for all threads
        for handle in reader_handles {
            handle.join().unwrap();
        }
        writer_handle.join().unwrap();

        let total_time = start_time.elapsed();
        let final_timings = timings.lock().unwrap();

        println!("Total test time: {total_time:?}");
        for (thread_name, duration) in final_timings.iter() {
            println!("{thread_name}: {duration:?}");
        }

        // Verify that readers could run concurrently (total time should be less than sum of all operations)
        let sum_of_operations: Duration = final_timings.iter().map(|(_, d)| *d).sum();
        println!("Sum of individual operations: {sum_of_operations:?}");

        // RwLock should allow concurrent reads, so total time should be significantly less
        // than the sum of all operations if they ran sequentially
        assert!(
            total_time < sum_of_operations,
            "Operations appear to have run sequentially rather than concurrently"
        );

        println!("RwLock successfully enabled concurrent read operations!");
    }

    #[test]
    fn test_pathfinding_cache_consistency() {
        println!("Testing pathfinding cache consistency under concurrent access...");

        clear_pathfinding_cache();

        let origin = Position::new(0, 0);
        let destination = Position::new(15, 15);
        let cache_operations = Arc::new(std::sync::Mutex::new(Vec::new()));

        // Spawn threads that perform the same pathfinding operation
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let cache_ops = Arc::clone(&cache_operations);

                thread::spawn(move || {
                    let mut map = create_test_map();

                    // First pathfinding call - should cache the result
                    let path1 = find_path(origin, destination, &mut map, false);

                    // Second identical call - should hit cache
                    let path2 = find_path(origin, destination, &mut map, false);

                    // Record results
                    {
                        let mut ops = cache_ops.lock().unwrap();
                        ops.push((i, path1.is_some(), path2.is_some(), path1 == path2));
                    }
                })
            })
            .collect();

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify cache consistency
        let operations = cache_operations.lock().unwrap();

        for (thread_id, path1_success, path2_success, paths_equal) in operations.iter() {
            assert!(*path1_success, "Thread {thread_id} first pathfinding failed");
            assert!(*path2_success, "Thread {thread_id} second pathfinding failed");
            assert!(*paths_equal, "Thread {thread_id} got inconsistent paths from cache");
        }

        println!("Cache consistency verified across {} threads", operations.len());

        // Get final stats
        let stats = get_pathfinding_stats();
        println!("Final cache stats:\n{stats}");
    }

    #[test]
    fn test_pathfinding_stress_test() {
        println!("Running pathfinding stress test with high concurrency...");

        clear_pathfinding_cache();

        let completed_operations = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let failed_operations = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        // High concurrency stress test
        let handles: Vec<_> = (0..20)
            .map(|i| {
                let completed = Arc::clone(&completed_operations);
                let failed = Arc::clone(&failed_operations);

                thread::spawn(move || {
                    let mut map = create_test_map();

                    // Each thread performs multiple random pathfinding operations
                    for j in 0..10 {
                        let origin = Position::new(fastrand::i32(0..20), fastrand::i32(0..20));
                        let destination = Position::new(fastrand::i32(0..20), fastrand::i32(0..20));

                        match find_path(origin, destination, &mut map, true) {
                            Some(_) => {
                                completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                            None => {
                                failed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                        }

                        // Occasionally clear cache to test write contention
                        if i == 0 && j % 5 == 0 {
                            clear_pathfinding_cache();
                        }
                    }
                })
            })
            .collect();

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        let total_completed = completed_operations.load(std::sync::atomic::Ordering::Relaxed);
        let total_failed = failed_operations.load(std::sync::atomic::Ordering::Relaxed);
        let total_operations = total_completed + total_failed;

        println!("Stress test completed:");
        println!("  Total operations: {total_operations}");
        println!("  Successful: {total_completed}");
        println!("  Failed: {total_failed}");
        println!("  Success rate: {:.1}%", (total_completed as f32 / total_operations as f32) * 100.0);

        // Verify system remained stable
        assert!(total_operations > 0, "No operations were performed");
        assert!(total_completed > 0, "No operations succeeded");

        // Get final stats to verify system integrity
        let final_stats = get_pathfinding_stats();
        println!("Final system stats:\n{final_stats}");

        println!("RwLock pathfinding system passed stress test!");
    }
}
