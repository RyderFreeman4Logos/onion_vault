# 1 Analysis of Existing Solutions

We begin by examining current password management solutions, highlighting both the advantages they have over previous methods and the issues that remain unresolved without further improvements:

- # 1.1 Passwords Memorized by the Human Brain,
  - ## 1.1.1 **Advantages**
    Easy and convenient.
  - ## 1.1.1 **Shortcomings**  
    Since most people do not possess a genius-level memory and are naturally inclined toward laziness, the following issues are highly likely to occur:

    - ### a. The passwords used do not have sufficient randomness (i.e. are too simple)

      <details markdown="1"><summary>Click to expand attack methods</summary>
          They are easily vulnerable to dictionary attacks and brute-force attacks.
      </details>

    - ### b. Even if one can remember high-entropy passwords, it is impractical to remember multiple different ones (leading to password reuse across apps). If one app is compromised and that app:
      - #### 1. Does not store passwords with salted hashing,
      - #### 2. Uses a weak hashing algorithm or has issues with salt generation, or
      - #### 3. Stores passwords using salted hashes but does not implement the salting on the client side,  
      then unsalted passwords can be extracted from the decrypted data packet, exposing login credentials for other apps as well.

- # 1.2 **Non-Open-Source Password Managers** (such as those from Google, Apple, Samsung, 1Password):
  - ## 1.2.1 **Advantages**: Easy to use
    They require the user to remember only one password to automatically generate and fill in a multitude of high-entropy passwords, and most offer automatic synchronization.
  - ## 1.2.2 **Shortcomings**:
    - ### a. Commercial companies might sell customer privacy (usernames) or even plaintext passwords, since many non‑open‑source password managers are not designed so that the provider has absolutely no ability to sell user data.
    - ### b. Insider threats (corporate spies or undercover agents from one’s own country or hostile governments) might attempt to attack users.
    - ### c. The security of the software can only be audited by a select few or specialized security firms, rather than being publicly available for scrutiny by everyone.
    - ### d. They also face almost all of the issues that open-source password managers encounter.

- # 1.3 **Open-Source or Semi-Open-Source Password Managers** (such as pass/passage, the KeePass series, Bitwarden):
  - ## 1.3.1 **Advantages**:  
    They offer high transparency, strong community support, and make security auditing easier.
  - ## 1.3.2 **Shortcomings**:
    - ### a. There remains a single point of failure risk, primarily relying on the security of the master password.  
      For example, if a user enters the master password on a compromised device to unlock the password database, attackers can capture the master password as well as all the information in the database through methods such as keylogging or memory scraping; even using hardware keys for two‑factor authentication does not eliminate this risk.

- # 1.4 **Hardware Keys** (FIDO2):
  - ## 1.4.1 **Advantages**:
    - ### a. Professional hardware keys are generally more secure than personal devices because they do not install third‑party software, thus presenting a smaller attack surface.
    - ### b. They are designed with significant investment in security and are generally considered unlikely to be compromised online (i.e. extracting the internal private key is very difficult).
    - ### c. Even if an attacker gains control of the user’s device (computer/phone), they will likely only obtain short‑lived login credentials for a limited number of platforms.
  - ## 1.4.2 **Shortcomings**:

    - ### a. **Imperfect FIDO2 Implementation**:  
      Currently, very few applications support FIDO2, and some platforms (such as Apple and Samsung) restrict the brands of hardware keys. In addition, some apps (for example, ProtonMail) only support FIDO2 as a form of two‑factor authentication (2FA) and cannot completely replace passwords. A solution is needed for this transitional period until FIDO2 is widely adopted and can be used exclusively without requiring a password during registration.

    - ### b. **Single Point of Failure in Some Hardware Keys**:  
      Although Apple and Google support passkeys, if an iCloud or Google account is hacked, the passkey may be misused by an attacker.

    - ### c. **Lack of a Screen or Buttons**:  
      Many hardware key devices lack a screen or even buttons, which prevents the user from confirming the operation they are approving. For example:
      - #### 1. An attacker might initiate a FIDO2 request for another platform (B) while the user logs into platform A, and the user may unknowingly approve the attacker’s desired operation.
      - #### 2. Similarly, a user may believe they are decrypting an insignificant piece of information (such as an email encrypted by a friend using the user’s public key), but in reality, the attacker is simultaneously initiating a decryption request for the password manager database.

    - ### d. **Backup Issues**:  
      The backup mechanisms for hardware keys have flaws. For example:
      - #### 1. When companies like Google and Apple use a phone or computer as a hardware key, the backup relies on the app account’s access permissions, carrying the same single point of failure risk as password managers; additionally, the system‑level protection is generally inferior to that of professional hardware keys.
      - #### 2. Devices like YubiKey require the purchase of multiple hardware keys to duplicate the private key, which may lead to loss of the private key if the backup device deteriorates or is destroyed by natural disasters (e.g., fire, flood, earthquake).
