#[cfg(test)]
mod tests {
    use crate::{
        uni_vault::{
            UniVault,
            UniVaultBuilder,
        },
        encryptor::{
            asymmetric::rage_trezor::{
                RageTrezorEncryptorBuilder,
                RageTrezorEncryptor,
            },
            Encryptor,
            HasUserIdentifier,
            SecretType,
        },
        re_export::std_anyhow::*,
        common::traits::JsonSerializable,
        common::test_helpers::{
            TREZOR_ENCRYPT_COMMENT,
            print_json_with_line
        },
    };

    #[test]
    #[ignore]
    fn test_uni_vault_by_rage_trezor()
    {
        let passowrd_for_test = 12345678.to_string();
        let password_as_bytes = passowrd_for_test.as_bytes();
        let mut recipients_with_my_backup_key = HashSet::new();
        recipients_with_my_backup_key.insert("ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILD0y5DXSqOmx/Tf0pKP23+JrCVSUrezlPDf/vcFFt3s".to_string());

        let rage_trezor_encryptor: RageTrezorEncryptor = RageTrezorEncryptorBuilder::default()
            .platform_id("uni.vault.onion")
            .username("test_account")
            .should_create_new_recipient(true)
            .recipients(recipients_with_my_backup_key)
            .secret_type(SecretType::Password)
            .bip32_path("m/44h/60h/11h/0/12")
            .comment(TREZOR_ENCRYPT_COMMENT)
            .build()
            .unwrap();


        let mut uni_vault: UniVault<RageTrezorEncryptor> = UniVaultBuilder::default()
            .encryptor(rage_trezor_encryptor)
            .data_type(SecretType::Password)
            .data(password_as_bytes)
            .build().unwrap();
        print_json_with_line!(&uni_vault);

        uni_vault.encrypt().unwrap();
        print_json_with_line!(&uni_vault);

        uni_vault.decrypt().unwrap();
        print_json_with_line!(&uni_vault);

        assert_eq!(password_as_bytes, uni_vault.data);
    }
}
