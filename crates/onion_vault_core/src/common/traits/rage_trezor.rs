pub use signature_to_keys::common::traits::JsonSerializable;
pub use crate::common::traits::EncryptionCapability;
pub use crate::common::enums::DataState;
pub use crate::re_export::uni_vault::*;
pub use crate::re_export::std_anyhow::*;
use uni_vault::common::test_helpers::TREZOR_ENCRYPT_COMMENT;
use uni_vault::common::constants::DEFAULT_BIP32PATH;
// use crate::common::test_helpers::*;



pub trait TrezorEncryptable<T>: Clone + JsonSerializable + EncryptionCapability<T, RageTrezorEncryptor> + HasUserIdentifier
// where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{
    fn traverse_and_encrypt_if_necessary(&mut self) -> anyhow::Result<()>;



    fn generate_backup_key_by_trezor(&mut self) -> anyhow::Result<PublicKey> {
            let mut rage_trezor_encryptor: RageTrezorEncryptor = RageTrezorEncryptorBuilder::default()
                .platform_id(self.platform_id())
                .username(self.username())
                .should_create_new_recipient(true)
                .secret_type(SecretType::BackupKey)
                .bip32_path(bip32_path())
                .comment(TREZOR_ENCRYPT_COMMENT)
                .build()
                .unwrap();

        rage_trezor_encryptor.get_rage_recipients()?;
        // let backup_key = rage_trezor_encryptor.recipients
        //     .iter()
        //     .next()
        //     .unwrap()
        //     .clone();

        // Ok(backup_key)
        Ok(rage_trezor_encryptor.public_key)
    }



    fn toggle_encryption_by_trezor(&mut self) -> anyhow::Result<()> {
        if let Some(encrypted_uni_vault) = self.get_encrypted_vault() {
            // data_to_string will try decrypt data if it is still encrypted
            let self_as_json = encrypted_uni_vault.clone().data_to_string()?;

            let decrypted_self = Self::from_json(&self_as_json)?;

            self.set_data_state_to_decrypted(decrypted_self)

        } else {
            let self_to_json_as_bytes = self.to_json().as_bytes().to_vec();


            let public_key = self.public_key();
            let mut recipients_with_my_backup_key = HashSet::new();
            if let Some(ref recipient_str) = public_key.recipient_str {
                recipients_with_my_backup_key.insert(recipient_str.clone());
            }
            if let Some(backup_key) = self.backup_key().recipient_str {
                recipients_with_my_backup_key.insert(backup_key);
            }

            let rage_trezor_encryptor: RageTrezorEncryptor = RageTrezorEncryptorBuilder::default()
                .platform_id(self.platform_id())
                .username(self.username())
                .should_create_new_recipient(public_key.recipient_str.is_none())
                .public_key(public_key)
                .recipients(recipients_with_my_backup_key)
                .secret_type(self.secret_type())
                .bip32_path(bip32_path())
                .comment(TREZOR_ENCRYPT_COMMENT)
                .build()
                .unwrap();

            let mut uni_vault: UniVault<RageTrezorEncryptor> = UniVaultBuilder::default()
                .encryptor(rage_trezor_encryptor)
                .data_type(self.secret_type())
                .data(self_to_json_as_bytes)
                .build().unwrap();
            // print_json_with_line!(&uni_vault);

            uni_vault.encrypt()?;

            self.set_public_key(&uni_vault.encryptor.public_key.clone());
            // debug_with_line!(rage_trezor_encryptor.public_key.clone());
            // rage_trezor_encryptor.get_rage_recipients

            self.set_data_state_to_encrypted(uni_vault);
        }
        Ok(())
    }

}




fn bip32_path() -> String {
     match env::var("BIP32_PATH") {
        Ok(bip32_path) => {
            bip32_path
        },
        Err(_) => {
            DEFAULT_BIP32PATH.into()
        }
    }
}
