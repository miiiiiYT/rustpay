use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NotAnIBAN,
    WrongIBANSize,
    NoPrivateKey,
    DevError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Error {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotAnIBAN => "the provided string was unable to be converted into an iban",
            Self::WrongIBANSize => "the provided string is too long or too short to be an iban",
            Self::NoPrivateKey => "the entity did not contain a private (signing) key",
            Self::DevError => "error for testing",
        }
    }
}