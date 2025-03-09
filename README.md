# solana-roadmap
记录下学习solana的流程，本文持续更新中。

## Rust语法基础
<https://doc.rust-lang.org/stable/rust-by-example/>

如果你之前有一些开发经验的话，我推荐直接阅读rust example快速掌握语法，花上一天半天的时间把example上的例子过一遍。当然一般人不可能全部记住，但是下次看到不太懂的语法的时候可以快速找到例子对应的地方再熟悉下记忆。

## Solana 账户模型
<https://solana.com/developers/cookbook>

如果你已经有evm开发经验的话，会发现solana上的数据持久化和evm上的有天壤之别，solana上的数据持久化都需要依赖PDA也就是程序衍生账号。推荐直接阅读官网的cookbook快速掌握相关的知识。

## Solana 交易和签名


## Anchor 框架
<https://www.anchor-lang.com/>

Anchor是Solana的主要开发框架，用于构建Solana程序(智能合约)。主要有以下特点:
- 简化了编写、测试、部署和与Solana程序交互的过程
- 通过内置的安全特性帮助开发者更快地构建生产级应用，同时减少潜在的漏洞
- 使用宏和特征来简化Rust代码，为程序提供清晰的结构，让开发者可以更专注于功能实现

### Anchor基础
- [程序结构](./examples/lib.rs)
- IDL文件(Interface Description Language)
- PDA
- 程序之间的交互

### SPL Tokens
