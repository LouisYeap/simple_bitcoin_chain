use sha2::{Sha256, Digest};
use chrono::prelude::*;

mod block;
mod transaction;
mod blockchain;

use blockchain::Blockchain;
use transaction::Transaction;

/* 主程序入口
 这个程序演示了一个简化版比特币区块链的基本功能：
 1. 创建区块链
 2. 添加交易
 3. 挖矿
 4. 验证区块链
 */

fn main() {
    // 创建新的区块链
    // 这会自动创建创世区块
    let mut blockchain = Blockchain::new();
    println!("创建新的区块链...");

    // 添加一些测试交易
    // 这些交易会被存储在待处理交易池中
    println!("\n添加一些测试交易...");
    blockchain.add_transaction(Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        10.0,
    ));
    blockchain.add_transaction(Transaction::new(
        "Bob".to_string(),
        "Charlie".to_string(),
        5.0,
    ));

    // 开始挖矿过程
    // 这将把待处理交易打包到新区块中
    println!("\n开始挖矿...");
    blockchain.mine_pending_transactions();

    // 添加更多交易
    // 这些交易会在下一次挖矿时被打包
    println!("\n添加更多交易...");
    blockchain.add_transaction(Transaction::new(
        "Charlie".to_string(),
        "Alice".to_string(),
        2.5,
    ));

    // 再次进行挖矿
    // 将新的交易打包到新区块中
    println!("\n再次开始挖矿...");
    blockchain.mine_pending_transactions();

    // 打印区块链信息
    // 显示每个区块的详细信息
    println!("\n区块链信息:");
    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("\n区块 #{}", i);
        println!("时间戳: {}", block.timestamp);
        println!("交易: {:?}", block.transactions);
        println!("前一个哈希: {}", block.previous_hash);
        println!("哈希: {}", block.hash);
        println!("Nonce: {}", block.nonce);
    }

    // 验证区块链的完整性
    // 检查所有区块的哈希值和链接是否正确
    println!("\n验证区块链...");
    if blockchain.is_chain_valid() {
        println!("区块链有效！");
    } else {
        println!("区块链无效！");
    }
}