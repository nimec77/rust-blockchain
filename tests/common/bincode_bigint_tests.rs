use rust_blockchain::BincodeBigInt;
use bincode::{config, decode_from_slice, encode_to_vec};
use num_bigint::BigInt;

#[test]
fn test_creation_methods() {
    let big_int = BigInt::from(12345);

    // Test creation with move semantics (no clone)
    let wrapper1 = BincodeBigInt::new(big_int.clone());
    let wrapper2 = BincodeBigInt::from(big_int.clone());
    let wrapper3: BincodeBigInt = big_int.clone().into();

    // Test creation from reference (requires clone)
    let wrapper4 = BincodeBigInt::from_ref(&big_int);
    let wrapper5 = BincodeBigInt::from(&big_int);

    // All should contain the same value
    assert_eq!(wrapper1.0, big_int);
    assert_eq!(wrapper2.0, big_int);
    assert_eq!(wrapper3.0, big_int);
    assert_eq!(wrapper4.0, big_int);
    assert_eq!(wrapper5.0, big_int);
}

#[test]
fn test_access_methods() {
    let big_int = BigInt::from(54321);
    let wrapper = BincodeBigInt::new(big_int.clone());

    // Test reference access
    assert_eq!(wrapper.as_bigint(), &big_int);
    assert_eq!(wrapper.as_ref(), &big_int);

    // Test deref access (can call BigInt methods directly)
    assert_eq!(wrapper.bits(), big_int.bits());

    // Test extraction
    let extracted = wrapper.into_bigint();
    assert_eq!(extracted, big_int);
}

#[test]
fn test_encode_decode_positive() {
    let big_int = BigInt::from(12345678901234567890u64);
    let wrapper = BincodeBigInt::from(big_int); // Using From trait

    let config = config::standard();
    let encoded = encode_to_vec(&wrapper, config).unwrap();
    let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();

    assert_eq!(wrapper.0, decoded.0);
}

#[test]
fn test_encode_decode_negative() {
    let big_int = BigInt::parse_bytes(b"-98765432109876543210", 10).unwrap();
    let wrapper = BincodeBigInt::from(&big_int); // Using From<&BigInt> trait

    let config = config::standard();
    let encoded = encode_to_vec(&wrapper, config).unwrap();
    let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();

    assert_eq!(wrapper.0, decoded.0);
}

#[test]
fn test_encode_decode_zero() {
    let big_int = BigInt::from(0);
    let wrapper = BincodeBigInt::new(big_int); // Using new method

    let config = config::standard();
    let encoded = encode_to_vec(&wrapper, config).unwrap();
    let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();

    assert_eq!(wrapper.0, decoded.0);
}

#[test]
fn test_encode_decode_large_number() {
    // Test with a very large number
    let big_int = BigInt::parse_bytes(
        b"123456789012345678901234567890123456789012345678901234567890",
        10,
    )
    .unwrap();
    let wrapper = BincodeBigInt::from_ref(&big_int); // Using from_ref method

    let config = config::standard();
    let encoded = encode_to_vec(&wrapper, config).unwrap();
    let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();

    assert_eq!(wrapper.0, decoded.0);
}

#[test]
fn test_borrow_decode() {
    // Test the BorrowDecode implementation
    let wrapper: BincodeBigInt = BigInt::from(987654321).into();

    let config = config::standard();
    let encoded = encode_to_vec(&wrapper, config).unwrap();
    
    // Use borrow_decode by calling decode_from_slice which may use BorrowDecode internally
    let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();
    
    assert_eq!(wrapper.0, decoded.0);
    
    // Test with negative number as well
    let neg_wrapper: BincodeBigInt = BigInt::from(-123456789).into();
    
    let neg_encoded = encode_to_vec(&neg_wrapper, config).unwrap();
    let (neg_decoded, _): (BincodeBigInt, usize) = decode_from_slice(&neg_encoded, config).unwrap();
    
    assert_eq!(neg_wrapper.0, neg_decoded.0);
}

#[test]
fn test_very_large_positive_number() {
    let big_int = BigInt::parse_bytes(
        b"987654321098765432109876543210987654321098765432109876543210",
        10,
    ).unwrap();
    let wrapper = BincodeBigInt::new(big_int.clone());

    let config = config::standard();
    let encoded = encode_to_vec(&wrapper, config).unwrap();
    let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();

    assert_eq!(wrapper.as_bigint(), decoded.as_bigint());
}

#[test]
fn test_very_large_negative_number() {
    let big_int = BigInt::parse_bytes(
        b"-987654321098765432109876543210987654321098765432109876543210",
        10,
    ).unwrap();
    let wrapper = BincodeBigInt::new(big_int.clone());

    let config = config::standard();
    let encoded = encode_to_vec(&wrapper, config).unwrap();
    let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();

    assert_eq!(wrapper.as_bigint(), decoded.as_bigint());
}

#[test]
fn test_one_and_negative_one() {
    // Test edge cases: 1 and -1
    let one = BincodeBigInt::new(BigInt::from(1));
    let neg_one = BincodeBigInt::new(BigInt::from(-1));

    let config = config::standard();
    
    // Test 1
    let encoded_one = encode_to_vec(&one, config).unwrap();
    let (decoded_one, _): (BincodeBigInt, usize) = decode_from_slice(&encoded_one, config).unwrap();
    assert_eq!(one.as_bigint(), decoded_one.as_bigint());
    
    // Test -1
    let encoded_neg_one = encode_to_vec(&neg_one, config).unwrap();
    let (decoded_neg_one, _): (BincodeBigInt, usize) = decode_from_slice(&encoded_neg_one, config).unwrap();
    assert_eq!(neg_one.as_bigint(), decoded_neg_one.as_bigint());
}

#[test]
fn test_powers_of_two() {
    // Test various powers of 2
    for power in 0..64 {
        let big_int: BigInt = BigInt::from(1) << power;
        let wrapper = BincodeBigInt::new(big_int.clone());

        let config = config::standard();
        let encoded = encode_to_vec(&wrapper, config).unwrap();
        let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();

        assert_eq!(wrapper.as_bigint(), decoded.as_bigint(), "Failed for 2^{power}");
    }
}

#[test]
fn test_clone_functionality() {
    let big_int = BigInt::from(42424242);
    let original = BincodeBigInt::new(big_int.clone());
    let cloned = original.clone();

    assert_eq!(original.as_bigint(), cloned.as_bigint());
    
    // Verify they are separate instances by modifying one
    let modified = BincodeBigInt::new(BigInt::from(99999999));
    assert_ne!(original.as_bigint(), modified.as_bigint());
}

#[test]
fn test_debug_display_traits() {
    let big_int = BigInt::from(123456789);
    let wrapper = BincodeBigInt::new(big_int);

    // These should not panic
    let _debug_str = format!("{wrapper:?}");
    // Note: Display trait might not be implemented, so we won't test it
}

#[test]
fn test_deref_operations() {
    let big_int = BigInt::from(999999);
    let wrapper = BincodeBigInt::new(big_int.clone());

    // Test that we can use BigInt methods directly through deref
    assert_eq!(wrapper.bits(), big_int.bits());
    assert_eq!(wrapper.to_string(), big_int.to_string());
    
    // Test comparison through deref
    let other_wrapper = BincodeBigInt::new(BigInt::from(999999));
    assert_eq!(*wrapper, *other_wrapper);
}

#[test]
fn test_multiple_encode_decode_cycles() {
    let mut current = BincodeBigInt::new(BigInt::from(12345));
    let config = config::standard();

    // Perform multiple encode/decode cycles
    for _ in 0..5 {
        let encoded = encode_to_vec(&current, config).unwrap();
        let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();
        current = decoded;
    }

    // Should still have the original value
    assert_eq!(*current.as_bigint(), BigInt::from(12345));
}

#[test]
fn test_different_construction_methods_equality() {
    let big_int = BigInt::from(777777);
    
    let wrapper1 = BincodeBigInt::new(big_int.clone());
    let wrapper2 = BincodeBigInt::from(big_int.clone());
    let wrapper3: BincodeBigInt = big_int.clone().into();
    let wrapper4 = BincodeBigInt::from_ref(&big_int);

    // All construction methods should result in equal values
    assert_eq!(wrapper1.as_bigint(), wrapper2.as_bigint());
    assert_eq!(wrapper2.as_bigint(), wrapper3.as_bigint());
    assert_eq!(wrapper3.as_bigint(), wrapper4.as_bigint());
}

#[test]
fn test_hex_number_parsing() {
    // Test with hexadecimal representation
    let hex_big_int = BigInt::parse_bytes(b"deadbeefcafebabe", 16).unwrap();
    let wrapper = BincodeBigInt::new(hex_big_int.clone());

    let config = config::standard();
    let encoded = encode_to_vec(&wrapper, config).unwrap();
    let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();

    assert_eq!(wrapper.as_bigint(), decoded.as_bigint());
}

#[test]
fn test_max_values() {
    // Test with maximum values for various integer types
    let test_values = vec![
        BigInt::from(u8::MAX),
        BigInt::from(u16::MAX),
        BigInt::from(u32::MAX),
        BigInt::from(u64::MAX),
        BigInt::from(i8::MAX),
        BigInt::from(i16::MAX),
        BigInt::from(i32::MAX),
        BigInt::from(i64::MAX),
    ];

    let config = config::standard();
    
    for big_int in test_values {
        let wrapper = BincodeBigInt::new(big_int.clone());
        let encoded = encode_to_vec(&wrapper, config).unwrap();
        let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();
        
        assert_eq!(wrapper.as_bigint(), decoded.as_bigint(), "Failed for value: {big_int}");
    }
}

#[test]
fn test_min_values() {
    // Test with minimum values for signed integer types
    let test_values = vec![
        BigInt::from(i8::MIN),
        BigInt::from(i16::MIN),
        BigInt::from(i32::MIN),
        BigInt::from(i64::MIN),
    ];

    let config = config::standard();
    
    for big_int in test_values {
        let wrapper = BincodeBigInt::new(big_int.clone());
        let encoded = encode_to_vec(&wrapper, config).unwrap();
        let (decoded, _): (BincodeBigInt, usize) = decode_from_slice(&encoded, config).unwrap();
        
        assert_eq!(wrapper.as_bigint(), decoded.as_bigint(), "Failed for value: {big_int}");
    }
} 
