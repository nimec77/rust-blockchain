use rust_blockchain::nodes::Node;

#[cfg(test)]
#[test]
fn test_node_new_with_valid_address() {
    let addr = "127.0.0.1:8080".to_string();
    let node = Node::new(addr.clone());
    assert_eq!(node.get_addr(), addr);
}

#[test]
fn test_node_new_with_ip_and_port() {
    let addr = "192.168.1.1:3000".to_string();
    let node = Node::new(addr.clone());
    assert_eq!(node.get_addr(), addr);
}

#[test]
fn test_node_new_with_localhost() {
    let addr = "localhost:8080".to_string();
    let node = Node::new(addr.clone());
    assert_eq!(node.get_addr(), addr);
}

#[test]
fn test_node_new_with_empty_string() {
    let addr = "".to_string();
    let node = Node::new(addr.clone());
    assert_eq!(node.get_addr(), addr);
}

#[test]
fn test_node_new_with_invalid_format() {
    let addr = "invalid-address".to_string();
    let node = Node::new(addr.clone());
    assert_eq!(node.get_addr(), addr);
}

/// Test the get_addr method
#[test]
fn test_get_addr_returns_correct_address() {
    let addr = "127.0.0.1:8080".to_string();
    let node = Node::new(addr.clone());
    assert_eq!(node.get_addr(), addr);
}

#[test]
fn test_get_addr_returns_clone() {
    let addr = "127.0.0.1:8080".to_string();
    let node = Node::new(addr.clone());
    let returned_addr = node.get_addr();

    // Ensure we get the same value
    assert_eq!(returned_addr, addr);

    // Verify it's a clone by checking we can modify the original without affecting the returned value
    drop(addr); // This should not affect returned_addr if it's truly a clone
    assert_eq!(returned_addr, "127.0.0.1:8080");
}

#[test]
fn test_get_addr_with_different_formats() {
    let test_cases = vec![
        "127.0.0.1:8080",
        "localhost:3000",
        "192.168.1.100:9000",
        "0.0.0.0:80",
        "[::1]:8080", // IPv6
    ];

    for addr_str in test_cases {
        let node = Node::new(addr_str.to_string());
        assert_eq!(node.get_addr(), addr_str.to_string());
    }
}

/// Test the parse_socket_addr method with valid addresses
#[test]
fn test_parse_socket_addr_with_valid_ipv4() {
    let addr = "127.0.0.1:8080".to_string();
    let node = Node::new(addr);
    let socket_addr = node.parse_socket_addr();

    assert_eq!(socket_addr.ip().to_string(), "127.0.0.1");
    assert_eq!(socket_addr.port(), 8080);
}

#[test]
fn test_parse_socket_addr_with_valid_ipv6() {
    let addr = "[::1]:8080".to_string();
    let node = Node::new(addr);
    let socket_addr = node.parse_socket_addr();

    assert_eq!(socket_addr.port(), 8080);
    assert!(socket_addr.is_ipv6());
}

#[test]
fn test_parse_socket_addr_with_zero_ip() {
    let addr = "0.0.0.0:80".to_string();
    let node = Node::new(addr);
    let socket_addr = node.parse_socket_addr();

    assert_eq!(socket_addr.ip().to_string(), "0.0.0.0");
    assert_eq!(socket_addr.port(), 80);
}

#[test]
fn test_parse_socket_addr_with_high_port() {
    let addr = "127.0.0.1:65535".to_string();
    let node = Node::new(addr);
    let socket_addr = node.parse_socket_addr();

    assert_eq!(socket_addr.ip().to_string(), "127.0.0.1");
    assert_eq!(socket_addr.port(), 65535);
}

/// Test the parse_socket_addr method with invalid addresses (should panic)
#[test]
#[should_panic]
fn test_parse_socket_addr_with_invalid_ip() {
    let addr = "999.999.999.999:8080".to_string();
    let node = Node::new(addr);
    node.parse_socket_addr(); // This should panic
}

#[test]
#[should_panic]
fn test_parse_socket_addr_with_invalid_port() {
    let addr = "127.0.0.1:99999".to_string();
    let node = Node::new(addr);
    node.parse_socket_addr(); // This should panic
}

#[test]
#[should_panic]
fn test_parse_socket_addr_with_no_port() {
    let addr = "127.0.0.1".to_string();
    let node = Node::new(addr);
    node.parse_socket_addr(); // This should panic
}

#[test]
#[should_panic]
fn test_parse_socket_addr_with_empty_string() {
    let addr = "".to_string();
    let node = Node::new(addr);
    node.parse_socket_addr(); // This should panic
}

#[test]
#[should_panic]
fn test_parse_socket_addr_with_invalid_format() {
    let addr = "not-an-address".to_string();
    let node = Node::new(addr);
    node.parse_socket_addr(); // This should panic
}

#[test]
#[should_panic]
fn test_parse_socket_addr_with_localhost_only() {
    let addr = "localhost".to_string();
    let node = Node::new(addr);
    node.parse_socket_addr(); // This should panic (no port)
}

/// Test Node clone functionality (since it derives Clone)
#[test]
fn test_node_clone() {
    let addr = "127.0.0.1:8080".to_string();
    let node1 = Node::new(addr.clone());
    let node2 = node1.clone();

    assert_eq!(node1.get_addr(), node2.get_addr());
    assert_eq!(node1.get_addr(), addr);
    assert_eq!(node2.get_addr(), addr);
}

/// Integration test combining multiple methods
#[test]
fn test_node_full_workflow() {
    let addr = "192.168.1.50:9000".to_string();

    // Create node
    let node = Node::new(addr.clone());

    // Verify address
    assert_eq!(node.get_addr(), addr);

    // Parse socket address
    let socket_addr = node.parse_socket_addr();
    assert_eq!(socket_addr.ip().to_string(), "192.168.1.50");
    assert_eq!(socket_addr.port(), 9000);

    // Test clone
    let cloned_node = node.clone();
    assert_eq!(cloned_node.get_addr(), addr);

    let cloned_socket_addr = cloned_node.parse_socket_addr();
    assert_eq!(cloned_socket_addr.ip().to_string(), "192.168.1.50");
    assert_eq!(cloned_socket_addr.port(), 9000);
}

/// Performance test to ensure methods are efficient
#[test]
fn test_node_performance() {
    let addr = "127.0.0.1:8080".to_string();
    let node = Node::new(addr.clone());

    // Test multiple calls to get_addr
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = node.get_addr();
    }
    let duration = start.elapsed();
    assert!(duration.as_millis() < 100, "get_addr should be fast");

    // Test multiple calls to parse_socket_addr
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = node.parse_socket_addr();
    }
    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 500,
        "parse_socket_addr should be reasonably fast"
    );
}

/// Test with various edge cases
#[test]
fn test_node_edge_cases() {
    // Test with minimum port
    let node = Node::new("127.0.0.1:0".to_string());
    let socket_addr = node.parse_socket_addr();
    assert_eq!(socket_addr.port(), 0);

    // Test with IPv4 loopback
    let node = Node::new("127.0.0.1:1234".to_string());
    let socket_addr = node.parse_socket_addr();
    assert!(socket_addr.ip().is_loopback());

    // Test with unspecified IPv4
    let node = Node::new("0.0.0.0:5678".to_string());
    let socket_addr = node.parse_socket_addr();
    assert!(socket_addr.ip().is_unspecified());
}
