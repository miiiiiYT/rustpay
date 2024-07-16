pub use currency::Currency;
use p256::ecdsa::Signature;

use crate::{merchant::Merchant, user::User};

mod currency;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    // amount is counted in thousandths, e.g. 1€ equals amount = 1000
    amount: u64,
    currency: Currency,
    merchant: Merchant,
    user: User,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignedTransaction {
    transaction: Transaction,
    signature: Signature,
}

impl Transaction {
    pub fn new(amount: u64, currency: Currency, merchant: Merchant, user: User) -> Self {
        Self { amount, currency, merchant, user }
    }
}