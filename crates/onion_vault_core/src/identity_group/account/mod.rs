//! This module defines the `Account` structure, which represents an account on a specific platform.

pub mod account_credential;
pub use account_credential::AccountCredential;


// use crate::vault_map::{
    // DataState,
    // JsonSerializable,
// };

use crate::re_export::uni_vault::*;
use crate::re_export::std_anyhow::*;

// use serde::{Deserialize, Serialize};
// use std::collections::BTreeMap;


/// Represents an account on a platform, including its credentials.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Account<E>
where E: Encryptor + Serialize
{
    /// A list of credentials associated with this account.
    #[builder(setter(into))]
    // #[builder(default = "BTreeMap::new()")]
    #[builder(default = "Vec::new()")]
    pub credentials: Vec<AccountCredential<E>>,
    // pub credentials: BTreeMap<SecretType, AccountCredential<E>>,

    #[builder(setter(into))]
    pub username: String,

    /// The identifier for the platform, such as "gmail.com".
    #[builder(setter(into))]
    pub platform_id: String,

    /// Type of secret being encrypted (e.g., Password, Text, etc.).
    #[builder(setter(into))]
    #[builder(default = "SecretType::Account")]
    pub secret_type: SecretType,
}

impl<E> Account<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{
    pub fn add_credential(&mut self, account_credential: AccountCredential<E>) {
        self.credentials.push(account_credential);
        // self.credentials.insert(account_credential.secret_type.clone(), account_credential);
    }




    pub fn get_credential_mut(&mut self, secret_type: SecretType) -> anyhow::Result<&mut AccountCredential<E>> {
        self.credentials.iter_mut().filter(|credential| credential.secret_type == secret_type)
        .next()
        .ok_or_else(|| Error::msg("Secret Type not found"))
    }
}




impl<E> HasUserIdentifier for Account<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{
    /// Returns the username of the user associated with this encryptor instance.
    fn username(&self) -> String {
        self.username.clone()
    }

    /// Returns the platform ID associated with this encryptor.
    fn platform_id(&self) -> String {
        self.platform_id.clone()
    }

    /// Returns the type of secret this encryptor is handling.
    fn secret_type(&self) -> SecretType {
        self.secret_type.clone()
    }

    fn public_key_created_at(&self) -> String {
        String::new()
    }
}
