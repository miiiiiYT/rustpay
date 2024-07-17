//! rustpay is a more secure and sound way to handle centralized transactions.
//! 
//! 

pub mod bank;
pub mod merchant;
pub mod transaction;
pub mod user;
mod error;
mod iban;
pub mod traits;

pub use error::Error;
pub use iban::IBAN;