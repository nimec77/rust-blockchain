use bincode::{Decode, Encode};
use num_bigint::BigInt;

use crate::common::BincodeBigInt;

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
