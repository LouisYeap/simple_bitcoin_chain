use crate::block::Block;
use crate::transaction::Transaction;

/* 区块链结构体
 * 管理整个区块链系统，包括：
 * - 维护区块的链式结构
 * - 管理待处理交易
 * - 处理挖矿过程
 * - 验证区块链的完整性
 */
pub struct Blockchain {
    /* 区块链中的所有区块
     * 按时间顺序存储，每个区块都链接到前一个区块
     */
    pub chain: Vec<Block>,

    /* 挖矿难度
     * 决定区块哈希值需要满足的条件
     * 难度越高，挖矿所需的时间越长
     */
    pub difficulty: usize,

    /* 待处理的交易池
     * 存储尚未被打包到区块中的交易
     * 当挖出新区块时，这些交易会被打包并清空
     */
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    /* 创建新的区块链
     * 
     * 初始化区块链，创建创世区块
     * 创世区块是区块链的第一个区块，其 previous_hash 为全0
     * 
     * 返回:
     * 返回一个新的区块链实例
     */
    pub fn new() -> Self {
        let mut chain = Vec::new();
        // 创建创世区块
        chain.push(Block::new(
            vec!["Genesis Block".to_string()],
            "0".repeat(64),
        ));

        Blockchain {
            chain,
            difficulty: 4, // 设置初始挖矿难度
            pending_transactions: Vec::new(),
        }
    }

    /* 获取最新的区块
     * 
     * 返回:
     * 返回区块链中最后一个区块的引用
     */
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /* 添加新的交易到待处理交易池
     * 
     * 参数:
     * transaction - 要添加的交易
     */
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    /* 挖矿过程
     * 
     * 1. 将待处理交易打包到新区块
     * 2. 执行工作量证明
     * 3. 将新区块添加到链中
     * 4. 清空待处理交易池
     */
    pub fn mine_pending_transactions(&mut self) {
        // 将待处理交易转换为字符串列表
        let transactions: Vec<String> = self.pending_transactions
            .iter()
            .map(|t| t.to_string())
            .collect();

        // 创建新区块
        let mut new_block = Block::new(
            transactions,
            self.get_latest_block().hash.clone(),
        );

        // 执行工作量证明
        new_block.mine_block(self.difficulty);

        // 将新区块添加到链中
        self.chain.push(new_block);

        // 清空待处理交易
        self.pending_transactions.clear();
    }

    /* 验证区块链的完整性
     * 
     * 检查：
     * 1. 每个区块的哈希值是否正确
     * 2. 区块是否按正确的顺序链接
     * 
     * 返回:
     * 如果区块链有效返回 true，否则返回 false
     */
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // 验证当前区块的哈希是否正确
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            // 验证区块是否链接到前一个区块
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
} 