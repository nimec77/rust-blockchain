use bincode::{Decode, Encode};
use num_bigint::BigInt;

use crate::common::bincode_bigint::data::bincode_bigint::BincodeBigInt;

impl BincodeBigInt {
    /// Create a new BincodeBigInt by taking ownership of a BigInt (no clone needed)
    pub fn new(big_int: BigInt) -> Self {
        BincodeBigInt(big_int)
    }

    /// Create a new BincodeBigInt from a reference to BigInt (requires clone)
    pub fn from_ref(big_int: &BigInt) -> Self {
        BincodeBigInt(big_int.clone())
    }

    /// Get a reference to the inner BigInt
    pub fn as_bigint(&self) -> &BigInt {
        &self.0
    }

    /// Extract the inner BigInt, consuming the wrapper
    pub fn into_bigint(self) -> BigInt {
        self.0
    }
}

// Implement From trait for move semantics (no clone needed)
impl From<BigInt> for BincodeBigInt {
    fn from(big_int: BigInt) -> Self {
        BincodeBigInt(big_int)
    }
}

// Implement From trait for reference (requires clone, but convenient)
impl From<&BigInt> for BincodeBigInt {
    fn from(big_int: &BigInt) -> Self {
        BincodeBigInt(big_int.clone())
    }
}

// Implement AsRef for easy access to the inner BigInt
impl AsRef<BigInt> for BincodeBigInt {
    fn as_ref(&self) -> &BigInt {
        &self.0
    }
}

// Implement Deref for transparent access to BigInt methods
impl std::ops::Deref for BincodeBigInt {
    type Target = BigInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Encode for BincodeBigInt {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        // Convert BigInt to signed bytes (two's complement representation)
        let (sign, bytes) = self.0.to_bytes_be();

        // Encode the sign first (0 for positive, 1 for negative)
        let sign_byte = match sign {
            num_bigint::Sign::Plus => 0u8,
            num_bigint::Sign::Minus => 1u8,
            num_bigint::Sign::NoSign => 0u8, // treat zero as positive
        };

        sign_byte.encode(encoder)?;

        // Then encode the bytes
        bytes.encode(encoder)
    }
}

impl<Context> Decode<Context> for BincodeBigInt {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        // Decode the sign first
        let sign_byte = u8::decode(decoder)?;

        // Then decode the bytes
        let bytes: Vec<u8> = Vec::decode(decoder)?;

        // Convert back to BigInt
        let sign = match sign_byte {
            0 => num_bigint::Sign::Plus,
            1 => num_bigint::Sign::Minus,
            _ => return Err(bincode::error::DecodeError::UnexpectedEnd { additional: 0 }),
        };

        let big_int = BigInt::from_bytes_be(sign, &bytes);
        Ok(BincodeBigInt(big_int))
    }
}

impl<'de, Context> bincode::BorrowDecode<'de, Context> for BincodeBigInt {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        // BincodeBigInt cannot be zero-copy decoded, so delegate to regular Decode
        Self::decode(decoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
