use crate::vault_map::VaultMap;
use crate::common::test_helpers::debug_with_line;

use crate::common::traits::*;
use crate::common::enums::DataState;



#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct OnionVault<E>
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
    #[builder(default = "\"filename of PasswordManager Database\".to_string()")]
    pub ownername: String,

    /// The identifier for the platform, such as "gmail.com".
    #[builder(setter(into))]
    #[builder(default = "\"onion_vault\".to_string()")]
    pub platform_id: String,

    /// Type of secret being encrypted (e.g., Password, Text, etc.).
    #[builder(setter(into))]
    #[builder(default = "SecretType::PasswordManager")]
    pub secret_type: SecretType,

    #[builder(setter(into))]
    #[builder(default = "\"0.0.1\".to_string()")]
    pub version: String,

    #[builder(setter(into))]
    #[builder(default = "DataState::Plain(VaultMap::new())")]
    pub vault_map: DataState<VaultMap<E>, E>,
}



impl TrezorEncryptable<VaultMap<RageTrezorEncryptor>> for OnionVault<RageTrezorEncryptor> {
    fn traverse_and_encrypt_if_necessary(&mut self) -> anyhow::Result<()> {
        let unencrypted_vault_map = match &mut self.vault_map {
            DataState::Plain(unencrypted_vault_map) => {
                unencrypted_vault_map
            },
            DataState::Encrypted(_) => {
                return Ok(())
            }
        };

        for identity_group in unencrypted_vault_map.identity_groups.iter_mut(){
            identity_group.traverse_and_encrypt_if_necessary()?
        }

        if self.should_be_encrypt {
            debug_with_line!("Touch Your Trezor");
            self.toggle_encryption_by_trezor()?;
        }
        Ok(())
    }
}



impl<E> HasUserIdentifier for OnionVault<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{
    /// Returns the username of the user associated with this encryptor instance.
    fn username(&self) -> String {
        self.ownername.clone()
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

impl<E> JsonSerializable for OnionVault<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{}

impl<E> EncryptionCapability<VaultMap<E>, E> for OnionVault<E>
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


    fn update_data_state(&mut self, data: DataState<VaultMap<E>, E>) {
        self.vault_map = data
    }


    fn set_data_state_to_encrypted(&mut self, uni_vault: UniVault<E>) {
        self.vault_map = DataState::Encrypted(uni_vault)
    }

    fn set_data_state_to_decrypted(&mut self, decrypted_data: Self) {
        self.vault_map = decrypted_data.vault_map
    }


    fn get_encrypted_vault(&self) -> Option<UniVault<E>> {
        match &self.vault_map {
            DataState::Plain(_) => None,
            DataState::Encrypted(res) => Some(res.clone()),
        }
    }
}

impl<E> OnionVault<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{
    pub fn get_vault_map_mut(&mut self) -> anyhow::Result<&mut VaultMap<E>> {
        match &mut self.vault_map {
            DataState::Plain(vault_map) => Ok(vault_map),
            DataState::Encrypted(_) => {
                Err(Error::msg("OnionVault are encrypted, cannot access"))
            }
        }
    }
}
