#[cfg(test)]
mod tests {
    use super::super::super::*;
    
    

    use crate::common::test_helpers::*;

    #[test]
    fn test_trezor_generator_builder() {
        let trezor_generator = TrezorGeneratorBuilder::default()
            .build()
            .unwrap();
        debug_with_line!(&trezor_generator);
        assert_eq!(1, 1);
    }



    #[test]
    #[ignore]
    fn test_trezor_generator() {
        // let signature = sign_msg_with_trezor(&TEST_DATA[0]).unwrap();
        let signature = TrezorGeneratorBuilder::default()
            .bip32_path("m/44h/60h/11h/0/12")
            .build()
            .unwrap()
            .sign_msg_with_trezor(&TEST_DATA[0])
            .unwrap();

        assert_eq!(TEST_DATA[1], signature);
    }
}

