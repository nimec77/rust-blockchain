use crate::transaction::Transaction;

impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        &self.id
    }
}
