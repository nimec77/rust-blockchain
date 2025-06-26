use crate::models::{tx_input::TXInput, tx_output::TXOutput};

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id() {
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
    fn test_get_id_empty() {
        // Test with an empty ID
        let transaction = Transaction {
            id: vec![],
            vin: vec![],
            vout: vec![],
        };

        assert_eq!(transaction.get_id(), &[]);
    }
}

