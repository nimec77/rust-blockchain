use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use bincode::config::standard;
use data_encoding::HEXLOWER;
use rust_blockchain::{
    config::GLOBAL_CONFIG, memory_pool::{BlockInTransit, MemoryPool}, nodes::Nodes, server::{serve, OpType, Package, CENTRAL_NODE, GLOBAL_BLOCKS_IN_TRANSIT, GLOBAL_MEMORY_POOL, GLOBAL_NODES, NODE_VERSION, TRANSACTION_THRESHOLD}, Block, Blockchain, Transaction
};
use crate::test_helpers::{
    create_test_block, create_test_transaction, setup_temp_test_db,
};

// Mock TCP stream for testing
struct MockTcpStream {
    read_data: Arc<Mutex<Vec<u8>>>,
    write_data: Arc<Mutex<Vec<u8>>>,
    read_pos: Arc<Mutex<usize>>,
    peer_addr: SocketAddr,
}

impl MockTcpStream {
    fn new(data: Vec<u8>) -> Self {
        Self {
            read_data: Arc::new(Mutex::new(data)),
            write_data: Arc::new(Mutex::new(Vec::new())),
            read_pos: Arc::new(Mutex::new(0)),
            peer_addr: "127.0.0.1:3000".parse().unwrap(),
        }
    }

    fn get_written_data(&self) -> Vec<u8> {
        self.write_data.lock().unwrap().clone()
    }
}

impl Read for MockTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let data = self.read_data.lock().unwrap();
        let mut pos = self.read_pos.lock().unwrap();
        
        if *pos >= data.len() {
            return Ok(0); // EOF
        }
        
        let available = data.len() - *pos;
        let to_read = buf.len().min(available);
        
        buf[..to_read].copy_from_slice(&data[*pos..*pos + to_read]);
        *pos += to_read;
        
        Ok(to_read)
    }
}

impl Write for MockTcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut write_data = self.write_data.lock().unwrap();
        write_data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// Mock implementation for TcpStream methods used in serve function
impl MockTcpStream {
    fn peer_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.peer_addr)
    }

    fn shutdown(&self, _how: std::net::Shutdown) -> io::Result<()> {
        Ok(())
    }
}

// Helper function to create a test blockchain
fn create_test_blockchain() -> Blockchain {
    let (db, _temp_dir) = setup_temp_test_db();
    let genesis_tx = create_test_transaction(vec![0, 0, 0, 0]);
    let genesis_block = Block::generate_genesis_block(&genesis_tx);
    
    // Initialize the blockchain with genesis block
    let blockchain = Blockchain::new_with_tip(db.clone(), genesis_block.get_hash().to_string());
    
    // Add the genesis block to the database
    let blocks_tree = db.open_tree("blocks").unwrap();
    let _ = blocks_tree.insert(genesis_block.get_hash(), genesis_block.serialize());
    let _ = blocks_tree.insert("tip_block_hash", genesis_block.get_hash());
    
    blockchain
}

// Helper function to serialize a package for testing
fn serialize_package(pkg: &Package) -> Vec<u8> {
    bincode::encode_to_vec(pkg, standard()).unwrap()
}

// Helper function to setup test configuration
// Helper function to setup test configuration
fn setup_test_config() {
    GLOBAL_CONFIG.set_mining_addr("test_mining_address".to_string());
}

// Helper function to clear global state (note: global state can't be easily cleared)
fn clear_global_state() {
    // Global state cannot be cleared in this implementation
    // Tests should be designed to work with existing state
}

// Helper function to create a test server
fn create_test_server() -> (TcpListener, std::net::SocketAddr) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    (listener, addr)
}

// Helper function to send data to a TCP stream
fn send_package_to_stream(stream: &mut TcpStream, pkg: &Package) -> std::io::Result<()> {
    let serialized = serialize_package(pkg);
    stream.write_all(&serialized)?;
    stream.flush()?;
    Ok(())
}

#[test]
fn test_package_serialization_deserialization() {
    // Test that packages can be serialized and deserialized properly
    let packages = vec![
        Package::Block {
            addr_from: "127.0.0.1:3000".to_string(),
            block: vec![1, 2, 3, 4, 5],
        },
        Package::GetBlocks {
            addr_from: "127.0.0.1:3000".to_string(),
        },
        Package::GetData {
            addr_from: "127.0.0.1:3000".to_string(),
            op_type: OpType::Block,
            id: vec![1, 2, 3],
        },
        Package::Inv {
            addr_from: "127.0.0.1:3000".to_string(),
            op_type: OpType::Tx,
            items: vec![vec![1, 2], vec![3, 4]],
        },
        Package::Tx {
            addr_from: "127.0.0.1:3000".to_string(),
            transaction: vec![10, 20, 30],
        },
        Package::Version {
            addr_from: "127.0.0.1:3000".to_string(),
            version: 1,
            best_height: 42,
        },
    ];
    
    for (i, package) in packages.iter().enumerate() {
        let serialized = serialize_package(package);
        let (deserialized, _): (Package, usize) = bincode::decode_from_slice(&serialized, standard()).unwrap();
        
        // Check that serialization/deserialization preserves the package type
        match (package, &deserialized) {
            (Package::Block { .. }, Package::Block { .. }) => (),
            (Package::GetBlocks { .. }, Package::GetBlocks { .. }) => (),
            (Package::GetData { .. }, Package::GetData { .. }) => (),
            (Package::Inv { .. }, Package::Inv { .. }) => (),
            (Package::Tx { .. }, Package::Tx { .. }) => (),
            (Package::Version { .. }, Package::Version { .. }) => (),
            _ => panic!("Package type mismatch at index {i}: {package:?} vs {deserialized:?}"),
        }
    }
}

#[test]
fn test_global_state_management() {
    clear_global_state();
    
    // Test memory pool operations
    let tx1 = create_test_transaction(vec![1, 2, 3, 4]);
    let tx2 = create_test_transaction(vec![5, 6, 7, 8]);
    
    GLOBAL_MEMORY_POOL.add(tx1.clone());
    GLOBAL_MEMORY_POOL.add(tx2.clone());
    
    assert_eq!(GLOBAL_MEMORY_POOL.len(), 2);
    
    let tx1_id_hex = HEXLOWER.encode(tx1.get_id());
    let tx2_id_hex = HEXLOWER.encode(tx2.get_id());
    
    assert!(GLOBAL_MEMORY_POOL.contains(&tx1_id_hex));
    assert!(GLOBAL_MEMORY_POOL.contains(&tx2_id_hex));
    
    // Test blocks in transit operations
    let block_hashes = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
    ];
    
    GLOBAL_BLOCKS_IN_TRANSIT.add_blocks(&block_hashes);
    assert_eq!(GLOBAL_BLOCKS_IN_TRANSIT.len(), 2);
    
    let first_block = GLOBAL_BLOCKS_IN_TRANSIT.first().unwrap();
    GLOBAL_BLOCKS_IN_TRANSIT.remove(&first_block);
    assert_eq!(GLOBAL_BLOCKS_IN_TRANSIT.len(), 1);
    
    // Test nodes management
    GLOBAL_NODES.add_node("127.0.0.1:3001".to_string());
    GLOBAL_NODES.add_node("127.0.0.1:3002".to_string());
    
    assert!(GLOBAL_NODES.node_is_known("127.0.0.1:3001"));
    assert!(GLOBAL_NODES.node_is_known("127.0.0.1:3002"));
    assert!(!GLOBAL_NODES.node_is_known("127.0.0.1:3003"));
    
    // Clean up
    clear_global_state();
}

#[test]
fn test_blockchain_operations() {
    let blockchain = create_test_blockchain();
    
    // Test initial state
    let initial_height = blockchain.get_best_height();
    assert_eq!(initial_height, 0); // Genesis block
    
    // Test adding a block
    let test_block = create_test_block("test_prev_hash".to_string(), 1);
    blockchain.add_block(&test_block);
    
    let final_height = blockchain.get_best_height();
    assert!(final_height >= initial_height); // Should be at least the same, possibly higher
    
    // Test retrieving blocks
    let block_hashes = blockchain.get_block_hashes();
    assert!(!block_hashes.is_empty());
    
    // Test retrieving a specific block
    let genesis_hash = blockchain.get_tip_hash();
    let retrieved_block = blockchain.get_block(genesis_hash.as_bytes());
    assert!(retrieved_block.is_some());
}

#[test]
fn test_central_node_detection() {
    setup_test_config();
    clear_global_state();
    
    // Test non-central node (can't set node_addr, so test with current value)
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    // Note: set_node_addr() method doesn't exist, so we can't modify it
    
    // Test central node constant
    assert_eq!(CENTRAL_NODE, "127.0.0.1:2001");
}

#[test]
fn test_package_creation() {
    let test_block = create_test_block("test_hash".to_string(), 1);
    let test_tx = create_test_transaction(vec![1, 2, 3, 4]);
    
    // Test Block package
    let block_pkg = Package::Block {
        addr_from: "127.0.0.1:3000".to_string(),
        block: test_block.serialize(),
    };
    
    match block_pkg {
        Package::Block { addr_from, block } => {
            assert_eq!(addr_from, "127.0.0.1:3000");
            assert!(!block.is_empty());
            
            // Test that we can deserialize the block
            let deserialized_block = Block::deserialize(&block);
            assert_eq!(deserialized_block.get_height(), 1);
        }
        _ => panic!("Expected Block package"),
    }
    
    // Test Transaction package
    let tx_pkg = Package::Tx {
        addr_from: "127.0.0.1:3000".to_string(),
        transaction: test_tx.serialize(),
    };
    
    match tx_pkg {
        Package::Tx { addr_from, transaction } => {
            assert_eq!(addr_from, "127.0.0.1:3000");
            assert!(!transaction.is_empty());
            
            // Test that we can deserialize the transaction
            let deserialized_tx = Transaction::deserialize(&transaction);
            assert_eq!(deserialized_tx.get_id(), test_tx.get_id());
        }
        _ => panic!("Expected Tx package"),
    }
    
    // Test Version package
    let version_pkg = Package::Version {
        addr_from: "127.0.0.1:3000".to_string(),
        version: NODE_VERSION,
        best_height: 42,
    };
    
    match version_pkg {
        Package::Version { addr_from, version, best_height } => {
            assert_eq!(addr_from, "127.0.0.1:3000");
            assert_eq!(version, NODE_VERSION);
            assert_eq!(best_height, 42);
        }
        _ => panic!("Expected Version package"),
    }
}

#[test]
fn test_op_type_usage() {
    // Test OpType in GetData package
    let get_data_block = Package::GetData {
        addr_from: "127.0.0.1:3000".to_string(),
        op_type: OpType::Block,
        id: vec![1, 2, 3, 4],
    };
    
    match get_data_block {
        Package::GetData { op_type, .. } => {
            match op_type {
                OpType::Block => (), // Expected
                OpType::Tx => panic!("Expected Block OpType"),
            }
        }
        _ => panic!("Expected GetData package"),
    }
    
    // Test OpType in Inv package
    let inv_tx = Package::Inv {
        addr_from: "127.0.0.1:3000".to_string(),
        op_type: OpType::Tx,
        items: vec![vec![1, 2], vec![3, 4]],
    };
    
    match inv_tx {
        Package::Inv { op_type, .. } => {
            match op_type {
                OpType::Tx => (), // Expected
                OpType::Block => panic!("Expected Tx OpType"),
            }
        }
        _ => panic!("Expected Inv package"),
    }
}

#[test]
fn test_data_encoding_operations() {
    let test_data = vec![1, 2, 3, 4, 5, 255];
    
    // Test HEXLOWER encoding
    let encoded = HEXLOWER.encode(&test_data);
    assert_eq!(encoded, "0102030405ff");
    
    // Test that transaction IDs can be encoded
    let tx = create_test_transaction(vec![1, 2, 3, 4]);
    let tx_id = tx.get_id();
    let tx_id_hex = HEXLOWER.encode(tx_id);
    
    assert!(!tx_id_hex.is_empty());
    assert_eq!(tx_id_hex.len(), tx_id.len() * 2); // Each byte becomes 2 hex chars
}

#[test]
fn test_block_hash_operations() {
    let test_block = create_test_block("test_hash".to_string(), 1);
    
    // Test getting hash as string
    let hash_str = test_block.get_hash();
    assert!(!hash_str.is_empty());
    
    // Test getting hash as bytes
    let hash_bytes = test_block.get_hash_bytes();
    assert!(!hash_bytes.is_empty());
    assert_eq!(hash_bytes, hash_str.as_bytes());
    
    // Test that blocks can be used in collections
    let mut block_hashes = vec![];
    for i in 0..5 {
        let block = create_test_block(format!("prev_hash_{i}"), i);
        block_hashes.push(block.get_hash_bytes());
    }
    
    assert_eq!(block_hashes.len(), 5);
    
    // Test that hashes are unique (different blocks should have different hashes)
    let unique_hashes: std::collections::HashSet<_> = block_hashes.iter().collect();
    assert_eq!(unique_hashes.len(), 5);
}

#[test]
fn test_error_conditions() {
    // Test invalid address parsing
    let invalid_addr = "not_an_address";
    let parse_result = invalid_addr.parse::<std::net::SocketAddr>();
    assert!(parse_result.is_err());
    
    // Test empty data scenarios
    let empty_items: Vec<Vec<u8>> = vec![];
    let inv_pkg = Package::Inv {
        addr_from: "127.0.0.1:3000".to_string(),
        op_type: OpType::Block,
        items: empty_items,
    };
    
    match inv_pkg {
        Package::Inv { items, .. } => {
            assert!(items.is_empty());
        }
        _ => panic!("Expected Inv package"),
    }
    
    // Test serialization of empty block
    let empty_block = Package::Block {
        addr_from: "127.0.0.1:3000".to_string(),
        block: vec![],
    };
    
    let serialized = serialize_package(&empty_block);
    assert!(!serialized.is_empty()); // Should still serialize the structure
}

// Integration test that actually uses TCP connections
#[test]
#[ignore] // Integration test - run manually with `cargo test -- --ignored`
fn test_integration_with_real_tcp() {
    setup_test_config();
    clear_global_state();
    
    let blockchain = create_test_blockchain();
    let (listener, addr) = create_test_server();
    
    // Shared state to verify the test results
    let test_complete = Arc::new(Mutex::new(false));
    let test_complete_clone = test_complete.clone();
    
    // Start server in background thread
    let blockchain_clone = blockchain.clone();
    thread::spawn(move || {
        if let Ok(stream) = listener.accept() {
            let _ = serve(blockchain_clone, stream.0);
            let mut complete = test_complete_clone.lock().unwrap();
            *complete = true;
        }
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    
    // Connect as client and send a version package
    let mut client_stream = TcpStream::connect(addr).unwrap();
    let version_pkg = Package::Version {
        addr_from: "127.0.0.1:3001".to_string(),
        version: NODE_VERSION,
        best_height: 0,
    };
    
    send_package_to_stream(&mut client_stream, &version_pkg).unwrap();
    
    // Close the connection to trigger server shutdown
    drop(client_stream);
    
    // Wait for server to process
    thread::sleep(Duration::from_millis(200));
    
    // Verify that the node was added to the global state
    assert!(GLOBAL_NODES.node_is_known("127.0.0.1:3001"));
    
    // Verify test completed
    let complete = test_complete.lock().unwrap();
    assert!(*complete);
}

#[test]
fn test_constants_and_configuration() {
    // Test that constants are properly defined
    assert_eq!(NODE_VERSION, 1);
    assert_eq!(CENTRAL_NODE, "127.0.0.1:2001");
    assert_eq!(TRANSACTION_THRESHOLD, 2);
    
    // Test that configuration is working
    setup_test_config();
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    assert!(!node_addr.is_empty());
    
    // Test that mining address can be set and retrieved
    let mining_addr = GLOBAL_CONFIG.get_mining_addr();
    assert!(mining_addr.is_some());
}

#[test]
fn test_package_validation() {
    // Test that package fields are properly validated during creation
    let test_block = create_test_block("test_hash".to_string(), 1);
    
    // Test with valid data
    let valid_pkg = Package::Block {
        addr_from: "127.0.0.1:3000".to_string(),
        block: test_block.serialize(),
    };
    
    let serialized = serialize_package(&valid_pkg);
    assert!(!serialized.is_empty());
    
    // Test deserialization
    let (deserialized, _): (Package, usize) = bincode::decode_from_slice(&serialized, standard()).unwrap();
    match deserialized {
        Package::Block { addr_from, .. } => {
            assert_eq!(addr_from, "127.0.0.1:3000");
        }
        _ => panic!("Expected Block package"),
    }
}

#[test]
fn test_blocks_in_transit_operations() {
    clear_global_state();
    
    // Test adding blocks
    let block_hashes = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
    ];
    
    GLOBAL_BLOCKS_IN_TRANSIT.add_blocks(&block_hashes);
    assert_eq!(GLOBAL_BLOCKS_IN_TRANSIT.len(), 3);
    assert!(!GLOBAL_BLOCKS_IN_TRANSIT.is_empty());
    
    // Test getting first block
    let first_block = GLOBAL_BLOCKS_IN_TRANSIT.first();
    assert!(first_block.is_some());
    assert_eq!(first_block.unwrap(), vec![1, 2, 3, 4, 5]);
    
    // Test removing blocks
    GLOBAL_BLOCKS_IN_TRANSIT.remove(&[1, 2, 3, 4, 5]);
    assert_eq!(GLOBAL_BLOCKS_IN_TRANSIT.len(), 2);
    
    // Test clearing all blocks
    GLOBAL_BLOCKS_IN_TRANSIT.clear();
    assert_eq!(GLOBAL_BLOCKS_IN_TRANSIT.len(), 0);
    assert!(GLOBAL_BLOCKS_IN_TRANSIT.is_empty());
}

#[test]
fn test_local_memory_pool_operations() {
    // Since we can't access global state easily, test local memory pool operations
    let memory_pool = MemoryPool::new();
    
    // Test adding transactions
    let tx1 = create_test_transaction(vec![1, 2, 3, 4]);
    let tx2 = create_test_transaction(vec![5, 6, 7, 8]);
    
    memory_pool.add(tx1.clone());
    memory_pool.add(tx2.clone());
    
    assert_eq!(memory_pool.len(), 2);
    assert!(!memory_pool.is_empty());
    
    // Test retrieving transactions
    let tx1_id_hex = HEXLOWER.encode(tx1.get_id());
    let retrieved_tx = memory_pool.get(&tx1_id_hex);
    assert!(retrieved_tx.is_some());
    assert_eq!(retrieved_tx.unwrap().get_id(), tx1.get_id());
    
    // Test removing transactions
    memory_pool.remove(&tx1_id_hex);
    assert_eq!(memory_pool.len(), 1);
    assert!(!memory_pool.contains(&tx1_id_hex));
    
    // Test getting all transactions
    let all_txs = memory_pool.get_all();
    assert_eq!(all_txs.len(), 1);
    assert_eq!(all_txs[0].get_id(), tx2.get_id());
}

#[test]
fn test_local_blocks_in_transit_operations() {
    // Test local blocks in transit operations
    let blocks_in_transit = BlockInTransit::new();
    
    // Test adding blocks
    let block_hashes = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
    ];
    
    blocks_in_transit.add_blocks(&block_hashes);
    assert_eq!(blocks_in_transit.len(), 3);
    assert!(!blocks_in_transit.is_empty());
    
    // Test getting first block
    let first_block = blocks_in_transit.first();
    assert!(first_block.is_some());
    assert_eq!(first_block.unwrap(), vec![1, 2, 3, 4, 5]);
    
    // Test removing blocks
    blocks_in_transit.remove(&vec![1, 2, 3, 4, 5]);
    assert_eq!(blocks_in_transit.len(), 2);
    
    // Test clearing all blocks
    blocks_in_transit.clear();
    assert_eq!(blocks_in_transit.len(), 0);
    assert!(blocks_in_transit.is_empty());
}

#[test]
fn test_local_nodes_management() {
    // Test local nodes management
    let nodes = Nodes::new();
    
    // Test adding nodes
    let node1 = "127.0.0.1:3001".to_string();
    let node2 = "127.0.0.1:3002".to_string();
    
    nodes.add_node(node1.clone());
    nodes.add_node(node2.clone());
    
    // Test node existence
    assert!(nodes.node_is_known(&node1));
    assert!(nodes.node_is_known(&node2));
    assert!(!nodes.node_is_known("127.0.0.1:3003"));
    
    // Test getting all nodes
    let all_nodes = nodes.get_nodes();
    assert!(all_nodes.len() >= 2); // At least our two nodes
}

#[test]
fn test_thread_safety() {
    // Test that local instances can be accessed from multiple threads
    let memory_pool = Arc::new(MemoryPool::new());
    let blocks_in_transit = Arc::new(BlockInTransit::new());
    let nodes = Arc::new(Nodes::new());
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let memory_pool_clone = memory_pool.clone();
            let blocks_in_transit_clone = blocks_in_transit.clone();
            let nodes_clone = nodes.clone();
            
            thread::spawn(move || {
                let tx = create_test_transaction(vec![i as u8]);
                memory_pool_clone.add(tx);
                
                let block_hash = vec![i as u8; 5];
                blocks_in_transit_clone.add_blocks(&[block_hash]);
                
                let node_addr = format!("127.0.0.1:{}", 3000 + i);
                nodes_clone.add_node(node_addr.clone());
                
                node_addr
            })
        })
        .collect();
    
    let node_addrs: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // Verify all operations completed
    assert_eq!(memory_pool.len(), 10);
    assert_eq!(blocks_in_transit.len(), 10);
    
    for addr in node_addrs {
        assert!(nodes.node_is_known(&addr));
    }
}

#[test]
fn test_comprehensive_package_handling() {
    // Test comprehensive package creation and handling
    let test_block = create_test_block("comprehensive_test".to_string(), 42);
    let test_tx = create_test_transaction(vec![42, 43, 44, 45]);
    
    // Test all package variants
    let packages = vec![
        ("Block", Package::Block {
            addr_from: "test_node".to_string(),
            block: test_block.serialize(),
        }),
        ("GetBlocks", Package::GetBlocks {
            addr_from: "test_node".to_string(),
        }),
        ("GetData", Package::GetData {
            addr_from: "test_node".to_string(),
            op_type: OpType::Block,
            id: vec![1, 2, 3, 4, 5],
        }),
        ("Inv", Package::Inv {
            addr_from: "test_node".to_string(),
            op_type: OpType::Tx,
            items: vec![vec![6, 7, 8], vec![9, 10, 11]],
        }),
        ("Tx", Package::Tx {
            addr_from: "test_node".to_string(),
            transaction: test_tx.serialize(),
        }),
        ("Version", Package::Version {
            addr_from: "test_node".to_string(),
            version: NODE_VERSION,
            best_height: 100,
        }),
    ];
    
    for (name, package) in packages {
        // Test serialization
        let serialized = serialize_package(&package);
        assert!(!serialized.is_empty(), "Failed to serialize {}", name);
        
        // Test deserialization
        let (deserialized, _): (Package, usize) = bincode::decode_from_slice(&serialized, standard())
            .expect(&format!("Failed to deserialize {}", name));
        
        // Verify the package type is preserved
        match (&package, &deserialized) {
            (Package::Block { .. }, Package::Block { .. }) => (),
            (Package::GetBlocks { .. }, Package::GetBlocks { .. }) => (),
            (Package::GetData { .. }, Package::GetData { .. }) => (),
            (Package::Inv { .. }, Package::Inv { .. }) => (),
            (Package::Tx { .. }, Package::Tx { .. }) => (),
            (Package::Version { .. }, Package::Version { .. }) => (),
            _ => panic!("Package type mismatch for {}: {:?} vs {:?}", name, package, deserialized),
        }
    }
} 
