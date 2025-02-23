// pub mod symmetric;
pub mod asymmetric;

use crate::re_export::std_anyhow::*;

use serde::{Serialize, Deserialize};


use std::fmt;



/// Trait for structures that have a user identifier. This includes methods for 
/// retrieving user-specific information like username, platform ID, and secret type.
pub trait HasUserIdentifier {
    /// Returns the username associated with this structure.
    fn username(&self) -> String;

    /// Returns the platform identifier for this structure.
    fn platform_id(&self) -> String;

    /// Returns the type of secret this structure is handling.
    fn secret_type(&self) -> SecretType;

    /// Generates a unique identifier combining platform_id and username.
    fn identifier(&self) -> String {
        format!("{} -> {}", self.platform_id(), self.username())
    }

    /// Returns self.PublicKey.public_key_created_at
    fn public_key_created_at(&self) -> String;

    /// Generates a hint for the user when signing for encryption, based on the secret type.
    fn generate_hint(&self) -> Option<String> {
        let user_identifier = match self.secret_type() {
            SecretType::IdentityGroup => "Group Name",
            _ => "User name",
        };
    let (date, time) = Self::parse_key_create_time(&self.public_key_created_at());
        Some(
            format!(
                "You are signing this HINT for{}encryption in Onion Vault:\n\n
Platform ID: \"{}\"\n\n{}: \"{}\"\n\nCreated at\n{}\n{}",
                self.secret_type().in_box(),
                self.platform_id(),
                user_identifier,
                self.username(),
                date,
                time
            )
        )
    }

    /// Parses an identifier string into platform_id and username components.
    fn parse_identifier(identifier: &str) -> (String, String) {
        // Find the position of the separator
        let separator_position = identifier.find(" -> ");

        if let Some(pos) = separator_position {
            // Split the string into two parts
            let platform_id = &identifier[..pos];
            let username = &identifier[pos + 4..];
            (platform_id.to_string(), username.to_string())
        } else {
            // If the separator is not found, return empty strings
            ("".to_string(), "".to_string())
        }
    }

    fn parse_key_create_time(datetime_str: &str) -> (String, String) {
        let separator_index = datetime_str.find('T').expect("Expected 'T' in the datetime string");

        let date = &datetime_str[0..separator_index];

        let time = &datetime_str[separator_index + 1..separator_index + 9];

        (date.to_string(), time.to_string())
    }
}

/// Enumeration of different types of secrets that can be managed or encrypted.
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecretType {
    UniVault,
    Account,
    JSON,
    RawBytes,
    PasswordManager,
    IdentityGroup,
    Password,
    TOTPKey,
    BackupKey,
    Text,
}
impl fmt::Display for SecretType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecretType::UniVault =>write!(f, "UniVault"),
            SecretType::Account =>write!(f, "Account"),
            SecretType::JSON =>write!(f, "JSON"),
            SecretType::RawBytes =>write!(f, "Raw Bytes"),
            SecretType::PasswordManager =>write!(f, "Passwd Manager"),
            SecretType::IdentityGroup =>write!(f, "Identity Group"),
            SecretType::Password =>write!(f, "Password"),
            SecretType::TOTPKey =>write!(f, "TOTP Key"),
            SecretType::BackupKey =>write!(f, "Backup Key"),
            SecretType::Text =>write!(f, "Text"),
            // MyEnum::Variant3 => write!(f, "This is Variant3"),
        }
    }
}
impl SecretType {
    /// Converts the `SecretType` into a human-readable string.
    pub fn to_string_name(&self) -> &'static str {
        match self {
            SecretType::UniVault => "UniVault",
            SecretType::Account => "Account",
            SecretType::JSON => "JSON",
            SecretType::RawBytes => "Raw Bytes",
            SecretType::PasswordManager => "Passwd Manager",
            SecretType::IdentityGroup => "Identity Group",
            SecretType::Password => "Password",
            SecretType::TOTPKey => "TOTP Key",
            SecretType::BackupKey => "Backup Key",
            SecretType::Text => "Text",
        }
    }

    /// Formats the `SecretType` into a boxed string representation for visual emphasis.
    pub fn in_box(&self) -> String {
        let string_name = self.to_string_name();
        let width = string_name.chars().count() + 2;
        let horizontal_line = format!("+{}+", "-".repeat(width));

        format!("\n{}\n| {} |\n{}\n", horizontal_line, string_name, horizontal_line)
    }
}

/// Trait defining basic encryption and decryption operations for different encryption algorithms.
pub trait Encryptor {
    /// Encrypts the provided secret bytes.
    fn encrypt(&mut self, secret: &[u8]) -> anyhow::Result<Vec<u8>>;

    /// Decrypts the provided encrypted bytes back to the original secret.
    fn decrypt(&self, encrypted_bytes: &[u8]) -> anyhow::Result<Vec<u8>>;

    // fn public_key(&self)-> Option<String>;


    /// Returns a descriptive name or identifier for the encryption method used.
    fn name(&self) -> String;
}
