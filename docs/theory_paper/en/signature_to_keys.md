# A New Deterministic Subkey Derivation Scheme for Ethereum Wallets *(Or any other crypto wallet)*

## Introduction  
In modern Ethereum libraries and hardware wallets, deterministic signatures have become the standard. Leveraging this advancement, I propose a new subkey derivation scheme where a cryptographically secure 256‑bit random number is generated and backed up via a BIP39 mnemonic. An Ethereum wallet is then created—not by directly generating a traditional key pair, but by using the wallet’s existing cryptographic infrastructure (including hardware keys) to sign **a message that describes the intended use of a subkey pair**.

Furthermore, by incorporating the signing timestamp into the message, the process ensures that each signature is uniquely tied to a particular moment in time. This deterministic signature, when processed through SHA‑256, yields a new 256‑bit “child” random number. This number can then serve as the seed for generating a new subkey pair or even another Ethereum wallet, allowing for a flexible and secure hierarchical derivation process.

## Methodology  
1. **Seed Generation and Backup:**  
   - Begin by generating a cryptographically secure 256‑bit random number.  
   - Back up this seed using a BIP39 mnemonic phrase to ensure recoverability.  

2. **Ethereum Wallet Creation:**  
   - Instead of generating a traditional key pair directly, derive an Ethereum wallet from the seed. This approach leverages the well‑tested cryptographic infrastructure of the cryptocurrency domain, including hardware-backed key storage.  

3. **Deterministic Signing with Timestamp:**  
   - Sign a message that describes the subkey’s purpose. Crucially, include the signing timestamp in the message.  
   - Because modern Ethereum libraries employ deterministic signing ([RFC 6979](https://datatracker.ietf.org/doc/html/rfc6979)), the same message (with its timestamp) will always yield the same signature.  

4. **Derivation of the Child Seed:**  
   - Compute the SHA‑256 hash of the obtained signature to produce a new 256‑bit “child” random number.  
   - Use this child seed to generate a new subkey pair or another Ethereum wallet, thus extending the key hierarchy.  

## Security Considerations  
By incorporating the signing time into the message, the scheme guarantees that each signature is unique in its temporal context, mitigating risks such as replay attacks or unintended signature collisions. Moreover, the deterministic nature of the signature ensures reproducibility—essential for hierarchical deterministic (HD) systems. However, it is imperative to ensure that the mapping from human‑readable derivation paths (if used) to actual derivation data is robust and collision‑resistant. A comprehensive security review is essential to confirm that the child keys remain isolated and that the master key cannot be compromised through any reversible operations.

## Conclusion  
This subkey derivation scheme presents an innovative way to leverage Ethereum’s mature cryptographic tools and deterministic signing capabilities. By including the signing time within the message, the method adds an extra layer of uniqueness and security, making it a promising alternative for hierarchical key management. With rigorous mapping of derivation paths and thorough security analyses, this approach could offer both enhanced usability and robust protection in the context of Ethereum wallet development.
