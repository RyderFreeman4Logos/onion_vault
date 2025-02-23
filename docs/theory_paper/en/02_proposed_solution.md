# 2 The Proposed Solution

- # 2.1 Choosing Trezor Among Hardware Keys
  - ## 2.1.1 **Advantages Over Other Hardware Keys**
    - ### Unlike Google/Apple’s passkey, its attack surface is smaller.
    - ### Unlike YubiKey, Trezor can utilize BIP39, allowing a 256‑bit private key to be backed up by recording 24 words. Users can prevent loss of the private key by writing down multiple copies of the mnemonic or etching it onto stainless steel plates. An optional Shamir Backup further increases backup resilience.
    - ### Unlike most hardware keys, Trezor features a touchscreen, so if the software support is adequate, users can clearly see the operation they are authorizing.
    - ### Unlike more dominant brands such as Ledger, Trezor is more open source. This makes it easier to audit, develop new features for, or even build your own hardware key using off‑the‑shelf components or a Raspberry Pi.

  - ## 2.1.2 **Unresolved Issues**
    - ### 1. FIDO2 is implemented imperfectly—for instance, Samsung inexplicably prohibits using Trezor as a hardware key.
    - ### 2. If FIDO2 is not used or if a different age key pair is generated for each distinct secret, the risk of a single point of failure with the master password still exists.
    - ### 3. While using Trezor together with age (by generating a distinct age key pair for each password) to encrypt and store passwords is secure and avoids single point failure, it is not very convenient and may lead to operational mistakes.

- # 2.2 OnionVault – The Trezor‑Based Password Manager Developed by the First Author  
  - ## Generating a Dedicated 256‑bit Key Pair for Each Password or Secret  
    For example, when encrypting the password for the Twitter account `ryder1999@gmail.com`, a descriptive string (such as “password stored for ryder1987@gmail.com on Twitter.com”) is generated. A seldom‑used BIP32 derivation path (for example, `m/44h/60h/11h/0/12`) is used to sign this string, producing a key pair, after which the public key is used to encrypt the password. This obscure derivation path does not affect the security of the main wallet (e.g., `m/44h/60h/1h/0/1`).
  - ## Identity Group Management  
    The password manager’s information is divided into multiple Identity Groups. For some groups (such as everyday social accounts), the usernames and passwords are not encrypted; for sensitive groups (such as accounts used by investigative journalists or political dissidents), even the usernames are encrypted and misleading “Group Names” (for example, “ryder’s registered porn site”) are used to protect the truly important identities.
  - ## Ethereum Signature as the Core Function of the Hardware Wallet  
    The reason for using Ethereum signatures instead of `trezorctl crypto encrypt-keyvalue` is that Ethereum signatures are destined to receive more security auditing resources and are better suited for adaptation to any other hardware or software wallet that supports Ethereum signatures.
  - ## Modern Encryption Scheme: age  
    The current encryption scheme takes the Ethereum signature, applies SHA256 to generate a 256‑bit random seed, and then derives an ssh_ed25519 key pair from that seed. This key pair is imported into [rage](https://github.com/str4d/rage/tree/main/age) (the Rust version of age) to generate the Identity and Recipient. The use of age not only simplifies development but also leverages its feature of easily encrypting for multiple recipients to implement the “Backup Key for OnionVault” functionality.

- # 2.3 Features of the New Password Management Scheme “OnionVault”
  - ## Layered Encryption Like an Onion  
    Unlike most mainstream password managers, even if part of the information is leaked, other information remains secure. For example, even if an attacker compromises a device, they will only be able to access the limited information (such as the currently decrypted password) on that device, and cannot access other undecrypted passwords or the usernames encrypted under other identity groups.
  - ## Preventing Blind Decryption  
    The descriptive string (Hint) shown on the Trezor touchscreen allows the user to confirm which password or piece of information is being processed, thereby avoiding the risk of blind signing.
  - ## Password Auto-fill  
    Unlike some password managers that require copying and pasting to input passwords, OnionVault uses the cross-platform tool enigo to automatically switch windows and fill in passwords, preventing passwords from appearing on the clipboard. (This has been successfully tested on the original author’s Linux computer and a borrowed Windows 11 machine; macOS should also be supported in theory, though it may require user experimentation, and mobile support will be added in future updates.)
  - ## No Need to Memorize Passwords  
    Users need only to properly back up the mnemonic during initialization as instructed by Trezor-Suite. They can choose the native 24-word backup or use Shamir Backup to split the backup across different locations; backup media options range from writing it on paper and clipping it into a book to purchasing multiple professional stainless steel plates with punched records (which are resistant to water, fire, and earthquakes).
  - ## Simpler Than Passwords  
    Using Trezor is simpler in operation than remembering and entering a sufficiently strong master password each time.
  - ## Trezor Natively Supports Passphrases  
    Users can input an **additional** passphrase via the touchscreen when using Trezor to derive new keys (or simply press Enter to skip). This adds difficulty for attackers if Trezor is stolen or lost (since the attacker must know both the private key and the passphrase).
  - ## Boldly Store  
    The 256‑bit encrypted password database can be safely stored on various cloud storage services or synchronized to a GitHub repository.
  - ## Trezor Can Natively Serve as a FIDO2 Device in Many Applications  
    - It can be used for passwordless logins: Google, GitHub, Coinbase, Bitwarden, users.rust-lang.org, etc.  
    - It can also be used for two-factor authentication (2FA): Twitter, ProtonMail, microsoft.com, etc.
  - ## Trezor Can Also Be Used for macOS and Linux Screen Lock and sudo Authentication  
    - [sudo](https://trezor.io/learn/a/what-is-u2f#:~:text=add%20U2F%20to-,sudo%20command)  
    - [macOS](https://blog.trezor.io/trezor-u2f-login-into-your-mac-os-x-9fdd808b0aa4#.lmymg29l6)  
    - [Linux](https://blog.trezor.io/trezor-u2f-login-into-your-linux-mint-bd3684d4a8ba)

- # 2.4 The Future of OnionVault
  - ## User Interface Enhancements  
    - The CLI version is extremely minimalist; the current color scheme and internationalization (I18n) still require improvement (personally, I tend to favor the CLI because it has less code and is easier to audit).  
    - However, some users may wish to use a GUI or at least a TUI, which will require developers’ efforts.  
    - Android and iOS adaptations are also needed; at the very least, they should allow automatic password filling on mobile devices similar to KeepassDx.
  - ## Synchronization and Backup Features  
    - Every change will be recorded using git to log the history of the encrypted password database, preventing accidental deletion of login credentials.  
    - A dedicated ssh key pair will be generated specifically for pushing and pulling the user’s encrypted password database from their GitHub repository.
  - ## Encrypting More Types of Information  
    - Add support for TOTP.  
    - Add support for Backup Key.  
    - Allow users to encrypt arbitrary strings.
  - ## Support for More Hardware Devices and Encryption Algorithms  
    - Increase support for other hardware keys.  
    - Provide “Wallet Connect” for users who cannot use hardware keys.  
    - For users who do not wish to use the current beta version of [rage](https://docs.rs/age/latest/age/#:~:text=Caution:%20all%20crate%20versions%20prior%20to%201.0%20are%20beta%20releases%20for%20testing%20purposesonly) for encryption, consider supporting native ssh_ed25519 for encryption.

# Conclusion  
This paper proposes a novel password management solution based on the Trezor hardware wallet, aiming to resolve the security vulnerabilities and single point of failure issues in existing password managers. By employing Ethereum signatures to generate random seeds, layered encryption, and identity group management, we effectively enhance the security and privacy of passwords. At the same time, we focus on user experience to ensure the solution is practical to operate.

Although the solution offers a range of security advantages, further auditing and optimization are needed. We hope that this work will provide everyone with a safer and more reliable password management tool.


| [Next Page](03_appendix_faq.md)|
|--------------------------------|
