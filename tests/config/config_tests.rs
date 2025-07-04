use std::{env, sync::Arc, thread};
use rust_blockchain::config::{Config, DEFAULT_NODE_ADDR};

#[test]
fn test_config_new_with_default_node_address() {
    // Store original value and ensure no NODE_ADDRESS env var is set for this test
    let original = env::var("NODE_ADDRESS").ok();
    unsafe { env::remove_var("NODE_ADDRESS"); }
    
    let config = Config::new();
    let node_addr = config.get_node_addr();
    
    assert_eq!(node_addr, DEFAULT_NODE_ADDR);
    assert!(!config.is_miner()); // Should not be a miner initially
    assert!(config.get_mining_addr().is_none()); // No mining address initially
    
    // Restore original value
    if let Some(val) = original {
        unsafe { env::set_var("NODE_ADDRESS", val); }
    }
}

#[test]
fn test_config_new_with_env_variable() {
    // Store original value
    let original = env::var("NODE_ADDRESS").ok();
    
    let test_addr = "192.168.1.100:3000";
    unsafe { env::set_var("NODE_ADDRESS", test_addr); }
    
    let config = Config::new();
    let node_addr = config.get_node_addr();
    
    assert_eq!(node_addr, test_addr);
    
    // Restore original value
    match original {
        Some(val) => unsafe { env::set_var("NODE_ADDRESS", val); },
        None => unsafe { env::remove_var("NODE_ADDRESS"); },
    }
}

#[test]
fn test_config_default_trait() {
    // Store original value and ensure no NODE_ADDRESS env var is set for this test
    let original = env::var("NODE_ADDRESS").ok();
    unsafe { env::remove_var("NODE_ADDRESS"); }
    
    let config = Config::default();
    let node_addr = config.get_node_addr();
    
    assert_eq!(node_addr, DEFAULT_NODE_ADDR);
    assert!(!config.is_miner());
    
    // Restore original value
    if let Some(val) = original {
        unsafe { env::set_var("NODE_ADDRESS", val); }
    }
}

#[test]
fn test_set_and_get_mining_address() {
    let config = Config::new();
    let mining_addr = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    
    // Initially no mining address
    assert!(config.get_mining_addr().is_none());
    assert!(!config.is_miner());
    
    // Set mining address
    config.set_mining_addr(mining_addr.to_string());
    
    // Verify mining address is set
    assert_eq!(config.get_mining_addr(), Some(mining_addr.to_string()));
    assert!(config.is_miner());
}

#[test]
fn test_set_mining_address_multiple_times() {
    let config = Config::new();
    let first_addr = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let second_addr = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
    
    // Set first mining address
    config.set_mining_addr(first_addr.to_string());
    assert_eq!(config.get_mining_addr(), Some(first_addr.to_string()));
    assert!(config.is_miner());
    
    // Override with second mining address
    config.set_mining_addr(second_addr.to_string());
    assert_eq!(config.get_mining_addr(), Some(second_addr.to_string()));
    assert!(config.is_miner());
}

#[test]
fn test_set_empty_mining_address() {
    let config = Config::new();
    
    // Set empty mining address
    config.set_mining_addr(String::new());
    
    // Should still be considered a miner (key exists, even if empty)
    assert_eq!(config.get_mining_addr(), Some(String::new()));
    assert!(config.is_miner());
}

#[test]
fn test_is_miner_logic() {
    let config = Config::new();
    
    // Initially not a miner
    assert!(!config.is_miner());
    
    // Becomes a miner after setting mining address
    config.set_mining_addr("test_address".to_string());
    assert!(config.is_miner());
}

#[test]
fn test_thread_safety_concurrent_reads() {
    let config = Arc::new(Config::new());
    let mut handles = vec![];
    
    // Spawn multiple threads to read node address concurrently
    for _ in 0..10 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let _addr = config_clone.get_node_addr();
            let _is_miner = config_clone.is_miner();
            let _mining_addr = config_clone.get_mining_addr();
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should not panic");
    }
}

#[test]
fn test_thread_safety_concurrent_writes() {
    let config = Arc::new(Config::new());
    let mut handles = vec![];
    
    // Spawn multiple threads to write mining addresses concurrently
    for i in 0..5 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let addr = format!("mining_address_{i}");
            config_clone.set_mining_addr(addr);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should not panic");
    }
    
    // Verify config is still in a valid state
    assert!(config.is_miner());
    assert!(config.get_mining_addr().is_some());
}

#[test]
fn test_thread_safety_mixed_operations() {
    let config = Arc::new(Config::new());
    let mut handles = vec![];
    
    // Spawn threads doing mixed read/write operations
    for i in 0..10 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            if i % 2 == 0 {
                // Read operations
                let _addr = config_clone.get_node_addr();
                let _is_miner = config_clone.is_miner();
                let _mining_addr = config_clone.get_mining_addr();
            } else {
                // Write operations
                let addr = format!("addr_{i}");
                config_clone.set_mining_addr(addr);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should not panic");
    }
    
    // Verify final state
    assert!(config.is_miner());
    let final_addr = config.get_mining_addr();
    assert!(final_addr.is_some());
    assert!(final_addr.unwrap().starts_with("addr_"));
}

#[test]
fn test_node_address_immutable_after_creation() {
    // Store original value
    let original = env::var("NODE_ADDRESS").ok();
    
    let test_addr = "10.0.0.1:4000";
    unsafe { env::set_var("NODE_ADDRESS", test_addr); }
    
    let config = Config::new();
    assert_eq!(config.get_node_addr(), test_addr);
    
    // Change env var after config creation
    unsafe { env::set_var("NODE_ADDRESS", "different.addr:5000"); }
    
    // Config should still return the original address
    assert_eq!(config.get_node_addr(), test_addr);
    
    // Restore original value
    match original {
        Some(val) => unsafe { env::set_var("NODE_ADDRESS", val); },
        None => unsafe { env::remove_var("NODE_ADDRESS"); },
    }
}

#[test]
fn test_config_state_consistency() {
    let config = Config::new();
    
    // Test consistency between is_miner() and get_mining_addr()
    assert_eq!(config.is_miner(), config.get_mining_addr().is_some());
    
    config.set_mining_addr("test".to_string());
    assert_eq!(config.is_miner(), config.get_mining_addr().is_some());
    
    // Multiple reads should be consistent
    for _ in 0..5 {
        assert_eq!(config.is_miner(), config.get_mining_addr().is_some());
        assert_eq!(config.get_mining_addr(), Some("test".to_string()));
    }
}

#[test]
fn test_special_characters_in_addresses() {
    let config = Config::new();
    
    // Test various special characters in mining address
    let special_addrs = vec![
        "!@#$%^&*()_+-={}[]|\\:;\"'<>?,.`~",
        "address with spaces",
        "address\nwith\nnewlines",
        "address\twith\ttabs",
        "unicode_address_🚀_test",
        "",
    ];
    
    for addr in special_addrs {
        config.set_mining_addr(addr.to_string());
        assert_eq!(config.get_mining_addr(), Some(addr.to_string()));
        assert!(config.is_miner());
    }
}

#[test]
fn test_environment_variable_priority() {
    // Store original value
    let original = env::var("NODE_ADDRESS").ok();
    
    // Test that environment variable takes precedence over default
    let custom_addr = "env.test.addr:9999";
    unsafe { env::set_var("NODE_ADDRESS", custom_addr); }
    
    let config = Config::new();
    assert_eq!(config.get_node_addr(), custom_addr);
    assert_ne!(config.get_node_addr(), DEFAULT_NODE_ADDR);
    
    // Restore original value
    match original {
        Some(val) => unsafe { env::set_var("NODE_ADDRESS", val); },
        None => unsafe { env::remove_var("NODE_ADDRESS"); },
    }
} 
