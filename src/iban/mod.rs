mod countrycodes;

use std::ops::{Deref, DerefMut};

use countrycodes::CountryCode;

use crate::Error;

/// IBAN length by country code
const IBAN_LENGTHS: &[(&str, usize)] = &[
    ("AL", 28), ("AD", 24), ("AT", 20), ("AZ", 28), ("BH", 22), ("BE", 16), ("BA", 20), ("BR", 29),
    ("BG", 22), ("CR", 22), ("HR", 21), ("CY", 28), ("CZ", 24), ("DK", 18), ("DO", 28), ("EE", 20),
    ("FI", 18), ("FR", 27), ("GE", 22), ("DE", 22), ("GI", 23), ("GR", 27), ("GL", 18), ("GT", 28),
    ("HU", 28), ("IS", 26), ("IE", 22), ("IL", 23), ("IT", 27), ("KZ", 20), ("KW", 30), ("LV", 21),
    ("LB", 28), ("LI", 21), ("LT", 20), ("LU", 20), ("MK", 19), ("MT", 31), ("MR", 27), ("MU", 30),
    ("MC", 27), ("MD", 24), ("ME", 22), ("NL", 18), ("NO", 15), ("PK", 24), ("PS", 29), ("PL", 28),
    ("PT", 25), ("RO", 24), ("SM", 27), ("SA", 24), ("RS", 22), ("SK", 24), ("SI", 19), ("ES", 24),
    ("SE", 24), ("CH", 21), ("TN", 24), ("TR", 26), ("AE", 23), ("GB", 22), ("VG", 24),
];

/// An international bank account number (IBAN).
/// 
/// Represented by a slice of 34 chars, which is the max length for an IBAN.
/// 
/// If a country uses less than the full 34 digits, the rest will be padded
/// with NUL. It's ensured that NUL's are only on the right of content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IBAN([char; 34]);

impl IBAN {
    /// Create a new IBAN, with all fields set to NUL.
    pub fn new() -> Self {
        IBAN(
            ['\0'; 34]
        )
    }

    /// Returns the length of the IBAN, in it's current representation.
    /// 
    /// This is how it would actually be written, so disregarding all NUL chars.
    pub fn len(&self) -> usize {
        let mut length = 0;

        for c in **self {
            if c != '\0' {
                length += 1;
            } else {
                break;
            }
        }

        length
    }

    /// Sets the country of the IBAN.
    pub fn set_country(&mut self, country: CountryCode) {
        let code = country.as_code();
        // slice accesses are valid since the slice must have 34 elements
        self[0] = code.0;
        self[1] = code.1;
    }

    /// Creates a new String from IBAN
    pub fn to_string(&self) -> String {
        self.iter().filter(|c| c.is_ascii()).collect::<String>()
    }

    pub fn is_valid(&self) -> bool {
        let iban = self.to_string();

        if iban.len() < 4 {
            return false;
        }

        let country_code = &iban[0..2];
        let iban_length = self.len();
        let expected_length = IBAN_LENGTHS.iter()
            .find(|&&(code, _)| code == country_code)
            .map(|&(_, length)| length);

        if Some(iban_length) != expected_length {
            return false;
        }

        // Rearrange: Move the first four characters to the end of the string
        let rearranged_iban = format!("{}{}", &iban[4..], &iban[0..4]);

        // Convert characters to digits
        let numeric_iban = rearranged_iban.chars().filter_map(|c| {
            match c {
                '0'..='9' => Some(c.to_digit(10).unwrap() as u8),
                'A'..='Z' => Some((c as u8 - 'A' as u8 + 10) as u8),
                _ => None,
            }
        }).collect::<Vec<_>>();

        // Convert the Vec<u8> to a single large number string
        let numeric_iban_str = numeric_iban.iter()
            .map(|&num| num.to_string())
            .collect::<String>();

        // Perform the Modulo 97 operation
        let mut remainder = 0u128;
        for chunk in numeric_iban_str.as_bytes().chunks(9) {
            let part_str = std::str::from_utf8(chunk).unwrap();
            let part_num: u128 = part_str.parse().unwrap();
            remainder = (remainder * 10u128.pow(part_str.len() as u32) + part_num) % 97;
        }

        remainder == 1
    }
}

impl Deref for IBAN {
    type Target = [char; 34];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IBAN {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }    
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_length() {
        let iban = IBAN::new();
        assert_eq!(iban.len(), 0);
    }

    #[test]
    fn length() {
        assert!(true)
    }

    #[test]
    fn correct_country() {
        let mut iban = IBAN::new();
        let country = CountryCode::DE;

        iban.set_country(country);
        
        let first_two_digits = (iban[0], iban[1]);
        assert_eq!(first_two_digits, country.as_code());
    }

    #[test]
    fn create_from() {
        let ibans = [
            "NL66 RABO 9942 4087 54",
            "DE15500105174256634144",
            "AT140027100972005647",
        ];

        for ib in ibans {
            let iban = IBAN::try_from(ib);
            assert!(iban.is_ok())
        }
    }

    #[test]
    fn create_from_fail() {
        let ibans = [
            "invalid_iban",
            "!§!§",
            "5123 1234 8373 1i28 asj23234234234234234dlk sa  832 912 9812",
        ];

        for ib in ibans {
            let iban = IBAN::try_from(ib);
            assert!(iban.is_err())
        }
    }

    #[test]
    fn check_validity() {
        let test_ibans = vec![
            "GB82 WEST 1234 5698 7654 32",
            "DE89 3704 0044 0532 0130 00",
            "FR14 2004 1010 0505 0001 3M02 606",
            "GR16 0110 1250 0000 0001 2300 695",
            "DE51 2131 1231 5532 1234 42",
            "GB54 AAAA BBBB CCCC DDDD EE",
            "IB",
            "DD14 2004 1010 0505 0001 3M02 606",
            "DE22 8472 162",
        ];

        let mut results: Vec<bool> = vec![];

        for iban in test_ibans {
            results.push(IBAN::try_from(iban).unwrap().is_valid())
        }

        let expected_results = vec![
            true,
            true,
            true,
            true,
            false,
            false,
            false,
            false,
            false,
        ];

        assert_eq!(results, expected_results)
    }
}