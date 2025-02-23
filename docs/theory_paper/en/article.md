# In-depth Reflections on Password Managers and Hardware Keys

After using various password managers, I have developed some personal reflections on the field of password management. First, why do we even use password managers? Without them, most people tend to remember only one or a few passwords. Because these passwords must be memorized, they usually lack sufficient randomness and are vulnerable to brute-force attacks. If a user employs the same password across all platforms, a breach in one application—especially if that application implements key derivation or hash storage poorly (for instance, by not using salting or even storing passwords in plaintext)—allows an attacker to obtain the plaintext password and subsequently compromise the user’s other accounts. Therefore, the core value of a password manager lies in generating high-entropy passwords and preventing password reuse, thereby reducing the risk of attacks.


---

However, password managers themselves are not flawless. I have used password managers from Google and Apple, as well as open‑source options like Bitwarden and KeePass. Commercial password managers carry the risk that a company might betray its users for economic gain or that insiders could act maliciously. Moreover, since they are not open source, it is difficult for users to be aware of, let alone supervise or prevent, these risks when they materialize. In addition, all password managers share a single point of failure: once an attacker gains access to the master password or main account (for example, by breaching a device or stealing cookies), they can unlock all the sensitive information stored within the manager.


---

## Advantages and Challenges of Hardware Keys

Hardware keys (such as those implementing FIDO2) can alleviate these issues to some extent. They are generally more secure than personal devices because they do not install third‑party software, which minimizes their attack surface and makes them difficult to crack online. FIDO2 protocols allow a hardware key to securely use its private key for decryption, signing, and authentication. Even if an attacker gains control of a user’s device, they can only obtain short‑term login credentials for a single platform.



---

Nevertheless, hardware keys are not without their vulnerabilities:

1. **Imperfect FIDO2 Implementation**  
   Currently, relatively few applications support FIDO2, and some platforms (such as Apple and Samsung) restrict the brands of acceptable hardware keys. For example, ProtonMail only supports FIDO2 as a two‑factor authentication (2FA) method rather than a complete replacement for passwords.
   
2. **Backup Issues**  
   The backup mechanisms for hardware keys have shortcomings. For instance, Google allows an Android phone to serve as a hardware key, but its backup relies on access to a Google account—exhibiting the same single point of failure as password managers. Devices like YubiKey require users to purchase multiple units to duplicate the private key, which may lead to backup devices failing due to aging or natural disasters (such as fires, floods, or earthquakes).

3. **Lack of Screen and Buttons**  
   Many hardware keys lack a screen or buttons, leaving users unable to verify the actions they are authorizing. An attacker might, for example, initiate a FIDO2 request on platform B at the same time a user logs into platform A, causing the user to unknowingly authorize an unintended operation. Similarly, a user might think they are decrypting a trivial piece of information (such as an encrypted email) when in fact an attacker is simultaneously initiating a decryption request on the password manager’s database.



---

## My Choice and Scheme Design

Based on the above analysis, I have chosen Trezor as my hardware key device. The primary reasons are its open‑source nature—instilling greater trust in the solution—and the expectation for future scalability. Although Trezor has some minor flaws in its encryption/decryption functions (such as the inability to completely confirm the content being decrypted), its overall security still exceeds that of traditional methods. To further enhance security, I designed a Trezor‑based password manager that employs the following scheme:


1. **Ethereum Signature to Generate a Random Seed**  
   An Ethereum signature is used to produce a 130‑character hexadecimal signature result, which is then hashed using SHA256 to generate a 256‑bit random number, and subsequently a key pair is derived from it.

2. **Layered Encryption**  
   A separate "master password" is generated for each password or piece of secret information, and it is encrypted via an Ethereum signature. For example, when encrypting the password for the Twitter account `ryder1987@gmail.com`, a descriptive string is generated (such as “password stored for `ryder1987@gmail.com` on Twitter.com”), and this string is signed using an uncommon BIP32 derivation path (for instance, `m/44h/60h/11h/0/12`). The resulting key pair is then used to encrypt the password with the public key. This obscure derivation path does not affect the security of the main wallet (e.g. `m/44h/60h/1h/0/1`).

3. **Identity Group Management**  
   The information within the password manager is divided into multiple identity groups. For some groups (such as everyday social accounts), the usernames and passwords are left unencrypted, while for sensitive groups (such as accounts used by investigative journalists), all information is encrypted and protected with misleading names (for example, “`ryder`’s registered porn site”) to preserve privacy.

For instance, if I wish to encrypt and store the automatically generated strong password for my Twitter account with the username `ryder1987@gmail.com`, I first create a formatted string—say, “decrypt the password stored for `ryder1987@gmail.com` on Twitter.com.” I then sign this string using an infrequently used BIP32 derivation path (for example, `m/44h/60h/11h/0/12`), ensuring that the default Ethereum main wallet (`m/44h/60h/1h/0/1`) remains secure. The signature result is used to generate a key pair, and the public key encrypts the password. Finally, the encrypted content along with the descriptive string (i.e. the hint) and the BIP32 path is stored. Later, when the password is needed, signing the hint once again allows retrieval of the private key to decrypt and recover the password.


---

### Advantages of the Scheme

- **Risk Diversification**: Each password is protected by its own independent “master password,” thereby increasing the difficulty for an attacker.
- **Layered Encryption**: Even if some information is leaked, the rest remains secure. For instance, even if an attacker compromises a device, they will only gain access to the limited information (such as the currently decrypted password) and not to other undecrypted passwords or usernames within other identity groups.
- **Hardware-level Security**: Using a hardware wallet for signing avoids the risk of the master password being brute‑forced or private key files being stored insecurely on a device’s disk.
- **Prevention of Blind Decryption**: The use of descriptive strings combined with an uncommon BIP32 path enables users to clearly understand what is being decrypted, thereby avoiding blind decryption or blind signing.

