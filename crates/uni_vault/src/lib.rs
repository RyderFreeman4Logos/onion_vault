#[deny(unsafe_code)]
pub mod common;
pub mod re_export;
pub mod uni_vault;

pub mod utils;

// pub mod encrypted_vault;
pub mod encryptor;

pub use uni_vault::{
    UniVault,
    UniVaultBuilder,
};
pub use encryptor::asymmetric::rage_trezor::{
    RageTrezorEncryptor,
    RageTrezorEncryptorBuilder,
};
pub use encryptor::{
    Encryptor,
    HasUserIdentifier,
    SecretType,
    asymmetric::{
        PublicKey,
        PublicKeyBuilder,
    },
};
// pub use common::test_helpers::TREZOR_ENCRYPT_COMMENT;
pub mod test;
