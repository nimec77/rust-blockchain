use rust_blockchain::{Transaction, TXInput, TXOutput};

use crate::test_helpers::{create_output_with_key_hash, create_output_with_value, create_sample_output};

// Transaction tests
#[test]
fn test_transaction_get_id() {
    // Create a test transaction with a known ID
    let test_id = vec![1, 2, 3, 4, 5];
    let transaction = Transaction {
        id: test_id.clone(),
        vin: vec![],
        vout: vec![],
    };

    // Test that get_id returns the correct ID
    assert_eq!(transaction.get_id(), test_id.as_slice());
}

#[test]
fn test_transaction_get_id_empty() {
    // Test with an empty ID
    let transaction = Transaction {
        id: vec![],
        vin: vec![],
        vout: vec![],
    };

    assert_eq!(transaction.get_id(), &[]);
}

// TXInput tests
#[test]
fn test_txinput_new() {
    let txid = b"test_transaction_id";
    let vout = 42;
    
    let tx_input = TXInput::new(txid, vout);
    
    assert_eq!(tx_input.txid, txid.to_vec());
    assert_eq!(tx_input.vout, vout);
    assert_eq!(tx_input.signature, vec![]);
    assert_eq!(tx_input.pub_key, vec![]);
}

#[test]
fn test_txinput_new_empty_txid() {
    let txid = b"";
    let vout = 0;
    
    let tx_input = TXInput::new(txid, vout);
    
    assert_eq!(tx_input.txid, vec![]);
    assert_eq!(tx_input.vout, 0);
    assert_eq!(tx_input.signature, vec![]);
    assert_eq!(tx_input.pub_key, vec![]);
}

#[test]
fn test_txinput_new_large_vout() {
    let txid = b"large_vout_test";
    let vout = usize::MAX;
    
    let tx_input = TXInput::new(txid, vout);
    
    assert_eq!(tx_input.txid, txid.to_vec());
    assert_eq!(tx_input.vout, usize::MAX);
}

#[test]
fn test_txinput_get_txid() {
    let txid = b"sample_transaction_id";
    let vout = 1;
    let tx_input = TXInput::new(txid, vout);
    
    assert_eq!(tx_input.get_txid(), txid);
}

#[test]
fn test_txinput_get_txid_empty() {
    let txid = b"";
    let vout = 0;
    let tx_input = TXInput::new(txid, vout);
    
    assert_eq!(tx_input.get_txid(), b"");
}

#[test]
fn test_txinput_get_vout() {
    let txid = b"test_txid";
    let vout = 123;
    let tx_input = TXInput::new(txid, vout);
    
    assert_eq!(tx_input.get_vout(), 123);
}

#[test]
fn test_txinput_get_vout_zero() {
    let txid = b"zero_vout_test";
    let vout = 0;
    let tx_input = TXInput::new(txid, vout);
    
    assert_eq!(tx_input.get_vout(), 0);
}

#[test]
fn test_txinput_get_pub_key_default_empty() {
    let txid = b"pub_key_test";
    let vout = 5;
    let tx_input = TXInput::new(txid, vout);
    
    // pub_key should be empty by default from new()
    assert_eq!(tx_input.get_pub_key(), b"");
}

#[test]
fn test_txinput_get_pub_key_with_manual_assignment() {
    let txid = b"manual_pub_key_test";
    let vout = 7;
    let mut tx_input = TXInput::new(txid, vout);
    
    // Manually assign a pub_key value to test the getter
    let test_pub_key = b"test_public_key_data";
    tx_input.pub_key = test_pub_key.to_vec();
    
    assert_eq!(tx_input.get_pub_key(), test_pub_key);
}

#[test]
fn test_txinput_clone() {
    let txid = b"clone_test_txid";
    let vout = 99;
    let mut original = TXInput::new(txid, vout);
    original.signature = b"test_signature".to_vec();
    original.pub_key = b"test_pub_key".to_vec();
    
    let cloned = original.clone();
    
    assert_eq!(original.txid, cloned.txid);
    assert_eq!(original.vout, cloned.vout);
    assert_eq!(original.signature, cloned.signature);
    assert_eq!(original.pub_key, cloned.pub_key);
    
    // Ensure they are separate instances
    assert_ne!(original.txid.as_ptr(), cloned.txid.as_ptr());
}

#[test]
fn test_txinput_serialization() {
    let txid = b"serialization_test";
    let vout = 42;
    let mut tx_input = TXInput::new(txid, vout);
    tx_input.signature = b"test_signature_data".to_vec();
    tx_input.pub_key = b"test_public_key".to_vec();
    
    // Test that it can be encoded and decoded
    let encoded = bincode::encode_to_vec(&tx_input, bincode::config::standard()).unwrap();
    let (decoded, _): (TXInput, usize) = bincode::decode_from_slice(&encoded, bincode::config::standard()).unwrap();
    
    assert_eq!(tx_input.txid, decoded.txid);
    assert_eq!(tx_input.vout, decoded.vout);
    assert_eq!(tx_input.signature, decoded.signature);
    assert_eq!(tx_input.pub_key, decoded.pub_key);
}

#[test]
fn test_txoutput_get_value_positive() {
    let output = create_output_with_value(150);
    assert_eq!(output.get_value(), 150);
}

#[test]
fn test_txoutput_get_value_zero() {
    let output = create_output_with_value(0);
    assert_eq!(output.get_value(), 0);
}

#[test]
fn test_txoutput_get_value_negative() {
    let output = create_output_with_value(-25);
    assert_eq!(output.get_value(), -25);
}

#[test]
fn test_txoutput_get_pub_key_hash_normal() {
    let key_hash = vec![10, 20, 30, 40, 50];
    let output = create_output_with_key_hash(key_hash.clone());
    assert_eq!(output.get_pub_key_hash(), key_hash.as_slice());
}

#[test]
fn test_txoutput_get_pub_key_hash_empty() {
    let output = create_output_with_key_hash(vec![]);
    assert_eq!(output.get_pub_key_hash(), &[]);
}

#[test]
fn test_txoutput_get_pub_key_hash_single_byte() {
    let key_hash = vec![255];
    let output = create_output_with_key_hash(key_hash.clone());
    assert_eq!(output.get_pub_key_hash(), key_hash.as_slice());
}

#[test]
fn test_txoutput_is_locked_with_key_matching() {
    let key_hash = vec![1, 2, 3, 4, 5];
    let output = create_output_with_key_hash(key_hash.clone());
    assert!(output.is_locked_with_key(&key_hash));
}

#[test]
fn test_txoutput_is_locked_with_key_not_matching() {
    let key_hash = vec![1, 2, 3, 4, 5];
    let different_key = vec![5, 4, 3, 2, 1];
    let output = create_output_with_key_hash(key_hash);
    assert!(!output.is_locked_with_key(&different_key));
}

#[test]
fn test_txoutput_is_locked_with_key_different_length() {
    let key_hash = vec![1, 2, 3];
    let longer_key = vec![1, 2, 3, 4, 5];
    let output = create_output_with_key_hash(key_hash);
    assert!(!output.is_locked_with_key(&longer_key));
}

#[test]
fn test_txoutput_is_locked_with_key_empty_keys() {
    let output = create_output_with_key_hash(vec![]);
    assert!(output.is_locked_with_key(&[]));
}

#[test]
fn test_txoutput_is_locked_with_key_empty_stored_key() {
    let output = create_output_with_key_hash(vec![]);
    let test_key = vec![1, 2, 3];
    assert!(!output.is_locked_with_key(&test_key));
}

#[test]
fn test_txoutput_struct_creation_and_fields() {
    let output = TXOutput {
        value: 42,
        pub_key_hash: vec![0xAB, 0xCD, 0xEF],
    };
    
    assert_eq!(output.value, 42);
    assert_eq!(output.pub_key_hash, vec![0xAB, 0xCD, 0xEF]);
}

#[test]
fn test_txoutput_clone_functionality() {
    let original = create_sample_output();
    let cloned = original.clone();
    
    assert_eq!(original.get_value(), cloned.get_value());
    assert_eq!(original.get_pub_key_hash(), cloned.get_pub_key_hash());
    assert!(cloned.is_locked_with_key(original.get_pub_key_hash()));
}

#[test]
fn test_txoutput_serialization_roundtrip() {
    let original = create_sample_output();
    
    // Serialize
    let encoded = bincode::encode_to_vec(&original, bincode::config::standard())
        .expect("Failed to serialize TXOutput");
    
    // Deserialize
    let decoded: TXOutput = bincode::decode_from_slice(&encoded, bincode::config::standard())
        .expect("Failed to deserialize TXOutput").0;
    
    // Verify roundtrip integrity
    assert_eq!(original.get_value(), decoded.get_value());
    assert_eq!(original.get_pub_key_hash(), decoded.get_pub_key_hash());
    assert!(decoded.is_locked_with_key(original.get_pub_key_hash()));
}

#[test]
fn test_txoutput_large_values() {
    let output = create_output_with_value(i32::MAX);
    assert_eq!(output.get_value(), i32::MAX);
    
    let output = create_output_with_value(i32::MIN);
    assert_eq!(output.get_value(), i32::MIN);
}

#[test]
fn test_txoutput_large_key_hash() {
    let large_key = vec![42u8; 1000]; // 1000 bytes of the same value
    let output = create_output_with_key_hash(large_key.clone());
    assert_eq!(output.get_pub_key_hash(), large_key.as_slice());
    assert!(output.is_locked_with_key(&large_key));
}

// Integration tests combining multiple transaction components
#[test]
fn test_complete_transaction_construction() {
    let tx_input = TXInput::new(b"input_txid", 0);
    let tx_output = TXOutput {
        value: 100,
        pub_key_hash: vec![1, 2, 3, 4],
    };
    
    let transaction = Transaction {
        id: vec![10, 20, 30],
        vin: vec![tx_input],
        vout: vec![tx_output],
    };
    
    assert_eq!(transaction.get_id(), &[10, 20, 30]);
    assert_eq!(transaction.vin.len(), 1);
    assert_eq!(transaction.vout.len(), 1);
    assert_eq!(transaction.vin[0].get_txid(), b"input_txid");
    assert_eq!(transaction.vout[0].get_value(), 100);
}

#[test]
fn test_transaction_with_multiple_inputs_outputs() {
    let tx_input1 = TXInput::new(b"input1", 0);
    let tx_input2 = TXInput::new(b"input2", 1);
    
    let tx_output1 = TXOutput {
        value: 50,
        pub_key_hash: vec![1, 2],
    };
    let tx_output2 = TXOutput {
        value: 30,
        pub_key_hash: vec![3, 4],
    };
    
    let transaction = Transaction {
        id: vec![100],
        vin: vec![tx_input1, tx_input2],
        vout: vec![tx_output1, tx_output2],
    };
    
    assert_eq!(transaction.vin.len(), 2);
    assert_eq!(transaction.vout.len(), 2);
    assert_eq!(transaction.vin[0].get_txid(), b"input1");
    assert_eq!(transaction.vin[1].get_txid(), b"input2");
    assert_eq!(transaction.vout[0].get_value(), 50);
    assert_eq!(transaction.vout[1].get_value(), 30);
}

#[test]
fn test_transaction_serialization() {
    let tx_input = TXInput::new(b"serialization_input", 5);
    let tx_output = TXOutput {
        value: 200,
        pub_key_hash: vec![5, 6, 7, 8],
    };
    
    let original_transaction = Transaction {
        id: vec![50, 60, 70],
        vin: vec![tx_input],
        vout: vec![tx_output],
    };
    
    // Serialize
    let encoded = bincode::encode_to_vec(&original_transaction, bincode::config::standard()).unwrap();
    
    // Deserialize
    let (decoded_transaction, _): (Transaction, usize) = 
        bincode::decode_from_slice(&encoded, bincode::config::standard()).unwrap();
    
    // Verify all fields
    assert_eq!(original_transaction.get_id(), decoded_transaction.get_id());
    assert_eq!(original_transaction.vin.len(), decoded_transaction.vin.len());
    assert_eq!(original_transaction.vout.len(), decoded_transaction.vout.len());
    assert_eq!(original_transaction.vin[0].get_txid(), decoded_transaction.vin[0].get_txid());
    assert_eq!(original_transaction.vout[0].get_value(), decoded_transaction.vout[0].get_value());
} 
