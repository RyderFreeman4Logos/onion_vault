//! This module defines `AccountCredential` for storing encrypted information like passwords or backup keys.


use crate::common::traits::*;
use crate::common::enums::*;
use crate::common::test_helpers::debug_with_line;
use crate::re_export::uni_vault::*;

use crate::re_export::std_anyhow::*;


/// Represents a single credential for an account, storing encrypted information and its type.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct AccountCredential<E>
where E: Encryptor + Serialize
{
    // /// Encrypted information as a string. This could be a password, backup key, or TOTP secret.
    // pub encrypted_info: Option<Cipher>,

    #[builder(setter(into))]
    #[builder(default = "true")]
    pub should_be_encrypt: bool,

    #[builder(setter(into))]
    #[builder(default = "PublicKeyBuilder::default().build().unwrap()")]
    pub public_key: PublicKey,

    #[builder(setter(into))]
    #[builder(default = "PublicKeyBuilder::default().build().unwrap()")]
    pub backup_key: PublicKey,

    /// unencrypted secret
    pub secret: DataState<String, E>,

    #[builder(setter(into))]
    pub username: String,

    /// The identifier for the platform, such as "gmail.com".
    #[builder(setter(into))]
    pub platform_id: String,

    /// Type of secret being encrypted (e.g., Password, Text, etc.).
    #[builder(setter(into))]
    #[builder(default = "SecretType::Password")]
    pub secret_type: SecretType,
}



impl TrezorEncryptable<String> for AccountCredential<RageTrezorEncryptor>{
    fn traverse_and_encrypt_if_necessary(&mut self) -> anyhow::Result<()> {
        match &self.secret {
            DataState::Plain(_) => {
                if self.should_be_encrypt {
                    debug_with_line!("Touch Your Trezor");
                    self.toggle_encryption_by_trezor()?;
                }
                Ok(())
            },
            DataState::Encrypted(_) => {
                return Ok(())
            }
        }
    }
}




impl<E> HasUserIdentifier for AccountCredential<E>
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
        self.public_key.public_key_created_at.clone()
    }
}

impl<E> JsonSerializable for AccountCredential<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{}

impl<E> EncryptionCapability<String, E> for AccountCredential<E>
// where E: Encryptor + Serialize
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



    fn update_data_state(&mut self, data: DataState<String, E>) {
        self.secret = data
    }


    fn set_data_state_to_encrypted(&mut self, uni_vault: UniVault<E>) {
        self.secret = DataState::Encrypted(uni_vault)
    }

    fn set_data_state_to_decrypted(&mut self, decrypted_data: Self) {
        self.secret = decrypted_data.secret
    }


    fn get_encrypted_vault(&self) -> Option<UniVault<E>> {
        match &self.secret {
            DataState::Plain(_) => None,
            DataState::Encrypted(res) => Some(res.clone()),
        }
    }
}
