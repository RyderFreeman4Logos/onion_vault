# 0 Introduction

- ## 0.1 Research Background and Motivation  
My research motivation stems from my profound personal experience---living in an authoritarian state that without internet access and freedom of speech. In such states, secure communication and privacy protection are not merely “hygiene practices” but essential measures to secure a brighter future for those you care about. My personal involvement in challenging authoritarian regimes through technological means has given me considerable insights and reflections on password management and privacy protection.

- ## 0.2 Objective Statement
This paper does not claim to offer a flawless solution; rather, it aims to provide a password management scheme that is safer and more ergonomically aligned than current market offerings. The problem we seek to solve is how to achieve high security while conforming to human cognitive habits to reduce user burden.

- ## 0.3 Why Not Use Traditional Password Management Solutions  
The core value of password managers lies in generating high-entropy passwords and preventing password reuse, thereby reducing the risk of attacks. However, existing password managers—whether commercial (such as those from Google and Apple) or open source (such as Bitwarden and KeePass)—have certain security vulnerabilities that are neither trivial nor negligible [Security Vulnerabilities](01_existing_solutions.md#12-%E9%9D%9E%E5%BC%80%E6%BA%90%E5%AF%86%E7%A0%81%E7%AE%A1%E7%90%86%E5%99%A8%E5%A6%82googleapplesamsung1password). This will be discussed in detail in the following section.

Hardware keys (e.g., those using FIDO2) offer an alternative that significantly reduces the attack surface but face issues such as imperfect implementation, backup difficulties, and insufficient user verification. On the other hand, FIDO2-supporting hardware cryptocurrency wallets (like Trezor), with their open-source nature, user confirmation mechanisms, and robust backup schemes, demonstrate enormous potential in the field of password management.

- ## 0.4 The New Solution: OnionVault  
Based on these observations and experiences, I propose a new solution that combines hardware wallets with password management. A Beta version has been developed and open sourced with the goal of providing more efficient protection for those who require security and privacy. We will discuss following topics:

- [The Advantages and Disadvantages of Existing Password Management Solutions](01_existing_solutions.md).  
- A [New Password Management Scheme Based on the Trezor Hardware Wallet](02_proposed_solution.md#22-%E6%9C%AC%E6%96%87%E7%AC%AC%E4%B8%80%E4%BD%9C%E8%80%85%E4%B8%BAtrezor%E5%BC%80%E5%8F%91%E7%9A%84%E5%AF%86%E7%A0%81%E7%AE%A1%E7%90%86%E5%99%A8-onionvault).  
- The new security features and other practical [Characteristics](./02_proposed_solution.md#23-%E6%96%B0%E7%9A%84%E5%AF%86%E7%A0%81%E7%AE%A1%E7%90%86%E6%96%B9%E6%A1%88onionvault%E5%85%B7%E5%A4%87%E5%A6%82%E4%B8%8Bfeatures) of the proposed scheme (some of which even conventional password managers have not properly implemented).  
- Considerations for user experience and usability such as [Using the System Without Memorizing or Typing a Password](02_proposed_solution.md#%E6%AF%94%E5%AF%86%E7%A0%81%E7%AE%80%E5%8D%95), [Password Auto-fill](02_proposed_solution.md#password-auto-fill), and [Replacing Memorized Passwords with Backup Mnemonics](02_proposed_solution.md#%E5%8F%AF%E4%BB%A5%E4%B8%8D%E7%94%A8%E8%AE%B0%E5%BF%86%E5%AF%86%E7%A0%81).  
- [Future Development Directions](02_proposed_solution.md#24-onionvault%E7%9A%84%E6%9C%AA%E6%9D%A5).  
- [Appendix and FAQ](03_appendix_faq.md).

Through this, I hope to provide everyone with a safer and more reliable tool that enables the free sharing of information and viewpoints without being monitored or attacked.



| [Next page](01_existing_solutions.md) |
|---------------------------------------|
