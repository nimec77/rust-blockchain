#!/usr/bin/env cargo run --bin cleanup_test_dbs --

//! Test Database Cleanup Utility
//! 
//! This utility finds and removes all leftover test database directories
//! that may remain after running tests. This is useful if tests are
//! interrupted or if cleanup fails for any reason.
//!
//! Usage: cargo run --bin cleanup_test_dbs

use std::fs;
use std::path::Path;
use std::{thread, time::Duration};

fn main() {
    println!("🧹 Rust Blockchain Test Database Cleanup Utility");
    println!("==============================================");
    println!();
    
    let cleaned_count = cleanup_test_databases();
    
    if cleaned_count > 0 {
        println!("🎉 Cleanup completed successfully!");
    } else {
        println!("✅ No cleanup needed - directory is already clean!");
    }
    
    println!();
    println!("You can run this utility anytime with:");
    println!("  cargo run --bin cleanup_test_dbs");
}

/// Main cleanup function that finds and removes test database directories
fn cleanup_test_databases() -> usize {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("❌ Error: Failed to get current directory: {e}");
            return 0;
        }
    };
    
    let mut cleaned_count = 0;
    
    let entries = match fs::read_dir(&current_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("❌ Error: Failed to read current directory: {e}");
            return 0;
        }
    };
    
    // Find all test database directories
    let test_dirs: Vec<(String, String)> = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_dir()
                && let Some(dir_name) = path.file_name().and_then(|n| n.to_str())
                    && dir_name.starts_with("test_db_") {
                        return Some((path.to_string_lossy().to_string(), dir_name.to_string()));
                    }
            None
        })
        .collect();
        
    if test_dirs.is_empty() {
        println!("✅ No test database directories found.");
    } else {
        println!("🗑️  Found {} test database director{} to clean up:", 
                test_dirs.len(), 
                if test_dirs.len() == 1 { "y" } else { "ies" });
        
        for (path_str, dir_name) in test_dirs {
            print!("   Removing: {dir_name} ... ");
            
            if robust_cleanup_test_db(&path_str) {
                println!("✅");
                cleaned_count += 1;
            } else {
                println!("⚠️");
            }
        }
        
        println!();
        println!("📊 Summary: Removed {} director{}", 
                cleaned_count, 
                if cleaned_count == 1 { "y" } else { "ies" });
    }
    
    cleaned_count
}

/// Robust cleanup function that handles locked databases and retries if necessary
fn robust_cleanup_test_db(test_path: &str) -> bool {
    if !Path::new(test_path).exists() {
        return true; // Already cleaned up
    }
    
    // Try cleanup multiple times with increasing delays to handle locked files
    for attempt in 0..10 {
        match fs::remove_dir_all(test_path) {
            Ok(()) => {
                return true; // Success
            }
            Err(e) => {
                // Check if it's a "file not found" error, which means cleanup succeeded
                if e.kind() == std::io::ErrorKind::NotFound {
                    return true;
                }
                
                if attempt == 9 {
                    // Final attempt failed
                    eprintln!();
                    eprintln!("      Warning: Failed to clean up '{}' after {} attempts: {}", 
                             test_path, attempt + 1, e);
                    eprintln!("      You may need to manually delete this directory.");
                    return false;
                } else {
                    // Wait with exponential backoff and try again
                    let delay = Duration::from_millis(50 * (1 << attempt)); // 50ms, 100ms, 200ms, etc.
                    thread::sleep(delay);
                }
            }
        }
    }
    
    false
} 
