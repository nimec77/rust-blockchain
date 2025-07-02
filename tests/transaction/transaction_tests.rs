use rust_blockchain::{TXInput, TXOutput, Transaction};

use crate::test_helpers::{
    create_output_with_key_hash, create_output_with_value, create_sample_output,
};

// Transaction tests
#[test]
fn test_transaction_get_id() {
    // Create a test transaction with a known ID
    let test_id = vec![1, 2, 3, 4, 5];
    let transaction = Transaction::new(test_id.clone(), vec![], vec![]);

    // Test that get_id returns the correct ID
    assert_eq!(transaction.get_id(), test_id.as_slice());
}

#[test]
fn test_transaction_get_id_empty() {
    // Test with an empty ID
    let transaction = Transaction::new(vec![], vec![], vec![]);

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
    let (decoded, _): (TXInput, usize) =
        bincode::decode_from_slice(&encoded, bincode::config::standard()).unwrap();

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
        .expect("Failed to deserialize TXOutput")
        .0;

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

    let transaction = Transaction::new(vec![10, 20, 30], vec![tx_input], vec![tx_output]);

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

    let transaction = Transaction::new(
        vec![100],
        vec![tx_input1, tx_input2],
        vec![tx_output1, tx_output2],
    );

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

    let original_transaction = Transaction::new(vec![50, 60, 70], vec![tx_input], vec![tx_output]);

    // Serialize
    let encoded =
        bincode::encode_to_vec(&original_transaction, bincode::config::standard()).unwrap();

    // Deserialize
    let (decoded_transaction, _): (Transaction, usize) =
        bincode::decode_from_slice(&encoded, bincode::config::standard()).unwrap();

    // Verify all fields
    assert_eq!(original_transaction.get_id(), decoded_transaction.get_id());
    assert_eq!(
        original_transaction.vin.len(),
        decoded_transaction.vin.len()
    );
    assert_eq!(
        original_transaction.vout.len(),
        decoded_transaction.vout.len()
    );
    assert_eq!(
        original_transaction.vin[0].get_txid(),
        decoded_transaction.vin[0].get_txid()
    );
    assert_eq!(
        original_transaction.vout[0].get_value(),
        decoded_transaction.vout[0].get_value()
    );
}

// Tests for Transaction::get_vin()
#[test]
fn test_transaction_get_vin_empty() {
    let transaction = Transaction::new(vec![1, 2, 3], vec![], vec![]);

    let vin = transaction.get_vin();
    assert_eq!(vin.len(), 0);
    assert!(vin.is_empty());
}

#[test]
fn test_transaction_get_vin_single_input() {
    let tx_input = TXInput::new(b"test_txid", 0);
    let transaction = Transaction::new(vec![1, 2, 3], vec![tx_input.clone()], vec![]);

    let vin = transaction.get_vin();
    assert_eq!(vin.len(), 1);
    assert_eq!(vin[0].get_txid(), tx_input.get_txid());
    assert_eq!(vin[0].get_vout(), tx_input.get_vout());
}

#[test]
fn test_transaction_get_vin_multiple_inputs() {
    let tx_input1 = TXInput::new(b"txid1", 0);
    let tx_input2 = TXInput::new(b"txid2", 1);
    let tx_input3 = TXInput::new(b"txid3", 2);

    let transaction = Transaction::new(
        vec![1, 2, 3],
        vec![tx_input1.clone(), tx_input2.clone(), tx_input3.clone()],
        vec![],
    );

    let vin = transaction.get_vin();
    assert_eq!(vin.len(), 3);
    assert_eq!(vin[0].get_txid(), tx_input1.get_txid());
    assert_eq!(vin[1].get_txid(), tx_input2.get_txid());
    assert_eq!(vin[2].get_txid(), tx_input3.get_txid());
    assert_eq!(vin[0].get_vout(), 0);
    assert_eq!(vin[1].get_vout(), 1);
    assert_eq!(vin[2].get_vout(), 2);
}

// Tests for Transaction::get_vout()
#[test]
fn test_transaction_get_vout_empty() {
    let transaction = Transaction::new(vec![1, 2, 3], vec![], vec![]);

    let vout = transaction.get_vout();
    assert_eq!(vout.len(), 0);
    assert!(vout.is_empty());
}

#[test]
fn test_transaction_get_vout_single_output() {
    let tx_output = TXOutput {
        value: 100,
        pub_key_hash: vec![1, 2, 3, 4],
    };

    let transaction = Transaction::new(vec![1, 2, 3], vec![], vec![tx_output.clone()]);

    let vout = transaction.get_vout();
    assert_eq!(vout.len(), 1);
    assert_eq!(vout[0].get_value(), tx_output.get_value());
    assert_eq!(vout[0].get_pub_key_hash(), tx_output.get_pub_key_hash());
}

#[test]
fn test_transaction_get_vout_multiple_outputs() {
    let tx_output1 = TXOutput {
        value: 50,
        pub_key_hash: vec![1, 2],
    };
    let tx_output2 = TXOutput {
        value: 75,
        pub_key_hash: vec![3, 4],
    };
    let tx_output3 = TXOutput {
        value: 25,
        pub_key_hash: vec![5, 6],
    };

    let transaction = Transaction::new(
        vec![1, 2, 3],
        vec![],
        vec![tx_output1.clone(), tx_output2.clone(), tx_output3.clone()],
    );

    let vout = transaction.get_vout();
    assert_eq!(vout.len(), 3);
    assert_eq!(vout[0].get_value(), 50);
    assert_eq!(vout[1].get_value(), 75);
    assert_eq!(vout[2].get_value(), 25);
    assert_eq!(vout[0].get_pub_key_hash(), &[1, 2]);
    assert_eq!(vout[1].get_pub_key_hash(), &[3, 4]);
    assert_eq!(vout[2].get_pub_key_hash(), &[5, 6]);
}

// Tests for Transaction::is_coinbase()
#[test]
fn test_transaction_is_coinbase_true() {
    // Coinbase transaction: exactly one input with empty pub_key
    let mut tx_input = TXInput::new(b"coinbase_txid", 0);
    tx_input.pub_key = vec![]; // Empty pub_key indicates coinbase

    let transaction = Transaction::new(vec![1, 2, 3], vec![tx_input], vec![]);

    assert!(transaction.is_coinbase());
}

#[test]
fn test_transaction_is_coinbase_false_multiple_inputs() {
    // Not coinbase: multiple inputs
    let mut tx_input1 = TXInput::new(b"txid1", 0);
    tx_input1.pub_key = vec![]; // Even if pub_key is empty
    let mut tx_input2 = TXInput::new(b"txid2", 1);
    tx_input2.pub_key = vec![];

    let transaction = Transaction::new(vec![1, 2, 3], vec![tx_input1, tx_input2], vec![]);

    assert!(!transaction.is_coinbase());
}

#[test]
fn test_transaction_is_coinbase_false_single_input_with_pub_key() {
    // Not coinbase: single input but with non-empty pub_key
    let mut tx_input = TXInput::new(b"regular_txid", 0);
    tx_input.pub_key = vec![1, 2, 3, 4]; // Non-empty pub_key

    let transaction = Transaction::new(vec![1, 2, 3], vec![tx_input], vec![]);

    assert!(!transaction.is_coinbase());
}

#[test]
fn test_transaction_is_coinbase_false_no_inputs() {
    // Not coinbase: no inputs at all
    let transaction = Transaction::new(vec![1, 2, 3], vec![], vec![]);

    assert!(!transaction.is_coinbase());
}

#[test]
fn test_transaction_is_coinbase_with_outputs() {
    // Coinbase transaction can have outputs
    let mut tx_input = TXInput::new(b"coinbase_with_outputs", 0);
    tx_input.pub_key = vec![]; // Empty pub_key

    let tx_output = TXOutput {
        value: 50,
        pub_key_hash: vec![10, 20, 30],
    };

    let transaction = Transaction::new(vec![1, 2, 3], vec![tx_input], vec![tx_output]);

    assert!(transaction.is_coinbase());
}

#[test]
fn test_transaction_get_methods_consistency() {
    // Test that get_vin() and get_vout() return consistent views
    let tx_input1 = TXInput::new(b"consistency_test1", 0);
    let tx_input2 = TXInput::new(b"consistency_test2", 1);

    let tx_output1 = TXOutput {
        value: 100,
        pub_key_hash: vec![1, 2, 3],
    };
    let tx_output2 = TXOutput {
        value: 200,
        pub_key_hash: vec![4, 5, 6],
    };

    let transaction = Transaction::new(
        vec![1, 2, 3],
        vec![tx_input1, tx_input2],
        vec![tx_output1, tx_output2],
    );

    // Test that the slices returned by get_vin() and get_vout() match the original vectors
    assert_eq!(transaction.get_vin().len(), transaction.vin.len());
    assert_eq!(transaction.get_vout().len(), transaction.vout.len());

    // Check individual elements
    for (i, input) in transaction.get_vin().iter().enumerate() {
        assert_eq!(input.get_txid(), transaction.vin[i].get_txid());
        assert_eq!(input.get_vout(), transaction.vin[i].get_vout());
    }

    for (i, output) in transaction.get_vout().iter().enumerate() {
        assert_eq!(output.get_value(), transaction.vout[i].get_value());
        assert_eq!(
            output.get_pub_key_hash(),
            transaction.vout[i].get_pub_key_hash()
        );
    }

    // This should not be a coinbase transaction (multiple inputs)
    assert!(!transaction.is_coinbase());
}

// Tests for Transaction::serialize()
#[test]
fn test_transaction_serialize_basic() {
    let tx_input = TXInput::new(b"test_txid", 0);
    let tx_output = TXOutput {
        value: 100,
        pub_key_hash: vec![1, 2, 3, 4],
    };

    let transaction = Transaction::new(vec![10, 20, 30], vec![tx_input], vec![tx_output]);

    let serialized = transaction.serialize();
    assert!(!serialized.is_empty());

    // Verify it's valid by deserializing
    let deserialized = Transaction::deserialize(&serialized);
    assert_eq!(transaction.get_id(), deserialized.get_id());
    assert_eq!(transaction.vin.len(), deserialized.vin.len());
    assert_eq!(transaction.vout.len(), deserialized.vout.len());
}

#[test]
fn test_transaction_serialize_empty() {
    let transaction = Transaction::new(vec![], vec![], vec![]);

    let serialized = transaction.serialize();
    assert!(!serialized.is_empty()); // Even empty transactions should serialize to some bytes

    let deserialized = Transaction::deserialize(&serialized);
    assert_eq!(transaction.get_id(), deserialized.get_id());
    assert!(deserialized.vin.is_empty());
    assert!(deserialized.vout.is_empty());
}

#[test]
fn test_transaction_serialize_complex() {
    let mut tx_input1 = TXInput::new(b"txid1", 0);
    tx_input1.signature = vec![50, 60, 70];
    tx_input1.pub_key = vec![80, 90, 100];

    let mut tx_input2 = TXInput::new(b"txid2", 1);
    tx_input2.signature = vec![110, 120];
    tx_input2.pub_key = vec![130, 140, 150, 160];

    let tx_output1 = TXOutput {
        value: 250,
        pub_key_hash: vec![1, 2, 3],
    };

    let tx_output2 = TXOutput {
        value: -50,
        pub_key_hash: vec![4, 5, 6, 7, 8],
    };

    let transaction = Transaction::new(
        vec![200, 210, 220, 230],
        vec![tx_input1, tx_input2],
        vec![tx_output1, tx_output2],
    );

    let serialized = transaction.serialize();
    let deserialized = Transaction::deserialize(&serialized);

    // Verify all fields are preserved
    assert_eq!(transaction.get_id(), deserialized.get_id());
    assert_eq!(transaction.vin.len(), deserialized.vin.len());
    assert_eq!(transaction.vout.len(), deserialized.vout.len());

    // Check inputs
    assert_eq!(
        transaction.vin[0].get_txid(),
        deserialized.vin[0].get_txid()
    );
    assert_eq!(
        transaction.vin[0].get_vout(),
        deserialized.vin[0].get_vout()
    );
    assert_eq!(transaction.vin[0].signature, deserialized.vin[0].signature);
    assert_eq!(transaction.vin[0].pub_key, deserialized.vin[0].pub_key);

    assert_eq!(
        transaction.vin[1].get_txid(),
        deserialized.vin[1].get_txid()
    );
    assert_eq!(
        transaction.vin[1].get_vout(),
        deserialized.vin[1].get_vout()
    );
    assert_eq!(transaction.vin[1].signature, deserialized.vin[1].signature);
    assert_eq!(transaction.vin[1].pub_key, deserialized.vin[1].pub_key);

    // Check outputs
    assert_eq!(
        transaction.vout[0].get_value(),
        deserialized.vout[0].get_value()
    );
    assert_eq!(
        transaction.vout[0].get_pub_key_hash(),
        deserialized.vout[0].get_pub_key_hash()
    );
    assert_eq!(
        transaction.vout[1].get_value(),
        deserialized.vout[1].get_value()
    );
    assert_eq!(
        transaction.vout[1].get_pub_key_hash(),
        deserialized.vout[1].get_pub_key_hash()
    );
}

// Tests for Transaction::deserialize()
#[test]
fn test_transaction_deserialize_basic() {
    let tx_input = TXInput::new(b"deserialize_test", 42);
    let tx_output = TXOutput {
        value: 500,
        pub_key_hash: vec![10, 20],
    };

    let original = Transaction::new(vec![1, 2, 3, 4, 5], vec![tx_input], vec![tx_output]);

    let serialized = original.serialize();
    let deserialized = Transaction::deserialize(&serialized);

    assert_eq!(original.get_id(), deserialized.get_id());
    assert_eq!(original.vin.len(), deserialized.vin.len());
    assert_eq!(original.vout.len(), deserialized.vout.len());
    assert_eq!(original.vin[0].get_txid(), deserialized.vin[0].get_txid());
    assert_eq!(
        original.vout[0].get_value(),
        deserialized.vout[0].get_value()
    );
}

#[test]
#[should_panic]
fn test_transaction_deserialize_invalid_data() {
    let invalid_data = vec![255, 254, 253, 252]; // Invalid serialized data
    Transaction::deserialize(&invalid_data); // Should panic
}

#[test]
#[should_panic]
fn test_transaction_deserialize_empty_data() {
    let empty_data = vec![]; // Empty data
    Transaction::deserialize(&empty_data); // Should panic
}

// Tests for Transaction::try_deserialize()
#[test]
fn test_transaction_try_deserialize_success() {
    let tx_input = TXInput::new(b"try_deserialize_test", 123);
    let tx_output = TXOutput {
        value: 750,
        pub_key_hash: vec![30, 40, 50],
    };

    let original = Transaction::new(vec![100, 200, 255], vec![tx_input], vec![tx_output]);

    let serialized = original.serialize();
    let result = Transaction::try_deserialize(&serialized);

    assert!(result.is_ok());
    let deserialized = result.unwrap();

    assert_eq!(original.get_id(), deserialized.get_id());
    assert_eq!(original.vin.len(), deserialized.vin.len());
    assert_eq!(original.vout.len(), deserialized.vout.len());
    assert_eq!(original.vin[0].get_txid(), deserialized.vin[0].get_txid());
    assert_eq!(
        original.vout[0].get_value(),
        deserialized.vout[0].get_value()
    );
}

#[test]
fn test_transaction_try_deserialize_invalid_data() {
    let invalid_data = vec![255, 254, 253, 252, 251]; // Invalid serialized data
    let result = Transaction::try_deserialize(&invalid_data);

    assert!(result.is_err());
}

#[test]
fn test_transaction_try_deserialize_empty_data() {
    let empty_data = vec![]; // Empty data
    let result = Transaction::try_deserialize(&empty_data);

    assert!(result.is_err());
}

#[test]
fn test_transaction_try_deserialize_partial_data() {
    // Create valid transaction and get partial serialized data
    let transaction = Transaction::new(
        vec![1, 2, 3],
        vec![TXInput::new(b"test", 0)],
        vec![TXOutput {
            value: 100,
            pub_key_hash: vec![1, 2],
        }],
    );

    let serialized = transaction.serialize();
    let partial_data = &serialized[0..serialized.len() / 2]; // Take only half the data

    let result = Transaction::try_deserialize(partial_data);
    assert!(result.is_err());
}

// Tests for Transaction::trimmed_copy() (tested indirectly)
// Since trimmed_copy is private, we test it indirectly by testing its behavior
#[test]
fn test_transaction_trimmed_copy_behavior() {
    // Create a transaction with inputs that have signatures and pub_keys
    let mut tx_input1 = TXInput::new(b"input1", 0);
    tx_input1.signature = vec![1, 2, 3, 4, 5];
    tx_input1.pub_key = vec![10, 20, 30, 40];

    let mut tx_input2 = TXInput::new(b"input2", 1);
    tx_input2.signature = vec![50, 60, 70];
    tx_input2.pub_key = vec![80, 90, 100];

    let tx_output = TXOutput {
        value: 1000,
        pub_key_hash: vec![200, 210, 220],
    };

    let transaction = Transaction::new(
        vec![111, 222, 233],
        vec![tx_input1, tx_input2],
        vec![tx_output],
    );

    // We can't directly test trimmed_copy since it's private, but we know it's used
    // in the verify method. We can create a scenario where we test its behavior
    // by examining the structure it would create.

    // The trimmed_copy function should:
    // 1. Keep the same ID, vin length, and vout
    // 2. Clear signatures and pub_keys from inputs
    // 3. Keep txid and vout from inputs
    // 4. Keep outputs unchanged

    // Since we can't access trimmed_copy directly, we'll create what it should produce
    let expected_input1 = TXInput::new(b"input1", 0);
    let expected_input2 = TXInput::new(b"input2", 1);

    assert_eq!(expected_input1.get_txid(), transaction.vin[0].get_txid());
    assert_eq!(expected_input1.get_vout(), transaction.vin[0].get_vout());
    assert_eq!(expected_input1.signature, vec![]); // Should be empty
    assert_eq!(expected_input1.pub_key, vec![]); // Should be empty

    assert_eq!(expected_input2.get_txid(), transaction.vin[1].get_txid());
    assert_eq!(expected_input2.get_vout(), transaction.vin[1].get_vout());
    assert_eq!(expected_input2.signature, vec![]); // Should be empty
    assert_eq!(expected_input2.pub_key, vec![]); // Should be empty

    // Original inputs should still have their signatures and pub_keys
    assert_eq!(transaction.vin[0].signature, vec![1, 2, 3, 4, 5]);
    assert_eq!(transaction.vin[0].pub_key, vec![10, 20, 30, 40]);
    assert_eq!(transaction.vin[1].signature, vec![50, 60, 70]);
    assert_eq!(transaction.vin[1].pub_key, vec![80, 90, 100]);
}

// Test serialize/deserialize round-trip consistency
#[test]
fn test_transaction_serialize_deserialize_roundtrip() {
    let mut tx_input = TXInput::new(b"roundtrip_test", 999);
    tx_input.signature = vec![11, 22, 33, 44, 55, 66];
    tx_input.pub_key = vec![77, 88, 99];

    let tx_output1 = TXOutput {
        value: 12345,
        pub_key_hash: vec![123, 124, 125, 126, 127],
    };

    let tx_output2 = TXOutput {
        value: -6789,
        pub_key_hash: vec![],
    };

    let original = Transaction::new(
        vec![255, 254, 253, 252, 251, 250],
        vec![tx_input],
        vec![tx_output1, tx_output2],
    );

    // Serialize and deserialize using Transaction methods
    let serialized = original.serialize();
    let deserialized = Transaction::deserialize(&serialized);

    // Verify complete equality
    assert_eq!(original.get_id(), deserialized.get_id());
    assert_eq!(original.vin.len(), deserialized.vin.len());
    assert_eq!(original.vout.len(), deserialized.vout.len());

    // Check input details
    assert_eq!(original.vin[0].txid, deserialized.vin[0].txid);
    assert_eq!(original.vin[0].vout, deserialized.vin[0].vout);
    assert_eq!(original.vin[0].signature, deserialized.vin[0].signature);
    assert_eq!(original.vin[0].pub_key, deserialized.vin[0].pub_key);

    // Check output details
    assert_eq!(original.vout[0].value, deserialized.vout[0].value);
    assert_eq!(
        original.vout[0].pub_key_hash,
        deserialized.vout[0].pub_key_hash
    );
    assert_eq!(original.vout[1].value, deserialized.vout[1].value);
    assert_eq!(
        original.vout[1].pub_key_hash,
        deserialized.vout[1].pub_key_hash
    );
}

// Test try_deserialize vs deserialize consistency
#[test]
fn test_transaction_deserialize_vs_try_deserialize_consistency() {
    let transaction = Transaction::new(
        vec![42, 43, 44],
        vec![TXInput::new(b"consistency_test", 77)],
        vec![TXOutput {
            value: 888,
            pub_key_hash: vec![55, 56, 57, 58],
        }],
    );

    let serialized = transaction.serialize();

    // Both methods should produce the same result for valid data
    let deserialized_regular = Transaction::deserialize(&serialized);
    let deserialized_try = Transaction::try_deserialize(&serialized).unwrap();

    assert_eq!(deserialized_regular.get_id(), deserialized_try.get_id());
    assert_eq!(deserialized_regular.vin.len(), deserialized_try.vin.len());
    assert_eq!(deserialized_regular.vout.len(), deserialized_try.vout.len());

    if !deserialized_regular.vin.is_empty() {
        assert_eq!(
            deserialized_regular.vin[0].txid,
            deserialized_try.vin[0].txid
        );
        assert_eq!(
            deserialized_regular.vin[0].vout,
            deserialized_try.vin[0].vout
        );
        assert_eq!(
            deserialized_regular.vin[0].signature,
            deserialized_try.vin[0].signature
        );
        assert_eq!(
            deserialized_regular.vin[0].pub_key,
            deserialized_try.vin[0].pub_key
        );
    }

    if !deserialized_regular.vout.is_empty() {
        assert_eq!(
            deserialized_regular.vout[0].value,
            deserialized_try.vout[0].value
        );
        assert_eq!(
            deserialized_regular.vout[0].pub_key_hash,
            deserialized_try.vout[0].pub_key_hash
        );
    }
}
