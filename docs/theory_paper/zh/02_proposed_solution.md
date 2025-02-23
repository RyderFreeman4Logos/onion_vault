# 2 我提出的解决方案

- # 2.1 在硬件密钥中选择Trezor
  - ## 2.1.1 **相比其他硬件密钥的优点**
    - ### 不同于Google/Apple的passkey, 攻击面更小
    - ### 不同于Yubikey, Trezor可以使用BIP39, 通过记录24个单词来备份256位私钥，用户可以通过抄写多份助记词或将其刻在不锈钢板上来防止私钥丢失。可选的沙米尔备份（Shamir Backup）进一步提高了备份的抗风险能力。
    - ### 不同于大多数硬件密钥, Trezor有触摸屏, 如果软件支持足够好，用户可以清楚地看到自己正在批准的操作具体是在干什么
    - ### 不同于可能市占率更高的Ledgar, Trezor更开源, 这使得我们更容易去审计它, 为它开发新功能; 甚至可以自己使用电子元件或树莓派制造硬件密钥

  - ## 2.1.2 **尚未解决的问题**
    - ### 1. FIDO2实施不完善, 比如三星不知为何禁止使用trezor作为硬件密钥
    - ### 2. 如果不使用FIDO2或为每一个不同的机密信息都生成不同的age密钥对, 则还是存在主密码单点故障问题
    - ### 3. 如果使用trezor搭配age(为每一个密码都生成一个不同的age密钥对)来加密储存密码确实足够安全, 不存在单点故障问题, 但不够方便且有可能操作失误


- # 2.2 本文第一作者为Trezor开发的密码管理器 OnionVault
  - ## 为每个密码或机密信息生成专门的256位密钥对
    例如，为Twitter账号`ryder1999@gmail.com`的密码加密时，生成描述字符串（如`ryder1987@gmail.com在Twitter.com中存储的密码`），并使用冷门的BIP32派生路径（如`m/44h/60h/11h/0/12`）签名该字符串，生成密钥对后使用公钥加密密码。这种冷门路径不会影响主要钱包（如`m/44h/60h/1h/0/1`）的安全。
  - ## 身份组管理
    将密码管理器中的信息分为多个身份组（Identity Group），部分身份组（如日常社交账号）不加密用户名和密码，而敏感身份组（如身为调查记者或政治异见人士使用的账号）则连用户名都会加密，并使用误导性`Group Name`（如`ryder注册的色情网站`）保护真正重要的身份。
  - ## ETH签名是硬件钱包的核心功能
    之所以使用`eth签名`而不是`trezorctl crypto encrypt-keyvalue`是因为前者注定会被投入更多资源进行安全审计, 而且也有利于适配其他任何具有`eth签名`功能的硬件钱包和软件钱包
  - ## 现代加密方案age
    目前使用的加密方案是对`ETH Signature`进行`SHA256`生成一个256位的随机种子, 然后从随机种子派生出ssh_ed25519密钥对, 然后用[rage](https://github.com/str4d/rage/tree/main/age)(rust版age)导入私钥生成Identity和Recipient; 使用age是为了开发更方便以及利用age`可以方便地对多个Recipient加密`这一feature实现`Backup Key for OnionVault`功能

 
- # 2.3 新的密码管理方案`OnionVault`具备如下Features
  - ## 像洋葱一样层层加密
    不同于绝大部分主流密码管理器, 即使部分信息泄露，其他信息仍可以保持安全。例如，攻击者即使入侵了设备，也只能获取在该台设备上解密过的有限信息（如当前解密的密码），而无法获取其他未解密的密码或加密的身份组下的用户名集合。
  - ## 防止盲解密
    通过Trezor触摸屏上显示的描述字符串（Hint），用户可以确认自己正在处理的密码或信息是哪个，避免盲签名风险。
  - ## Password Auto fill
    相比于某些密码管理器只能通过复制粘贴来填入密码, OnionVault 使用了跨平台的enigo来自动切换窗口然后填写密码, 避免密码出现在剪切板(目前在原作者的Linux电脑和借来的Windows11上成功测试, 理论上Macos也支持, 但需要自行尝试, 移动设备用户暂时需要等待开发者们后期添加支持)
  - ## 可以不用记忆密码
    只需要在初始化时按照Trezor-Suite的指示妥善备份助记词即可, 可以选择原生的24词备份, 也可以选择使用`Shamir Backup`来在多地分片备份; 备份介质可以选择`用笔抄在纸上夹进书里`, 也可以二十多块买专业的`不锈钢板打孔记录`(防水防火灾防地震)
  - ## 比密码简单
    使用Trezor比记住并每次输入足够强的主密码在操作上更简单
  - ## Trezor 原生支持PassPhrase
    用户可以在使用Trezor时通过触摸屏填写**额外**的密码来派生新密钥(也可以直接回车不填), 这样可以在Trezor被抢走或遗失时增加破解难度(攻击者必须同时知道私钥和PassPhrase)
  - ## 大胆地存
    256位加密后的密码数据库可以放心地存储到各种网盘/同步到GitHub存储库
  - ## Trezor原本就可以在不少Application用作FIDO2设备
    - 可用于免密码登录: Google, GitHub, CoinBase, bitwarden, users.rust-lang.org, Etc
    - 可用于2FA: Twitter, ProtonMail, microsoft.com, Etc
  - ## Trezor还能用于macos和linux的锁屏解锁和sudo
    - [sudo](https://trezor.io/learn/a/what-is-u2f#:~:text=add%20U2F%20to-,sudo%20command)
    - [Macos](https://blog.trezor.io/trezor-u2f-login-into-your-mac-os-x-9fdd808b0aa4#.lmymg29l6)
    - [Linux](https://blog.trezor.io/trezor-u2f-login-into-your-linux-mint-bd3684d4a8ba)


- # 2.4 OnionVault的未来
  - ## 用户界面完善
    - CLI版本是极简版, 目前的配色方案和I18n还有待优化(另外我个人其实倾向于一直使用CLI, 因为代码量最少, 易于审计)
    - 但还是会有用户会想要使用GUI或至少TUI, 有待开发者努力
    - Android和IOS也需要想办法适配, 至少要能够在移动设备上像KeepassDx那样自动填入密码
  - ## 同步备份功能
    - 每次改动后使用git来记录加密后的密码数据库的变化历史, 防止误删登录凭证
    - 专门生成一个ssh密钥对用于push和pull用户用于存储加密后密码数据库的github仓库
  - ## 加密更多类型的信息
    - 增加对`TOTP`的支持
    - 增加对`Backup Key`的支持
    - 让用户能够自由加密特定字符串
  - ## 支持更多硬件设备和加密算法
    - 增加对其他硬件密钥的支持
    - 为没有条件使用硬件密钥的用户提供`Wallet Connect`
    - 有些用户并不想用目前还是[Beta版的rage](https://docs.rs/age/latest/age/#:~:text=Caution:%20all%20crate%20versions%20prior%20to%201.0%20are%20beta%20releases%20for%20testing%20purposesonly)来加密, 考虑支持使用原生ssh_ed25519进行加密


# 结论
本文提出了一个基于Trezor硬件钱包的新型密码管理方案，旨在解决现有密码管理器中的安全隐患和单点故障问题。通过采用以太坊签名生成随机种子、分层加密和身份组管理，我们有效地增强了密码的安全性和隐私保护。同时，我们也关注用户体验，希望确保方案在实际使用中易于操作。

尽管这个方案提供了一系列安全优势，但需要进一步的审计和优化。我们希望通过此工作，能够为所有人提供一个更安全、更可靠的密码管理工具

| [下一页](03_appendix_faq.md)|
|----------------------------|
