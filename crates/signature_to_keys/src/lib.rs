// Module for handling signature generation
pub mod signature_generators;

// Internal modules
pub mod re_export;
pub mod common;

pub use signature_generators::eth::trezor::TrezorGenerator;
// mod extensions;

// Import necessary dependencies
use re_export::std_anyhow::*;
use sha2::{Sha256, Digest};
use ssh_key::{
    PrivateKey,
    LineEnding,
    private::Ed25519Keypair,
};
use age::ssh::Identity;
use zeroize::Zeroize;
use std::io::Cursor;

// Trait for generating keys from signatures
pub trait SignatureToKeyGenerator {
    /// Generates a new instance of the implementing type with a signature based on a BIP32 path.
    /// 
    /// # Arguments
    /// * `path` - A string representing the BIP32 derivation path.
    /// * `hint_message` - An additional message to enhance uniqueness or entropy.
    /// 
    /// # Returns
    /// A Result containing the new instance or an error if generation fails.
    fn sign_with_bip32_path(path: &str, hint_message: &str) -> anyhow::Result<Self> where Self: Sized;

    /// Get the signature field of Self (of type Vec<u8>). If this field has not been set through
    /// signing a message (i.e., its value is None), throw an error.
    /// 
    /// Obtain the signature of a specific message with a specific private key. There are no
    /// specific requirements for format or algorithm; it only needs to ensure that an attacker
    /// cannot obtain it based on length and randomness.
    /// Theoretically, as long as it can return a password with very high random entropy, that would suffice.
    ///
    /// # Returns
    /// A Result containing the signature as a vector of bytes or an error if signing fails.
    fn signature(&self) -> anyhow::Result<Vec<u8>>;

    fn message_info_json(&self) -> String;

    /// Generates an SSH key pair from the signature.
    /// 
    /// This method hashes the signature to produce a secure seed for key generation.
    ///
    /// # Returns
    /// A Result with an Ed25519 key pair or an error if generation fails.
    fn generate_ssh_key_pair_from_signature(&self) -> anyhow::Result<Ed25519Keypair> {
        // Get the signature from the implementing type
        let mut signature = self.signature()?;

        // Hash the signature to create a seed
        let mut hasher = Sha256::new();
        hasher.update(&signature);
        signature.zeroize(); // Securely clear the signature from memory

        // Convert hash to a fixed size seed for keypair generation
        let mut seed: [u8; 32] = hasher.finalize().into();

        // Generate the SSH key pair from the seed
        let ssh_key_pair = Ed25519Keypair::from_seed(&seed);

        // Clear seed from memory for security
        seed.zeroize();

        Ok(ssh_key_pair)
    }

    /// Generates an identity for use with the 'rage' encryption tool from the SSH key pair.
    /// 
    /// # Returns
    /// A Result containing the Identity or an error if generation fails.
    fn generate_rage_identity(&self) -> anyhow::Result<Identity> {
        // Generate SSH key pair from signature
        let ssh_key_pair = self.generate_ssh_key_pair_from_signature()?;
        let private_key = PrivateKey::from(ssh_key_pair);

        // Convert private key to OpenSSH format
        let mut buffer = private_key.to_openssh(LineEnding::LF).unwrap();
        // let ssh_private_key_name = "ssh private key name".to_string();
        let ssh_private_key_name = self.message_info_json();


        // Create an Identity from the buffer
        let identity = Identity::from_buffer(
            Cursor::new(&mut *buffer),
            Some(ssh_private_key_name)
        ).expect("Failed to create Identity");

        // Clear buffer from memory for security
        buffer.zeroize();

        Ok(identity)
    }
}
