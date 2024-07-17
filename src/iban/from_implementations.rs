use super::IBAN;
use crate::Error;

impl TryFrom<String> for IBAN {
    type Error = crate::Error;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let input = value.replace(" ", "").to_uppercase();

        if input.len() > 34 {
            return Err(Error::WrongIBANSize);
        }

        let mut iban = IBAN::new();
        for (i, ch) in input.chars().enumerate() {
            if !ch.is_ascii_alphanumeric() {
                return Err(Error::NotAnIBAN);
            }
            iban[i] = ch;
        }

        Ok(iban)
    }
}

impl TryFrom<&str> for IBAN {
    type Error = crate::Error;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let input = value.replace(" ", "").to_uppercase();

        if input.len() > 34 {
            return Err(Error::WrongIBANSize);
        }

        let mut iban = IBAN::new();
        for (i, ch) in input.chars().enumerate() {
            if !ch.is_ascii_alphanumeric() {
                return Err(Error::NotAnIBAN);
            }
            iban[i] = ch;
        }

        Ok(iban)
    }
}

impl TryFrom<Vec<u8>> for IBAN {
    type Error = crate::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > 34 {
            return Err(Error::WrongIBANSize);
        }

        let mut iban = IBAN::new();
        let input = value.iter().map(|b| *b as char).collect::<Vec<char>>();
        for i in 0..input.len() {
            // cant be none since the range is known
            let ch = input.get(i).unwrap();
            if !ch.is_ascii() {
                return Err(Error::NotAnIBAN);
            }
            iban[i] = *ch;
        }

        Ok(iban)
    }
}

impl From<[char; 34]> for IBAN {
    fn from(value: [char; 34]) -> Self {
        IBAN(value)
    }
}