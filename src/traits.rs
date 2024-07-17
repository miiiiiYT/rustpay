use crate::transaction::{SignedTransaction, Transaction};

pub trait TransactionSign {
    /// Makes the caller sign a transaction.
    fn sign(&self, transaction: Transaction) -> SignedTransaction;
}

pub trait ToBytes {
    /// Returns `self` as a byte slice.
    fn as_bytes(&self) -> &[u8];
}