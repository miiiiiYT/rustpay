use p256::ecdsa::{SigningKey, VerifyingKey};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    account_number: String,
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl User {
    pub fn new(account_number: String) -> Self {
        let mut rng = ChaCha20Rng::from_entropy();
        let signing_key = SigningKey::random(&mut rng);
        let verifying_key = VerifyingKey::from(&signing_key);

        User { account_number, signing_key, verifying_key }
    }
}