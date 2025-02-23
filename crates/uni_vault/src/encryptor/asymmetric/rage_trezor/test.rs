#[cfg(test)]
mod tests {
    use crate::encryptor::{
        asymmetric::rage_trezor::{
            RageTrezorEncryptorBuilder,
            RageTrezorEncryptor,
        },
        Encryptor,
        HasUserIdentifier,
        SecretType,
    };
    use crate::common::traits::JsonSerializable;
    use crate::common::test_helpers::{
        TREZOR_ENCRYPT_COMMENT,
        print_json_with_line
    };

    #[test]
    #[ignore]
    fn test_rage_trezor_encryptor_generator_builder() {
        let passowrd_for_test = 12345678.to_string();
        let password_as_bytes = passowrd_for_test.as_bytes();

        let mut rage_trezor_encryptor: RageTrezorEncryptor = RageTrezorEncryptorBuilder::default()
            .platform_id("uni.vault.onion")
            .username("test_account")
            .secret_type(SecretType::Password)
            .bip32_path("m/44h/60h/11h/0/12")
            .comment(TREZOR_ENCRYPT_COMMENT)
            .build()
            .unwrap();
        print_json_with_line!(&rage_trezor_encryptor);

        let encrypted_bytes = rage_trezor_encryptor.encrypt(password_as_bytes)
            .unwrap();
        print_json_with_line!(&rage_trezor_encryptor);

        let decrypted_bytes = rage_trezor_encryptor.decrypt(&encrypted_bytes)
            .unwrap();

        assert_eq!(password_as_bytes, decrypted_bytes);
    }
}
