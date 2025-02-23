#[cfg(test)]
mod tests {
    use super::super::super::*;
    
    
    use hex;

    use crate::common::test_helpers::*;

    #[test]
    // #[ignore]
    fn test_trezor_ssh_keypair_generator() {
        // let mut trezor_generator = TrezorGenerator::sign_with_bip32_path(TEST_DATA[3], TEST_DATA[0])
        //     .unwrap();
        // trezor_generator without using trezor
        let mut trezor_generator:TrezorGenerator = {
            let mut trezor_generator= TrezorGeneratorBuilder::default()
                .build()
                .unwrap();

            let signature = TEST_DATA[1].as_bytes().to_vec();
            trezor_generator.signature = Some(signature.clone());

            let signature_str = std::str::from_utf8(&signature).unwrap();
            assert_eq!(TEST_DATA[1], signature_str);
            trezor_generator
        };

        let signature = TEST_DATA[1].as_bytes().to_vec();
        trezor_generator.signature = Some(signature.clone());

        let signature_str = std::str::from_utf8(&signature).unwrap();
        assert_eq!(TEST_DATA[1], signature_str);

        let ssh_key_pair = trezor_generator.generate_ssh_key_pair_from_signature()
            .unwrap();

        debug_with_line!(hex::encode(ssh_key_pair.to_bytes()));
    }



    #[test]
    fn test_trezor_rage_identity_generator() {
        // let mut trezor_generator = TrezorGenerator::sign_with_bip32_path(TEST_DATA[3], TEST_DATA[0])
        //     .unwrap();
        // trezor_generator without using trezor
        let trezor_generator:TrezorGenerator = {
            let mut trezor_generator= TrezorGeneratorBuilder::default()
                .build()
                .unwrap();

            let signature = TEST_DATA[1].as_bytes().to_vec();
            trezor_generator.signature = Some(signature.clone());

            let signature_str = std::str::from_utf8(&signature).unwrap();
            assert_eq!(TEST_DATA[1], signature_str);
            trezor_generator
        };

        let ssh_key_pair = trezor_generator.generate_ssh_key_pair_from_signature()
            .unwrap();
        debug_with_line!(hex::encode(ssh_key_pair.to_bytes()));

        let identity = trezor_generator.generate_rage_identity()
            .unwrap();

        // debug_with_line!();

        let recipient = age::ssh::Recipient::try_from(identity.clone()).unwrap();
        debug_with_line!(recipient.to_string());

        let decrypted = age::decrypt(&identity, TEST_DATA[5].as_bytes())
            .unwrap();

        let decoded = String::from_utf8(decrypted).unwrap();
        debug_with_line!(decoded);
        assert_eq!(TEST_DATA[4], decoded);
    }
}

