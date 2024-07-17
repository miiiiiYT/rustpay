use crate::transaction::{SignedTransaction, Transaction};
use crate::Error;

pub trait TransactionSign {
    /// Makes the caller sign a transaction.
    fn sign(&self, transaction: Transaction) -> Result<SignedTransaction, Error>;
}

pub trait ToBytes {
    /// Returns `self` as an owned byte slice.
    fn as_bytes(&self) -> Vec<u8>;
}