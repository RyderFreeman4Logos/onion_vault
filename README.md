# OnionVault CLI

**The New Era Password Manager — OnionVault**

[中文版](docs/theory_paper/zh/README.md)

---

## Beta Demo
Click below to watch the demo:

[![Demo Video](https://img.youtube.com/vi/3Pv8UDvxwpY/0.jpg)](https://youtu.be/3Pv8UDvxwpY)

---

## Introduction

Traditional password managers often suffer from several issues:
- **Low-Entropy Memorized Passwords:** Most users choose only a few easy-to-remember passwords, resulting in insufficient randomness.
- **Single Point of Failure:** Centralized storage and a single master password mean that if compromised, all sensitive data is at risk.
- **Privacy and Trust Concerns:** Commercial solutions may sacrifice user privacy for profit, and opaque internal audits make it difficult to build trust.

To overcome these challenges, OnionVault offers an entirely new solution that strikes a perfect balance between security and convenience. Its key advantages include:

- **Hardware-Level Security:** By leveraging hardware keys such as Trezor, all key operations are executed within secure hardware, preventing private keys from being stored on vulnerable personal devices (which might be infected with malware or tricked by malicious links).
- **Layered Encryption Design:** Each password or piece of sensitive data—even entire identity groups—is encrypted with its own independent “master password.” This means that even if some data is compromised, the rest remains secure. (For further analysis, see [New Proposed Solution](docs/theory_paper/en/02_proposed_solution.md).)
- **Standing on the Shoulders of Giants:** OnionVault makes extensive use of the battle-tested security infrastructure developed by the cryptocurrency industry—a field where attackers are highly motivated by lucrative rewards and defenders invest massive resources to secure assets. By adopting proven standards like BIP39 for secure key backup using mnemonics, BIP32 for deterministic generation of sub-wallet addresses (or subkey pairs), and deterministic Ethereum signatures per RFC6979, we inherit decades of rigorous research and real-world defense mechanisms.
- **Cross-Platform Support:** Developed in Rust, OnionVault takes full advantage of Rust's inherent memory safety and high performance. The current version has been successfully tested on Linux and Windows 11, with plans to release TUI and GUI versions (and support for macOS, Android, and iOS) in the future.

Detailed implementation and underlying principles are documented in [docs/theory_paper](docs/theory_paper). Interested readers can explore further:
- [Introduction](docs/theory_paper/en/00_introduction.md)
- [Shortcomings of Existing Solutions](docs/theory_paper/en/01_existing_solutions.md)
- [New Proposed Solution](docs/theory_paper/en/02_proposed_solution.md)
- [FAQ](docs/theory_paper/en/03_appendix_faq.md)
- Additional information can be found in [The Prototype of This Project](docs/theory_paper/en/article.md) and [Key Generation via Signatures](docs/theory_paper/en/signature_to_keys.md)

---

## Feature Overview

- A command-line interface that is extremely concise and easy to audit.
- Layered encryption technology implemented via hardware key signing, ensuring that each password or sensitive data item is encrypted independently, thus eliminating single points of failure.
- Implemented in Rust, a language that inherently provides numerous security features.
- The current version successfully implements the basic functionality of the password manager on Linux and Windows 11 (macOS is theoretically supported). Future development plans are listed below.

---

## To Do *(Order Not Specified)*

* [x] **Autofill Passwords:** Automatically fill in passwords in browser or desktop application input fields on Windows, Linux, and macOS.
* [ ] **Code Audit:** Invite the open-source community or security experts to perform a comprehensive audit of the code and fix any vulnerabilities to ensure project security.
* [ ] **I18n:** (Using LLM) Translate the interface into multiple languages.
* [ ] **Comments and Documentation:** (Using LLM) Enhance the comments and documentation for each Rust file.
* [ ] **User Experience Optimization:** Improve the CLI interaction (including color schemes) to enhance usability and friendliness.
* [ ] **TOTP Support:** Support the storage of TOTP secrets and automatic refresh of 6-digit verification codes.
* [ ] **Extended Secret Storage:** In addition to passwords, support storage of text, backup keys, mnemonic phrases, and other types of sensitive information.
* [ ] **TUI Version**
* [ ] **GUI Version**
* [ ] **Android Version**
* [ ] **iOS Version**
* [ ] **Support for Additional Hardware Key Types**

---

## Participation and Contributions

This project welcomes contributions from developers and security enthusiasts alike. Join our discussion group and help build a safer password management solution!
| [Join the Discussion Room](https://matrix.to/#/#onionvault:matrix.org) |
|--------------------------------------------------------|

---

## Donations and Support

If you want to support me, please consider donating to support the development of more useful tools:
| [Cryptocurrency Address](https://etherscan.io/verifySig/263386) |
|---------------------------------------------------------|
