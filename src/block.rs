use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use hex;

/* 区块结构体
 * 这是区块链中的基本单位，每个区块包含多个交易记录
 * 通过哈希值与前一个区块链接，形成链式结构
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /* 区块创建的时间戳
     * 使用 Unix 时间戳格式，记录区块创建的确切时间
     */
    pub timestamp: i64,

    /* 区块中包含的交易列表
     * 在实际的比特币系统中，这里会包含更复杂的交易结构
     * 包括输入、输出、签名等信息
     */
    pub transactions: Vec<String>,

    /* 前一个区块的哈希值
     * 这是区块链链接的关键，确保区块按顺序链接
     * 创世区块的 previous_hash 为全0
     */
    pub previous_hash: String,

    /* 当前区块的哈希值
     * 通过对区块内容进行 SHA256 哈希计算得到
     * 包含时间戳、交易数据、前一个区块哈希和 nonce 值
     */
    pub hash: String,

    /* 工作量证明的随机数
     * 在挖矿过程中不断调整，直到找到满足难度要求的哈希值
     */
    pub nonce: u64,
}

impl Block {
    /* 创建新的区块
     * 
     * 参数:
     * transactions - 要包含在区块中的交易列表
     * previous_hash - 前一个区块的哈希值
     * 
     * 返回:
     * 返回一个包含初始哈希值的新区块
     */
    pub fn new(transactions: Vec<String>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    /* 计算区块的哈希值
     * 
     * 使用 SHA256 算法计算区块的哈希值
     * 哈希输入包括：
     * - 时间戳
     * - 交易数据（JSON 格式）
     * - 前一个区块的哈希
     * - nonce 值
     * 
     * 返回:
     * 返回计算得到的哈希值的十六进制字符串表示
     */
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let content = format!(
            "{}{}{}{}",
            self.timestamp,
            serde_json::to_string(&self.transactions).unwrap(),
            self.previous_hash,
            self.nonce
        );
        hasher.update(content.as_bytes());
        hex::encode(hasher.finalize())
    }

    /* 执行工作量证明（挖矿）
     * 
     * 通过不断调整 nonce 值，直到找到满足难度要求的哈希值
     * 难度由前导零的数量决定，例如难度为4时，要求哈希值以4个0开头
     * 
     * 参数:
     * difficulty - 挖矿难度，决定哈希值需要满足的条件
     */
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("区块已挖出: {}", self.hash);
    }
} 