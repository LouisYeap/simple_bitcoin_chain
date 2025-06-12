# Simple Bitcoin Chain

这是一个使用 Rust 实现的简化版比特币区块链系统，用于学习和理解区块链的基本概念和工作原理。

## 项目简介

本项目实现了一个简化版的比特币区块链系统，包含以下核心功能：

- 区块的创建和链接
- 交易的处理和打包
- 工作量证明（挖矿）机制
- 区块链验证

虽然这是一个简化版本，但它展示了比特币的核心概念和基本工作原理。

## 功能特点

- 🔗 区块链接：每个区块都通过哈希值链接到前一个区块
- 💰 交易处理：支持基本的交易创建和处理
- ⛏️ 工作量证明：实现了基本的挖矿机制
- ✅ 区块链验证：可以验证整个区块链的完整性
- 📝 详细注释：代码包含完整的注释，便于理解

## 技术栈

- Rust 2025
- SHA256 哈希算法
- 序列化/反序列化 (serde)
- 时间处理 (chrono)

## 系统要求

- Rust 1.70.0 或更高版本
- Cargo 包管理器
- 至少 2GB 可用内存
- 支持的操作系统：
  - Linux (推荐 Ubuntu 20.04 或更高版本)
  - macOS
  - Windows 10/11 (需要 WSL2)

## 项目结构

```
simple_bitcoin_chain/
├── src/
│   ├── block.rs      // 区块结构定义
│   ├── transaction.rs // 交易结构定义
│   ├── blockchain.rs  // 区块链实现
│   └── main.rs       // 主程序入口
├── Cargo.toml        // 项目配置和依赖
└── README.md         // 项目文档
```

## 安装说明

1. 确保已安装 Rust 开发环境：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装必要的系统依赖（Ubuntu/Debian）：
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

3. 克隆项目：
```bash
git clone https://github.com/Louis/simple_bitcoin_chain.git
cd simple_bitcoin_chain
```

4. 编译项目：
```bash
cargo build --release
```

## 使用方法

### 基本使用

运行项目：
```bash
cargo run
```

### 高级使用示例

1. 创建自定义交易：
```rust
use simple_bitcoin_chain::transaction::Transaction;
use simple_bitcoin_chain::blockchain::Blockchain;

let mut blockchain = Blockchain::new();

// 创建多个交易
let transaction1 = Transaction::new(
    "Alice".to_string(),
    "Bob".to_string(),
    10.0,
);

let transaction2 = Transaction::new(
    "Bob".to_string(),
    "Charlie".to_string(),
    5.0,
);

// 添加交易到区块链
blockchain.add_transaction(transaction1);
blockchain.add_transaction(transaction2);

// 执行挖矿
blockchain.mine_pending_transactions();
```

2. 调整挖矿难度：
```rust
let mut blockchain = Blockchain::new();
blockchain.difficulty = 5; // 增加挖矿难度
```

3. 验证区块链：
```rust
if blockchain.is_chain_valid() {
    println!("区块链验证通过！");
} else {
    println!("区块链验证失败！");
}
```

4. 查看区块信息：
```rust
for (i, block) in blockchain.chain.iter().enumerate() {
    println!("区块 #{}", i);
    println!("时间戳: {}", block.timestamp);
    println!("交易数量: {}", block.transactions.len());
    println!("哈希值: {}", block.hash);
}
```

## 核心概念说明

### 区块（Block）
- 包含时间戳、交易列表、前一个区块的哈希、当前区块的哈希和 nonce 值
- 使用 SHA256 算法计算哈希值
- 通过工作量证明机制创建新区块

### 交易（Transaction）
- 包含发送方、接收方、金额和时间戳
- 存储在待处理交易池中
- 在挖矿过程中被打包到新区块

### 区块链（Blockchain）
- 维护区块的链式结构
- 管理待处理交易
- 实现挖矿功能
- 提供区块链验证机制

## 扩展建议

这个项目可以进一步扩展，添加更多功能：

1. 钱包系统
   - 公钥/私钥生成
   - 交易签名
   - 余额管理

2. 网络功能
   - P2P 网络通信
   - 节点发现
   - 区块同步

3. 共识机制
   - 更复杂的挖矿算法
   - 难度调整
   - 分叉处理

4. 智能合约
   - 脚本系统
   - 合约执行
   - 状态管理

## 贡献指南

欢迎提交 Issue 和 Pull Request 来改进这个项目。在提交代码之前，请确保：

1. 代码符合 Rust 编码规范
2. 添加适当的测试
3. 更新相关文档

## 许可证

本项目采用 MIT 许可证。详情请查看 [LICENSE](LICENSE) 文件。

MIT 许可证的主要条款：
- 允许商业使用
- 允许修改
- 允许分发
- 允许私有使用
- 不提供任何保证

## 作者

Louis

## 致谢

- 比特币白皮书
- Rust 社区
- 所有贡献者

## 常见问题

1. **Q: 为什么我的挖矿速度很慢？**
   A: 挖矿速度取决于您的计算机性能和设置的难度值。可以尝试降低难度值来加快挖矿速度。

2. **Q: 如何添加更多交易？**
   A: 使用 `blockchain.add_transaction()` 方法添加新交易，然后调用 `mine_pending_transactions()` 进行挖矿。

3. **Q: 如何验证区块链的完整性？**
   A: 使用 `blockchain.is_chain_valid()` 方法验证整个区块链的完整性。

## 更新日志

### v1.0.0 (2024-03-xx)
- 初始版本发布
- 实现基本的区块链功能
- 添加工作量证明机制
- 实现交易处理
# simple_bitcoin_chain
