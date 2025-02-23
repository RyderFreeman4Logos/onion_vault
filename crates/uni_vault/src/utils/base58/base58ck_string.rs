use base58ck::{decode_check, encode_check};

use serde::{Serialize, Deserialize};

use anyhow::Result;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Base58CkString(String);

impl Base58CkString {
    /// Constructs a new `Base58CkString` from raw bytes.
    pub fn from_bytes(data: &[u8]) -> Self {
        Base58CkString(encode_check(data))
    }

    /// Decodes the Base58Check encoded string back to bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let res = decode_check(&self.0)?;
        Ok(res)
    }

    /// Converts the internal `String` to a Human-readable string.
    pub fn readable_string(self) -> Result<String> {
        // Convert bytes to String assuming the bytes represent valid UTF-8 data
        let res = String::from_utf8(self.to_bytes()?)?;
        Ok(res)
    }


    /// Converts the internal `String` to a Human-readable string.
    pub fn try_readable_string(self) -> String {
        let raw_string = self.to_string();
        // Convert bytes to String assuming the bytes represent valid UTF-8 data
        String::from_utf8(self.to_bytes().unwrap_or(raw_string.as_bytes().to_owned()))
            .unwrap_or(raw_string)
    }
}

impl From<&str> for Base58CkString {
    /// Creates a `Base58CkString` from an already encoded string. 
    /// This does not perform any validation; use with caution.
    fn from(s: &str) -> Self {
        Base58CkString(s.to_string())
    }
}

impl From<String> for Base58CkString {
    /// Creates a `Base58CkString` from an already encoded string. 
    /// This does not perform any validation; use with caution.
    fn from(s: String) -> Self {
        Base58CkString(s)
    }
}

impl std::fmt::Display for Base58CkString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes_to_bytes() {
        let data = vec![0x00, 0x01, 0x02, 0x03];
        let base58_check = Base58CkString::from_bytes(&data);
        let decoded = base58_check.to_bytes().unwrap();
        assert_eq!(data, decoded);
    }


    #[test]
    fn test_from_string() {
        let original_data = vec![0x00, 0x01, 0x02, 0x03];
        let encoded = Base58CkString::from_bytes(&original_data);
        let from_string = Base58CkString::from(encoded.to_string());
        assert_eq!(encoded, from_string);
    }

    #[test]
    fn test_error_case() {
        // Here we're testing with an invalid base58 check string
        let invalid_base58 = Base58CkString::from("InvalidBase58CkString".to_string());
        assert!(invalid_base58.to_bytes().is_err());
    }
}
