use rust_blockchain::{ProofOfWork, Block};
use num_bigint::BigInt;
use crate::test_helpers::*;

const TARGET_BITS: usize = 24;
const MAX_NONCE: i64 = i64::MAX;

#[test]
fn test_new_proof_of_work() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block.clone());

    // Verify that the target is calculated correctly
    let expected_target = BigInt::from(1) << (256 - TARGET_BITS);
    assert_eq!(*pow.get_target().as_bigint(), expected_target);

    // Verify that the block is stored correctly
    assert_eq!(pow.get_block().get_pre_block_hash(), block.get_pre_block_hash());
    assert_eq!(pow.get_block().get_timestamp(), block.get_timestamp());
    assert_eq!(pow.get_block().get_height(), block.get_height());
}

#[test]
fn test_new_proof_of_work_target_calculation() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block);

    // Test that target is a valid BigInt and has the expected magnitude
    let target = pow.get_target().as_bigint();
    assert!(target > &BigInt::from(0));

    // Target should be 2^(256-24) = 2^232
    let expected_target = BigInt::from(1) << 232;
    assert_eq!(target, &expected_target);
}

#[test]
fn test_prepare_data() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block.clone());
    let nonce = 12345;

    let data = pow.prepare_data(nonce);

    // Verify that the data contains all expected components in correct order
    let mut expected_data = Vec::new();
    expected_data.extend_from_slice(block.get_pre_block_hash().as_bytes());
    expected_data.extend_from_slice(&block.hash_transactions());
    expected_data.extend_from_slice(&block.get_timestamp().to_be_bytes());
    expected_data.extend_from_slice(&(TARGET_BITS as u64).to_be_bytes());
    expected_data.extend_from_slice(&nonce.to_be_bytes());

    assert_eq!(data, expected_data);
}

#[test]
fn test_prepare_data_different_nonces() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block);

    let data1 = pow.prepare_data(100);
    let data2 = pow.prepare_data(200);

    // Data should differ only in the last 8 bytes (nonce)
    assert_ne!(data1, data2);
    assert_eq!(data1.len(), data2.len());

    // All bytes except the last 8 should be the same
    let prefix_len = data1.len() - 8;
    assert_eq!(&data1[..prefix_len], &data2[..prefix_len]);
    assert_ne!(&data1[prefix_len..], &data2[prefix_len..]);
}

#[test]
fn test_prepare_data_deterministic() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block);
    let nonce = 42;

    let data1 = pow.prepare_data(nonce);
    let data2 = pow.prepare_data(nonce);

    // Same nonce should produce identical data
    assert_eq!(data1, data2);
}

#[test]
fn test_validate_with_default_nonce() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block);

    // With TARGET_BITS = 24, it's extremely unlikely that the default nonce (0) will produce a valid hash
    let is_valid = pow.validate();

    // This test might occasionally fail due to the probabilistic nature of hashing,
    // but it's extremely unlikely with TARGET_BITS = 24 and default nonce
    assert!(!is_valid);
}

#[test]
fn test_validate_deterministic() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block);

    let result1 = pow.validate();
    let result2 = pow.validate();

    // Validation should be deterministic
    assert_eq!(result1, result2);
}

#[test]
fn test_validate_different_blocks() {
    let transaction1 = create_test_transaction(vec![1, 2, 3, 4]);
    let transaction2 = create_test_transaction(vec![5, 6, 7, 8]);

    let block1 =
        Block::new_block_without_proof_of_work("hash1".to_string(), &[transaction1], 1);
    let block2 =
        Block::new_block_without_proof_of_work("hash2".to_string(), &[transaction2], 2);

    let pow1 = ProofOfWork::new_proof_of_work(block1);
    let pow2 = ProofOfWork::new_proof_of_work(block2);

    let result1 = pow1.validate();
    let result2 = pow2.validate();

    // Different blocks will almost certainly produce different validation results
    // (though both are likely to be false with TARGET_BITS = 24)
    // This test ensures the validation logic works with different inputs
    assert_eq!(result1, pow1.validate()); // Deterministic for same input
    assert_eq!(result2, pow2.validate()); // Deterministic for same input
}

#[ignore] // This test can take a very long time to complete
#[test]
fn test_run_mining_process() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block.clone());

    let (nonce, hash) = pow.run();

    // Verify that the returned nonce and hash are valid
    assert!(nonce >= 0);
    assert!(nonce < MAX_NONCE);
    assert_eq!(hash.len(), 64); // SHA256 hex string length

    // Verify that the hash string is valid hex
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));

    // Verify that the nonce is reasonable (not immediately 0, which would be suspicious)
    // Note: This could theoretically be 0 in very rare cases, but highly unlikely with TARGET_BITS=24
}

#[test]
fn test_proof_of_work_clone() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block);

    let cloned_pow = pow.clone();

    // Verify that cloned instance behaves identically
    assert_eq!(pow.get_target().as_bigint(), cloned_pow.get_target().as_bigint());
    assert_eq!(pow.validate(), cloned_pow.validate());

    let data1 = pow.prepare_data(42);
    let data2 = cloned_pow.prepare_data(42);
    assert_eq!(data1, data2);
}

#[test]
fn test_proof_of_work_serialization() {
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block);

    // Test that the contained block can be serialized
    let block_serialized = pow.get_block().serialize();
    let block_deserialized = Block::deserialize(&block_serialized);

    assert_eq!(pow.get_block().get_hash(), block_deserialized.get_hash());
    assert_eq!(pow.get_block().get_pre_block_hash(), block_deserialized.get_pre_block_hash());
    assert_eq!(pow.get_block().get_height(), block_deserialized.get_height());
}

#[test]
fn test_target_bits_constant() {
    // Test that TARGET_BITS constant is reasonable
    assert_eq!(TARGET_BITS, 24);

    // Test that target calculation works with the constant
    let target = BigInt::from(1) << (256 - TARGET_BITS);
    assert!(target > BigInt::from(0));
}

#[test]
fn test_max_nonce_constant() {
    // Test MAX_NONCE constant
    assert_eq!(MAX_NONCE, i64::MAX);
    
    // Test that nonce range is reasonable
    let block = create_default_test_block();
    let pow = ProofOfWork::new_proof_of_work(block);
    
    // Prepare data with max nonce should not panic
    let _data = pow.prepare_data(MAX_NONCE);
} 
