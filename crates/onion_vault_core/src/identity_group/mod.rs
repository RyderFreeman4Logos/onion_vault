//! This module defines the `IdentityGroup`, which represents a group of accounts organized by some criterion (e.g., work, social).

// use crate::accounts::DataState;

pub mod account;
pub use account::{
    Account,
    // AccountIdentifier,
    // SecretType,
};

// use crate::traits::{
//     JsonSerializable,
//     TrezorEncryptable,
// };

use crate::common::traits::*;

// use uni_vault::UniVault;

use crate::common::enums::*;
use crate::common::test_helpers::debug_with_line;
use crate::re_export::uni_vault::*;
use crate::re_export::std_anyhow::*;
// use uni_vault::{
//     UniVault,
//     PassPhrase,
//     encrypted_vault::AuthInfo,
// };


// use anyhow::anyhow::Result;

// use serde::{Deserialize, Serialize};

// use std::collections::HashMap;

/// Represents a group of accounts, where each account is identified by a unique string identifier.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct IdentityGroup<E>
where E: Encryptor + Serialize
{
    #[builder(setter(into))]
    #[builder(default = "true")]
    pub should_be_encrypt: bool,

    #[builder(setter(into))]
    #[builder(default = "PublicKeyBuilder::default().build().unwrap()")]
    pub public_key: PublicKey,

    #[builder(setter(into))]
    #[builder(default = "PublicKeyBuilder::default().build().unwrap()")]
    pub backup_key: PublicKey,

    #[builder(setter(into))]
    pub group_name: String,

    /// The identifier for the platform, such as "gmail.com".
    #[builder(setter(into))]
    #[builder(default = "\"onion_vault\".to_string()")]
    pub platform_id: String,

    /// Type of secret being encrypted (e.g., Password, Text, etc.).
    #[builder(setter(into))]
    #[builder(default = "SecretType::IdentityGroup")]
    pub secret_type: SecretType,

    #[builder(setter(into))]
    #[builder(default = "DataState::Plain(Vec::new())")]
    pub accounts: DataState<Vec<Account<E>>, E>,
}

impl TrezorEncryptable<Vec<Account<RageTrezorEncryptor>>> for IdentityGroup<RageTrezorEncryptor> {
     fn traverse_and_encrypt_if_necessary(&mut self) -> anyhow::Result<()> {
        let accounts = match &mut self.accounts {
            DataState::Plain(accounts) => {
                accounts
            },
            DataState::Encrypted(_) => {
                return Ok(())
            }
        };

        for account in accounts.iter_mut(){
            // for (_, credential) in account.credentials.iter_mut() {
            for credential in account.credentials.iter_mut() {
                if credential.should_be_encrypt {
                    credential.traverse_and_encrypt_if_necessary()?;
                }
            }
        }

        if self.should_be_encrypt {
            debug_with_line!("Touch Your Trezor");
            self.toggle_encryption_by_trezor()?;
        }
        Ok(())
     }
}

impl<E> IdentityGroup<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{
    /// Adds an account to the group. If the identifier already exists, the account is appended to the existing vector.
    ///
    /// # Arguments
    ///
    /// * `identifier` - A unique identifier for the account, typically a combination of platform ID and username.
    /// * `account` - The `Account` to be added to the group.
    pub fn add_account(&mut self, account: Account<E>) -> anyhow::Result<()>{
        if let DataState::Plain(ref mut accounts) = &mut self.accounts {
            accounts.push(account);
            Ok(())
        } else {
            Err(Error::msg("need to be decrypt"))?
        }
    }




    pub fn accounts_mut(&mut self) -> anyhow::Result<&mut Vec<Account<E>>> {
        match &mut self.accounts {
            DataState::Plain(accounts) => {
                Ok(accounts)
            },
            DataState::Encrypted(_) => {
                Err(Error::msg("Accounts are encrypted, cannot access"))
            }
        }
    }



    pub fn get_account_mut(&mut self, identifier: &str) -> anyhow::Result<&mut Account<E>> {
        self.accounts_mut()?.iter_mut().find(|account| {
            account.identifier() == identifier
        }).ok_or_else(|| Error::msg(format!("Account \"{}\" not found", identifier)))
    }
}



impl<E> HasUserIdentifier for IdentityGroup<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{
    /// Returns the username of the user associated with this encryptor instance.
    fn username(&self) -> String {
        self.group_name.clone()
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
        self.public_key.public_key_created_at.clone()
    }
}

impl<E> JsonSerializable for IdentityGroup<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{}

impl<E> EncryptionCapability<Vec<Account<E>>, E> for IdentityGroup<E>
where E: Clone + Encryptor + Serialize + for<'de> Deserialize<'de>
{
    fn backup_key(&self) -> PublicKey {
        self.backup_key.clone()
    }



    fn public_key(&self)-> PublicKey {
        self.public_key.clone()
    }

    fn set_public_key(&mut self, public_key: &PublicKey){
        self.public_key = public_key.to_owned()
    }



    fn update_data_state(&mut self, data: DataState<Vec<Account<E>>, E>) {
        self.accounts = data
    }


    fn set_data_state_to_encrypted(&mut self, uni_vault: UniVault<E>) {
        self.accounts = DataState::Encrypted(uni_vault)
    }

    fn set_data_state_to_decrypted(&mut self, decrypted_data: Self) {
        self.accounts = decrypted_data.accounts
    }


    fn get_encrypted_vault(&self) -> Option<UniVault<E>> {
        match &self.accounts {
            DataState::Plain(_) => None,
            DataState::Encrypted(res) => Some(res.clone()),
        }
    }
}
