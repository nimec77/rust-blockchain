use rust_blockchain::server::{OpType, Package};
use bincode::{config, decode_from_slice, encode_to_vec};

#[test]
fn test_optype_encode_decode_tx() {
    let op_type = OpType::Tx;
    let config = config::standard();
    
    let encoded = encode_to_vec(&op_type, config).unwrap();
    let (decoded, _): (OpType, usize) = decode_from_slice(&encoded, config).unwrap();
    
    // OpType::Tx should encode to byte 0
    assert_eq!(encoded, vec![0]);
    
    // Verify round-trip
    match decoded {
        OpType::Tx => (),
        _ => panic!("Expected OpType::Tx, got {decoded:?}"),
    }
}

#[test]
fn test_optype_encode_decode_block() {
    let op_type = OpType::Block;
    let config = config::standard();
    
    let encoded = encode_to_vec(&op_type, config).unwrap();
    let (decoded, _): (OpType, usize) = decode_from_slice(&encoded, config).unwrap();
    
    // OpType::Block should encode to byte 1 (automatic implementation)
    assert_eq!(encoded, vec![1]);
    
    // Verify round-trip
    match decoded {
        OpType::Block => (),
        _ => panic!("Expected OpType::Block, got {decoded:?}"),
    }
}

#[test]
fn test_optype_decode_invalid_byte() {
    let config = config::standard();
    
    // Test invalid byte value (2 is not a valid OpType)
    let invalid_encoded = vec![2];
    let result: Result<(OpType, usize), _> = decode_from_slice(&invalid_encoded, config);
    
    assert!(result.is_err());
    if let Err(e) = result {
        // Should be a DecodeError::UnexpectedVariant
        assert!(e.to_string().contains("UnexpectedVariant") || e.to_string().contains("decode"));
    }
}

#[test]
fn test_optype_round_trip_all_variants() {
    let variants = vec![OpType::Tx, OpType::Block];
    let config = config::standard();
    
    for variant in variants {
        let encoded = encode_to_vec(&variant, config).unwrap();
        let (decoded, _): (OpType, usize) = decode_from_slice(&encoded, config).unwrap();
        
        match (&variant, &decoded) {
            (OpType::Tx, OpType::Tx) => (),
            (OpType::Block, OpType::Block) => (),
            _ => panic!("Round-trip failed for {variant:?}, got {decoded:?}"),
        }
    }
}

#[test]
fn test_package_encode_decode_block() {
    let package = Package::Block {
        addr_from: "localhost:3000".to_string(),
        block: vec![1, 2, 3, 4, 5],
    };
    let config = config::standard();
    
    let encoded = encode_to_vec(&package, config).unwrap();
    let (decoded, _): (Package, usize) = decode_from_slice(&encoded, config).unwrap();
    
    // Package::Block should have discriminant 0
    assert_eq!(encoded[0], 0);
    
    match decoded {
        Package::Block { addr_from, block } => {
            assert_eq!(addr_from, "localhost:3000");
            assert_eq!(block, vec![1, 2, 3, 4, 5]);
        }
        _ => panic!("Expected Package::Block, got {decoded:?}"),
    }
}

#[test]
fn test_package_encode_decode_get_blocks() {
    let package = Package::GetBlocks {
        addr_from: "localhost:3001".to_string(),
    };
    let config = config::standard();
    
    let encoded = encode_to_vec(&package, config).unwrap();
    let (decoded, _): (Package, usize) = decode_from_slice(&encoded, config).unwrap();
    
    // Package::GetBlocks should have discriminant 1
    assert_eq!(encoded[0], 1);
    
    match decoded {
        Package::GetBlocks { addr_from } => {
            assert_eq!(addr_from, "localhost:3001");
        }
        _ => panic!("Expected Package::GetBlocks, got {decoded:?}"),
    }
}

#[test]
fn test_package_encode_decode_get_data() {
    let package = Package::GetData {
        addr_from: "localhost:3002".to_string(),
        op_type: OpType::Block,
        id: vec![10, 20, 30],
    };
    let config = config::standard();
    
    let encoded = encode_to_vec(&package, config).unwrap();
    let (decoded, _): (Package, usize) = decode_from_slice(&encoded, config).unwrap();
    
    // Package::GetData should have discriminant 2
    assert_eq!(encoded[0], 2);
    
    match decoded {
        Package::GetData { addr_from, op_type, id } => {
            assert_eq!(addr_from, "localhost:3002");
            match op_type {
                OpType::Block => (),
                _ => panic!("Expected OpType::Block, got {op_type:?}"),
            }
            assert_eq!(id, vec![10, 20, 30]);
        }
        _ => panic!("Expected Package::GetData, got {decoded:?}"),
    }
}

#[test]
fn test_package_encode_decode_inv() {
    let package = Package::Inv {
        addr_from: "localhost:3003".to_string(),
        op_type: OpType::Tx,
        items: vec![vec![1, 2], vec![3, 4], vec![5, 6]],
    };
    let config = config::standard();
    
    let encoded = encode_to_vec(&package, config).unwrap();
    let (decoded, _): (Package, usize) = decode_from_slice(&encoded, config).unwrap();
    
    // Package::Inv should have discriminant 3
    assert_eq!(encoded[0], 3);
    
    match decoded {
        Package::Inv { addr_from, op_type, items } => {
            assert_eq!(addr_from, "localhost:3003");
            match op_type {
                OpType::Tx => (),
                _ => panic!("Expected OpType::Tx, got {op_type:?}"),
            }
            assert_eq!(items, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
        }
        _ => panic!("Expected Package::Inv, got {decoded:?}"),
    }
}

#[test]
fn test_package_encode_decode_tx() {
    let package = Package::Tx {
        addr_from: "localhost:3004".to_string(),
        transaction: vec![100, 200, 255],
    };
    let config = config::standard();
    
    let encoded = encode_to_vec(&package, config).unwrap();
    let (decoded, _): (Package, usize) = decode_from_slice(&encoded, config).unwrap();
    
    // Package::Tx should have discriminant 4
    assert_eq!(encoded[0], 4);
    
    match decoded {
        Package::Tx { addr_from, transaction } => {
            assert_eq!(addr_from, "localhost:3004");
            assert_eq!(transaction, vec![100, 200, 255]);
        }
        _ => panic!("Expected Package::Tx, got {decoded:?}"),
    }
}

#[test]
fn test_package_encode_decode_version() {
    let package = Package::Version {
        addr_from: "localhost:3005".to_string(),
        version: 1,
        best_height: 42,
    };
    let config = config::standard();
    
    let encoded = encode_to_vec(&package, config).unwrap();
    let (decoded, _): (Package, usize) = decode_from_slice(&encoded, config).unwrap();
    
    // Package::Version should have discriminant 5
    assert_eq!(encoded[0], 5);
    
    match decoded {
        Package::Version { addr_from, version, best_height } => {
            assert_eq!(addr_from, "localhost:3005");
            assert_eq!(version, 1);
            assert_eq!(best_height, 42);
        }
        _ => panic!("Expected Package::Version, got {decoded:?}"),
    }
}

#[test]
fn test_package_decode_invalid_discriminant() {
    let config = config::standard();
    
    // Test invalid discriminant (6 is not a valid Package variant)
    let invalid_encoded = vec![6];
    let result: Result<(Package, usize), _> = decode_from_slice(&invalid_encoded, config);
    
    assert!(result.is_err());
    if let Err(e) = result {
        // Should be a DecodeError::UnexpectedVariant or similar
        assert!(e.to_string().contains("UnexpectedVariant") || e.to_string().contains("decode"));
    }
}

#[test]
fn test_package_with_empty_data() {
    // Test packages with empty vectors and strings
    let packages = vec![
        Package::Block {
            addr_from: "".to_string(),
            block: vec![],
        },
        Package::GetBlocks {
            addr_from: "".to_string(),
        },
        Package::GetData {
            addr_from: "".to_string(),
            op_type: OpType::Tx,
            id: vec![],
        },
        Package::Inv {
            addr_from: "".to_string(),
            op_type: OpType::Block,
            items: vec![],
        },
        Package::Tx {
            addr_from: "".to_string(),
            transaction: vec![],
        },
        Package::Version {
            addr_from: "".to_string(),
            version: 0,
            best_height: 0,
        },
    ];
    
    let config = config::standard();
    
    for package in packages {
        let encoded = encode_to_vec(&package, config).unwrap();
        let (decoded, _): (Package, usize) = decode_from_slice(&encoded, config).unwrap();
        
        // Verify round-trip works with empty data
        match (&package, &decoded) {
            (Package::Block { addr_from: a1, block: b1 }, Package::Block { addr_from: a2, block: b2 }) => {
                assert_eq!(a1, a2);
                assert_eq!(b1, b2);
            }
            (Package::GetBlocks { addr_from: a1 }, Package::GetBlocks { addr_from: a2 }) => {
                assert_eq!(a1, a2);
            }
            (Package::GetData { addr_from: a1, op_type: o1, id: i1 }, Package::GetData { addr_from: a2, op_type: o2, id: i2 }) => {
                assert_eq!(a1, a2);
                assert_eq!(i1, i2);
                match (o1, o2) {
                    (OpType::Tx, OpType::Tx) | (OpType::Block, OpType::Block) => (),
                    _ => panic!("OpType mismatch"),
                }
            }
            (Package::Inv { addr_from: a1, op_type: o1, items: i1 }, Package::Inv { addr_from: a2, op_type: o2, items: i2 }) => {
                assert_eq!(a1, a2);
                assert_eq!(i1, i2);
                match (o1, o2) {
                    (OpType::Tx, OpType::Tx) | (OpType::Block, OpType::Block) => (),
                    _ => panic!("OpType mismatch"),
                }
            }
            (Package::Tx { addr_from: a1, transaction: t1 }, Package::Tx { addr_from: a2, transaction: t2 }) => {
                assert_eq!(a1, a2);
                assert_eq!(t1, t2);
            }
            (Package::Version { addr_from: a1, version: v1, best_height: h1 }, Package::Version { addr_from: a2, version: v2, best_height: h2 }) => {
                assert_eq!(a1, a2);
                assert_eq!(v1, v2);
                assert_eq!(h1, h2);
            }
            _ => panic!("Package variant mismatch"),
        }
    }
}

#[test]
fn test_package_with_large_data() {
    // Test with large data to ensure no overflow issues
    let large_data = vec![0u8; 10000];
    let large_items = vec![vec![1u8; 1000]; 5];
    
    let packages = vec![
        Package::Block {
            addr_from: "very-long-address-name-that-exceeds-typical-length".to_string(),
            block: large_data.clone(),
        },
        Package::GetData {
            addr_from: "another-very-long-address-name".to_string(),
            op_type: OpType::Block,
            id: large_data.clone(),
        },
        Package::Inv {
            addr_from: "yet-another-long-address".to_string(),
            op_type: OpType::Tx,
            items: large_items,
        },
        Package::Tx {
            addr_from: "final-long-address-name".to_string(),
            transaction: large_data,
        },
    ];
    
    let config = config::standard();
    
    for package in packages {
        let encoded = encode_to_vec(&package, config).unwrap();
        let (decoded, _): (Package, usize) = decode_from_slice(&encoded, config).unwrap();
        
        // Just verify the encoding/decoding doesn't crash with large data
        // The exact comparison would be too verbose, but we can check the discriminant
        match (&package, &decoded) {
            (Package::Block { .. }, Package::Block { .. }) => (),
            (Package::GetData { .. }, Package::GetData { .. }) => (),
            (Package::Inv { .. }, Package::Inv { .. }) => (),
            (Package::Tx { .. }, Package::Tx { .. }) => (),
            _ => panic!("Package variant mismatch with large data"),
        }
    }
}

#[test]
fn test_discriminant_values() {
    // Test that discriminant values are as expected
    let config = config::standard();
    
    let packages = vec![
        (Package::Block { addr_from: "test".to_string(), block: vec![] }, 0u8),
        (Package::GetBlocks { addr_from: "test".to_string() }, 1u8),
        (Package::GetData { addr_from: "test".to_string(), op_type: OpType::Tx, id: vec![] }, 2u8),
        (Package::Inv { addr_from: "test".to_string(), op_type: OpType::Block, items: vec![] }, 3u8),
        (Package::Tx { addr_from: "test".to_string(), transaction: vec![] }, 4u8),
        (Package::Version { addr_from: "test".to_string(), version: 1, best_height: 1 }, 5u8),
    ];
    
    for (package, expected_discriminant) in packages {
        let encoded = encode_to_vec(&package, config).unwrap();
        assert_eq!(encoded[0], expected_discriminant, "Wrong discriminant for {package:?}");
    }
}

#[test]
fn test_optype_discriminant_values() {
    // Test that OpType discriminant values are as expected
    let config = config::standard();
    
    let op_types = vec![
        (OpType::Tx, 0u8),
        (OpType::Block, 1u8),
    ];
    
    for (op_type, expected_discriminant) in op_types {
        let encoded = encode_to_vec(&op_type, config).unwrap();
        assert_eq!(encoded[0], expected_discriminant, "Wrong discriminant for {op_type:?}");
    }
}



 
