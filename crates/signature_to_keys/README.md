# signature_to_keys

## Overview

`signature_to_keys` is a Rust crate that implements a novel deterministic subkey derivation scheme for various cryptographic applications. Instead of generating key pairs directly, this crate leverages a master private key to sign a human-readable string. The resulting signature is then hashed with SHA-256 to produce a 256-bit seed, which is used to generate key pairs for systems such as SSH (ed25519), Ethereum wallets, and other cryptographic structures (e.g., keys using curve25519-dalek).

## Features

- **Deterministic Subkey Derivation:**  
  Utilizes deterministic signing (as defined in RFC 6979) combined with a timestamp to ensure that each derived subkey is unique and reproducible.
  
- **Flexible Key Generation:**  
  Supports the generation of multiple types of key pairs including SSH ed25519 keys (, Ethereum wallet keys, and keys for advanced cryptographic libraries like curve25519-dalek will be implemented soon).
  
- **Hierarchical Derivation:**  
  By integrating signing timestamps(recommended) and purpose descriptions into the signature, the scheme allows for secure and hierarchical key derivation.
  
- **Enhanced Security:**  
  The method incorporates safeguards such as backing up the initial 256-bit seed using a BIP39 mnemonic and ensuring collision resistance via SHA-256.

## Methodology

1. **Seed Generation and Backup:**  
   A cryptographically secure 256-bit random seed is generated and backed up with a BIP39 mnemonic phrase to ensure recoverability.
   
2. **Ethereum Wallet Creation:**  
   Instead of directly generating a traditional key pair, an Ethereum wallet is derived from the seed, leveraging existing secure infrastructures.
   
3. **Deterministic Signing with Timestamp:**  
   A message that describes the intended use of the subkey pair (including the signing timestamp) is signed with the master private key. Due to deterministic signing, the signature is uniquely tied to that moment.
   
4. **Child Seed Derivation:**  
   The signature is then hashed using SHA-256 to produce a new 256-bit seed. This child seed is used to generate a new subkey pair or even a new Ethereum wallet, enabling a flexible hierarchical derivation process.

## Security Considerations

Incorporating the signing timestamp into the derivation process ensures that each signature is unique, preventing replay attacks and reducing the risk of signature collisions. The deterministic nature of the scheme ensures that the same inputs always yield the same subkey, which is crucial for hierarchical deterministic (HD) systems. However, a comprehensive security review is recommended to validate that the master key remains secure and that the mapping between derivation paths and keys is robust.

## Usage

Add `signature_to_keys` as a dependency in your `Cargo.toml`

```shell
cargo add signature_to_keys

