use bincode::{Decode, Encode};
use num_bigint::BigInt;

#[derive(Clone)]
pub struct BincodeBigInt(pub BigInt);

