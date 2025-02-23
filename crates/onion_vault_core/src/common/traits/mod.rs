pub mod rage_trezor;

pub use rage_trezor::TrezorEncryptable;

pub use signature_to_keys::common::traits::JsonSerializable;
pub use crate::common::enums::DataState;
pub use crate::re_export::uni_vault::*;
pub use crate::re_export::std_anyhow::*;


// pub trait EncryptionTraversal<T, C, E>
// where C: Clone + JsonSerializable + EncryptionCapability<T, E> + HasUserIdentifier
// {
//      fn traverse_and_encrypt_if_necessary(&mut self) -> Result<()>;
// }



pub trait EncryptionCapability<T, E>
where E: Encryptor + Serialize
{
    fn update_data_state(&mut self, data: DataState<T, E>);
    fn set_data_state_to_encrypted(&mut self, uni_vault: UniVault<E>);
    fn set_data_state_to_decrypted(&mut self, decrypted_data: Self);
    fn backup_key(&self) -> PublicKey;
    fn public_key(&self) -> PublicKey;
    fn set_public_key(&mut self, public_key: &PublicKey);
    // fn get_secret_type(&self) -> SecretType;
    // fn get_identifier_name(&self) -> String;
    // fn get_secret_type(&self) -> SecretType;
    // // fn is_encrypted(&self) -> bool;
    /// Returns `None` if the structure is unencrypted
    fn get_encrypted_vault(&self) -> Option<UniVault<E>>;
}



// pub trait HasIdentifierName {
//     fn get_identifier_name(&self) -> String;
// }

// pub trait DataStateUpdateable<T, E>
// where E: Encryptor + Serialize
// {
//     fn update_data_state(&mut self, data: DataState<T, E>);
//     // fn data_state_encrypted(&mut self, data: DataState::);
// }

// // pub trait IsEncrypted {
// //     /// Returns `true` if the structure is encrypted, `false` otherwise.
// //     fn is_encrypted(&self) -> bool;
// // }
