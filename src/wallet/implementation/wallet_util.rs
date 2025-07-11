use crate::{util, wallet::data::wallet::{ADDRESS_CHECK_SUM_LEN, VERSION}};

pub fn validate_address(address: &str) -> bool {
    // Handle empty or invalid base58
    if address.is_empty() {
        return false;
    }
    
    let payload = util::base58_decode(address);
    
    // Check minimum length: version (1) + at least some pub_key_hash + checksum (4)
    if payload.len() < 1 + ADDRESS_CHECK_SUM_LEN {
        return false;
    }
    
    // Check if payload is empty (invalid base58 decode)
    if payload.is_empty() {
        return false;
    }
    
    let actual_checksum = payload[payload.len() - ADDRESS_CHECK_SUM_LEN..].to_vec();
    let version = payload[0];
    
    // Check version
    if version != VERSION {
        return false;
    }
    
    let pub_key_hash = payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN].to_vec();

    let mut target_vec = vec![];
    target_vec.push(version);
    target_vec.extend(pub_key_hash);
    let target_checksum = checksum(target_vec.as_slice());
    actual_checksum.eq(target_checksum.as_slice())
}

pub fn convert_address(pub_hash_key: &[u8]) -> String {
    let mut payload: Vec<u8> = vec![];
    payload.push(VERSION);
    payload.extend(pub_hash_key);
    let checksum = checksum(payload.as_slice());
    payload.extend(checksum.as_slice());
    util::base58_encode(payload.as_slice())
}

pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = util::sha256_digest(pub_key);
    util::ripemd160_digest(pub_key_sha256.as_slice())
}

pub fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = util::sha256_digest(payload);
    let second_sha = util::sha256_digest(first_sha.as_slice());
    second_sha[0..ADDRESS_CHECK_SUM_LEN].to_vec()
}
