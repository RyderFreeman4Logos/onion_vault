#[cfg(test)]
mod tests {
    use crate::{
        onion_vault::{
            OnionVault,
            OnionVaultBuilder,
        },
        identity_group::{
            IdentityGroupBuilder,
            account::{
                AccountBuilder,
                account_credential::AccountCredentialBuilder,
            },
        },
        common::{
            traits::{
                JsonSerializable,
                TrezorEncryptable,
            },
            enums::DataState,
            test_helpers::{
                debug_with_line,
                print_json_with_line,
            },
        },

    };

    use crate::common::traits::*;
    
    



    #[test]
    // #[ignore]
    fn test_onion_vault_core() {
        /// Creat an onion_vault
        let mut onion_vault4ryder: OnionVault<RageTrezorEncryptor> = OnionVaultBuilder::default()
            .should_be_encrypt(true)
            .ownername("RyderFreeman")
            .build().unwrap();


        debug_with_line!("Touch Your Trezor");
        let backup_key = onion_vault4ryder.generate_backup_key_by_trezor().unwrap();
        onion_vault4ryder.backup_key = backup_key.clone();
        print_json_with_line!(&onion_vault4ryder);



        let vault_map4ryder = onion_vault4ryder.get_vault_map_mut().unwrap();
        print_json_with_line!(&vault_map4ryder);



        let work_identity = IdentityGroupBuilder::default()
            .should_be_encrypt(false)
            .backup_key(backup_key.clone())
            .group_name("Uber Driver")
            .build().unwrap();

        vault_map4ryder.add_identity_group(work_identity);

        print_json_with_line!(&vault_map4ryder);
        drop(vault_map4ryder);
        print_json_with_line!(onion_vault4ryder);


        let secret_identity_group_name = "My Porn accounts";
        {
            let vault_map4ryder = onion_vault4ryder.get_vault_map_mut()
                .unwrap();
            /// "In fact, the accounts within this identity group are used for
            /// activities of investigative journalists and political dissidents."
            let secret_identity = IdentityGroupBuilder::default()
                // different from work_identity, secret_identity should be encrypted
                .should_be_encrypt(true)
                .backup_key(backup_key.clone())
                .group_name(secret_identity_group_name)
                .build().unwrap();

            vault_map4ryder.add_identity_group(secret_identity);

            print_json_with_line!(&vault_map4ryder);
            print_json_with_line!(&onion_vault4ryder);
        }



        let secret_account_username = "program.think_young_version@gmail.com";
        let secret_account_platform_id = "app.element.io";
        let auto_generated_strong_password = String::from(
            "YKm@3KAe9mKjpxesRFzkKPd7WMTuikvL"
        );
        let mut secret_account_identifier = String::new();
        {
            let vault_map4ryder = onion_vault4ryder.get_vault_map_mut().unwrap();
            let secret_identity = vault_map4ryder.get_identity_group_mut(secret_identity_group_name)
                .unwrap();
            let mut secret_account = AccountBuilder::default()
                .username(&*secret_account_username)
                .platform_id(&*secret_account_platform_id)
                .build().unwrap();

            secret_account_identifier = secret_account.identifier();

            let credential4secret_account = AccountCredentialBuilder::default()
                .should_be_encrypt(true)
                .backup_key(backup_key.clone())
                .username(&secret_account.username)
                .platform_id(&secret_account.platform_id)
                .secret(DataState::Plain(auto_generated_strong_password.clone()))
                .secret_type(SecretType::Password)
                .build().unwrap();

            secret_account.add_credential(credential4secret_account);
            secret_identity.add_account(secret_account);
            print_json_with_line!(&onion_vault4ryder);
        }

        onion_vault4ryder.traverse_and_encrypt_if_necessary()
            .unwrap();
        print_json_with_line!(&onion_vault4ryder);

        {
            debug_with_line!("Touch Your Trezor");
            onion_vault4ryder.toggle_encryption_by_trezor()
                .unwrap();
            print_json_with_line!(&onion_vault4ryder);
            let vault_map4ryder = onion_vault4ryder.get_vault_map_mut().unwrap();
            let secret_identity = vault_map4ryder.get_identity_group_mut(secret_identity_group_name)
                .unwrap();
            debug_with_line!("Touch Your Trezor");
            secret_identity.toggle_encryption_by_trezor()
                .unwrap();
            let secret_account = secret_identity.get_account_mut(&secret_account_identifier)
                .unwrap();


            let credential4secret_account = secret_account.get_credential_mut(SecretType::Password)
                .unwrap();

            debug_with_line!("Touch Your Trezor");
            credential4secret_account.toggle_encryption_by_trezor().unwrap();
            let DataState::Plain(ref decrypted_password) = credential4secret_account.secret else { panic!() };
            assert_eq!(&auto_generated_strong_password, decrypted_password);

            print_json_with_line!(&onion_vault4ryder);
        }




        onion_vault4ryder.traverse_and_encrypt_if_necessary()
            .unwrap();
        print_json_with_line!(&onion_vault4ryder);



        // print_json_with_line!(&secret_identity);
        assert_eq!(1, 1);
    }

    #[test]
    // #[ignore]
    fn test_onion_vault_reencrypt() {
        let json_str = r#"{"should_be_encrypt":true,"public_key":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMZ1rnXQJCh0t2tclUgRvLdBcS3wK5IKS7V6XDq+YOvB","backup_key":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAe3nm3/EVp9m/GO6qt0erUsLbpbNh50Wlrl3iHUCdEy","ownername":"RyderFreeman","platform_id":"onion_vault","secret_type":"PasswordManager","version":"0.0.1","vault_map":{"Plain":{"identity_groups":[{"should_be_encrypt":false,"backup_key":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAe3nm3/EVp9m/GO6qt0erUsLbpbNh50Wlrl3iHUCdEy","public_key":null,"group_name":"Uber Driver","platform_id":"onion_vault","secret_type":"IdentityGroup","accounts":{"Plain":[]}},{"should_be_encrypt":true,"backup_key":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAe3nm3/EVp9m/GO6qt0erUsLbpbNh50Wlrl3iHUCdEy","public_key":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMzqJWpx1oDEkOpgSm3wpjDvHd0If25lSf4Pc/2DcsWc","group_name":"My Porn accounts","platform_id":"onion_vault","secret_type":"IdentityGroup","accounts":{"Plain":[{"credentials":[{"should_be_encrypt":true,"backup_key":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAe3nm3/EVp9m/GO6qt0erUsLbpbNh50Wlrl3iHUCdEy","public_key":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOhYWJUQqd23yHqtV1rTPsN/XlWBn+oe/oXjQnD1LAG+","secret":{"Plain":"YKm@3KAe9mKjpxesRFzkKPd7WMTuikvL"},"username":"program.think_young_version@gmail.com","platform_id":"app.element.io","secret_type":"Password"}],"username":"program.think_young_version@gmail.com","platform_id":"app.element.io","secret_type":"Account"}]}}],"version":"0.0.1"}}}"#;
        let mut onion_vault4ryder: OnionVault<RageTrezorEncryptor> = OnionVault::from_json(json_str)
            .unwrap();
        print_json_with_line!(&onion_vault4ryder);
        onion_vault4ryder.traverse_and_encrypt_if_necessary()
            .unwrap();
        debug_with_line!("$$$$$$$$$$$$$$$$$");
        print_json_with_line!(&onion_vault4ryder);


        assert_eq!(1, 1);
    }
}
