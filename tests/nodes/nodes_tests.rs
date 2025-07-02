use rust_blockchain::nodes::Nodes;
use std::sync::Arc;
use std::thread;

#[cfg(test)]
#[test]
/// Test Nodes::new() - creates a new empty Nodes instance
fn test_nodes_new() {
    let nodes = Nodes::new();
    assert_eq!(nodes.len(), 0);
    assert!(nodes.get_nodes().is_empty());
    assert!(nodes.first().is_none());
}

/// Test Nodes::add_node() - basic functionality
#[test]
fn test_add_node_single() {
    let nodes = Nodes::new();
    let addr = "127.0.0.1:8080".to_string();

    nodes.add_node(addr.clone());

    assert_eq!(nodes.len(), 1);
    assert!(nodes.node_is_known(&addr));
    assert_eq!(nodes.first().unwrap().get_addr(), addr);
}

/// Test Nodes::add_node() - multiple nodes
#[test]
fn test_add_node_multiple() {
    let nodes = Nodes::new();
    let addr1 = "127.0.0.1:8080".to_string();
    let addr2 = "192.168.1.1:3000".to_string();
    let addr3 = "localhost:9000".to_string();

    nodes.add_node(addr1.clone());
    nodes.add_node(addr2.clone());
    nodes.add_node(addr3.clone());

    assert_eq!(nodes.len(), 3);
    assert!(nodes.node_is_known(&addr1));
    assert!(nodes.node_is_known(&addr2));
    assert!(nodes.node_is_known(&addr3));

    let all_nodes = nodes.get_nodes();
    assert_eq!(all_nodes.len(), 3);
    assert!(all_nodes.iter().any(|node| node.get_addr() == addr1));
    assert!(all_nodes.iter().any(|node| node.get_addr() == addr2));
    assert!(all_nodes.iter().any(|node| node.get_addr() == addr3));
}

/// Test Nodes::add_node() - duplicate prevention
#[test]
fn test_add_node_duplicate_prevention() {
    let nodes = Nodes::new();
    let addr = "127.0.0.1:8080".to_string();

    nodes.add_node(addr.clone());
    nodes.add_node(addr.clone()); // Try to add the same node again
    nodes.add_node(addr.clone()); // And again

    assert_eq!(nodes.len(), 1); // Should still be only 1 node
    assert!(nodes.node_is_known(&addr));
}

/// Test Nodes::add_node() - empty string address
#[test]
fn test_add_node_empty_address() {
    let nodes = Nodes::new();
    let empty_addr = "".to_string();

    nodes.add_node(empty_addr.clone());

    assert_eq!(nodes.len(), 1);
    assert!(nodes.node_is_known(&empty_addr));
    assert_eq!(nodes.first().unwrap().get_addr(), empty_addr);
}

/// Test Nodes::evict_node() - basic functionality
#[test]
fn test_evict_node_existing() {
    let nodes = Nodes::new();
    let addr1 = "127.0.0.1:8080".to_string();
    let addr2 = "192.168.1.1:3000".to_string();

    nodes.add_node(addr1.clone());
    nodes.add_node(addr2.clone());
    assert_eq!(nodes.len(), 2);

    nodes.evict_node(&addr1);

    assert_eq!(nodes.len(), 1);
    assert!(!nodes.node_is_known(&addr1));
    assert!(nodes.node_is_known(&addr2));
}

/// Test Nodes::evict_node() - non-existent node
#[test]
fn test_evict_node_non_existent() {
    let nodes = Nodes::new();
    let addr = "127.0.0.1:8080".to_string();

    nodes.add_node(addr.clone());
    assert_eq!(nodes.len(), 1);

    // Try to evict a non-existent node
    nodes.evict_node("192.168.1.1:3000");

    assert_eq!(nodes.len(), 1); // Should remain unchanged
    assert!(nodes.node_is_known(&addr));
}

/// Test Nodes::evict_node() - from empty collection
#[test]
fn test_evict_node_from_empty() {
    let nodes = Nodes::new();

    // Try to evict from empty collection
    nodes.evict_node("127.0.0.1:8080");

    assert_eq!(nodes.len(), 0);
}

/// Test Nodes::evict_node() - all nodes
#[test]
fn test_evict_all_nodes() {
    let nodes = Nodes::new();
    let addr1 = "127.0.0.1:8080".to_string();
    let addr2 = "192.168.1.1:3000".to_string();

    nodes.add_node(addr1.clone());
    nodes.add_node(addr2.clone());
    assert_eq!(nodes.len(), 2);

    nodes.evict_node(&addr1);
    nodes.evict_node(&addr2);

    assert_eq!(nodes.len(), 0);
    assert!(!nodes.node_is_known(&addr1));
    assert!(!nodes.node_is_known(&addr2));
    assert!(nodes.first().is_none());
}

/// Test Nodes::first() - with nodes
#[test]
fn test_first_with_nodes() {
    let nodes = Nodes::new();
    let addr1 = "127.0.0.1:8080".to_string();
    let addr2 = "192.168.1.1:3000".to_string();

    nodes.add_node(addr1.clone());
    nodes.add_node(addr2.clone());

    let first_node = nodes.first().unwrap();
    assert_eq!(first_node.get_addr(), addr1); // First added should be first returned
}

/// Test Nodes::first() - empty collection
#[test]
fn test_first_empty_collection() {
    let nodes = Nodes::new();
    assert!(nodes.first().is_none());
}

/// Test Nodes::first() - after eviction
#[test]
fn test_first_after_eviction() {
    let nodes = Nodes::new();
    let addr1 = "127.0.0.1:8080".to_string();
    let addr2 = "192.168.1.1:3000".to_string();

    nodes.add_node(addr1.clone());
    nodes.add_node(addr2.clone());

    // Remove first node
    nodes.evict_node(&addr1);

    let first_node = nodes.first().unwrap();
    assert_eq!(first_node.get_addr(), addr2); // Second node should now be first
}

/// Test Nodes::get_nodes() - empty collection
#[test]
fn test_get_nodes_empty() {
    let nodes = Nodes::new();
    let all_nodes = nodes.get_nodes();
    assert!(all_nodes.is_empty());
}

/// Test Nodes::get_nodes() - returns clones
#[test]
fn test_get_nodes_returns_clones() {
    let nodes = Nodes::new();
    let addr = "127.0.0.1:8080".to_string();

    nodes.add_node(addr.clone());

    let nodes_vec = nodes.get_nodes();
    assert_eq!(nodes_vec.len(), 1);
    assert_eq!(nodes_vec[0].get_addr(), addr);

    // Verify it's a clone by checking we can drop the original
    drop(nodes);
    assert_eq!(nodes_vec[0].get_addr(), addr); // Should still work
}

/// Test Nodes::len() - various states
#[test]
fn test_len_various_states() {
    let nodes = Nodes::new();

    // Empty
    assert_eq!(nodes.len(), 0);

    // Add one
    nodes.add_node("127.0.0.1:8080".to_string());
    assert_eq!(nodes.len(), 1);

    // Add another
    nodes.add_node("192.168.1.1:3000".to_string());
    assert_eq!(nodes.len(), 2);

    // Try to add duplicate
    nodes.add_node("127.0.0.1:8080".to_string());
    assert_eq!(nodes.len(), 2); // Should remain 2

    // Remove one
    nodes.evict_node("127.0.0.1:8080");
    assert_eq!(nodes.len(), 1);

    // Remove last
    nodes.evict_node("192.168.1.1:3000");
    assert_eq!(nodes.len(), 0);
}

/// Test Nodes::node_is_known() - basic functionality
#[test]
fn test_node_is_known_basic() {
    let nodes = Nodes::new();
    let addr = "127.0.0.1:8080".to_string();

    // Not known initially
    assert!(!nodes.node_is_known(&addr));

    // Add and check
    nodes.add_node(addr.clone());
    assert!(nodes.node_is_known(&addr));

    // Remove and check
    nodes.evict_node(&addr);
    assert!(!nodes.node_is_known(&addr));
}

/// Test Nodes::node_is_known() - case sensitivity
#[test]
fn test_node_is_known_case_sensitivity() {
    let nodes = Nodes::new();
    let addr_lower = "localhost:8080".to_string();
    let addr_upper = "LOCALHOST:8080".to_string();

    nodes.add_node(addr_lower.clone());

    assert!(nodes.node_is_known(&addr_lower));
    assert!(!nodes.node_is_known(&addr_upper)); // Should be case sensitive
}

/// Test Nodes::node_is_known() - empty collection
#[test]
fn test_node_is_known_empty_collection() {
    let nodes = Nodes::new();
    assert!(!nodes.node_is_known("127.0.0.1:8080"));
    assert!(!nodes.node_is_known(""));
}

/// Test thread safety - concurrent access
#[test]
fn test_concurrent_access() {
    let nodes = Arc::new(Nodes::new());
    let mut handles = vec![];

    // Spawn multiple threads that add nodes
    for i in 0..10 {
        let nodes_clone: Arc<Nodes> = Arc::clone(&nodes);
        let handle = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", 8000 + i);
            nodes_clone.add_node(addr.clone());
            assert!(nodes_clone.node_is_known(&addr));
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Check final state
    assert_eq!(nodes.len(), 10);
    for i in 0..10 {
        let addr = format!("127.0.0.1:{}", 8000 + i);
        assert!(nodes.node_is_known(&addr));
    }
}

/// Test thread safety - concurrent add/evict
#[test]
fn test_concurrent_add_evict() {
    let nodes = Arc::new(Nodes::new());
    let mut handles = vec![];

    // Add some initial nodes
    for i in 0..5 {
        let addr = format!("127.0.0.1:{}", 8000 + i);
        nodes.add_node(addr);
    }

    // Spawn threads that add and evict nodes concurrently
    for i in 0..5 {
        let nodes_clone: Arc<Nodes> = Arc::clone(&nodes);
        let handle = thread::spawn(move || {
            let addr_to_add = format!("192.168.1.{i}:8080");
            let addr_to_evict = format!("127.0.0.1:{}", 8000 + i);

            nodes_clone.add_node(addr_to_add.clone());
            nodes_clone.evict_node(&addr_to_evict);

            assert!(nodes_clone.node_is_known(&addr_to_add));
            assert!(!nodes_clone.node_is_known(&addr_to_evict));
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Check final state
    assert_eq!(nodes.len(), 5); // Should have 5 nodes (evicted 5, added 5)
    for i in 0..5 {
        let addr_new = format!("192.168.1.{i}:8080");
        let addr_old = format!("127.0.0.1:{}", 8000 + i);
        assert!(nodes.node_is_known(&addr_new));
        assert!(!nodes.node_is_known(&addr_old));
    }
}

/// Integration test - full workflow
#[test]
fn test_full_workflow() {
    let nodes = Nodes::new();

    // Start with empty collection
    assert_eq!(nodes.len(), 0);
    assert!(nodes.get_nodes().is_empty());
    assert!(nodes.first().is_none());

    // Add multiple nodes
    let addresses = vec![
        "127.0.0.1:8080",
        "192.168.1.1:3000",
        "localhost:9000",
        "10.0.0.1:5000",
    ];

    for addr in &addresses {
        nodes.add_node(addr.to_string());
    }

    // Verify all added
    assert_eq!(nodes.len(), addresses.len());
    for addr in &addresses {
        assert!(nodes.node_is_known(addr));
    }

    // Check first node
    let first = nodes.first().unwrap();
    assert_eq!(first.get_addr(), addresses[0]);

    // Get all nodes and verify
    let all_nodes = nodes.get_nodes();
    assert_eq!(all_nodes.len(), addresses.len());
    for (i, node) in all_nodes.iter().enumerate() {
        assert_eq!(node.get_addr(), addresses[i]);
    }

    // Remove some nodes
    nodes.evict_node(addresses[0]);
    nodes.evict_node(addresses[2]);

    assert_eq!(nodes.len(), 2);
    assert!(!nodes.node_is_known(addresses[0]));
    assert!(nodes.node_is_known(addresses[1]));
    assert!(!nodes.node_is_known(addresses[2]));
    assert!(nodes.node_is_known(addresses[3]));

    // First should now be the second original node
    let first_after_eviction = nodes.first().unwrap();
    assert_eq!(first_after_eviction.get_addr(), addresses[1]);

    // Try to add duplicate
    nodes.add_node(addresses[1].to_string());
    assert_eq!(nodes.len(), 2); // Should remain 2

    // Clear all remaining
    nodes.evict_node(addresses[1]);
    nodes.evict_node(addresses[3]);

    assert_eq!(nodes.len(), 0);
    assert!(nodes.get_nodes().is_empty());
    assert!(nodes.first().is_none());
}

/// Edge case: Test with very long addresses
#[test]
fn test_long_addresses() {
    let nodes = Nodes::new();
    let long_addr = format!("{}:8080", "a".repeat(1000));

    nodes.add_node(long_addr.clone());

    assert_eq!(nodes.len(), 1);
    assert!(nodes.node_is_known(&long_addr));
    assert_eq!(nodes.first().unwrap().get_addr(), long_addr);
}

/// Edge case: Test with special characters in addresses
#[test]
fn test_special_characters() {
    let nodes = Nodes::new();
    let special_addrs = vec![
        "127.0.0.1:8080".to_string(),
        "[::1]:8080".to_string(), // IPv6
        "node-with-dash:3000".to_string(),
        "node_with_underscore:4000".to_string(),
    ];

    for addr in &special_addrs {
        nodes.add_node(addr.clone());
    }

    assert_eq!(nodes.len(), special_addrs.len());
    for addr in &special_addrs {
        assert!(nodes.node_is_known(addr));
    }
}
