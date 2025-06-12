use serde::{Serialize, Deserialize};
use chrono::Utc;

/* 交易结构体
 * 表示区块链中的一笔交易
 * 在实际的比特币系统中，交易结构会更复杂，包含：
 * - 输入（UTXO）
 * - 输出
 * - 签名
 * - 公钥
 * - 交易费用等信息
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /* 发送方的地址
     * 在实际系统中，这是一个公钥哈希
     */
    pub sender: String,

    /* 接收方的地址
     * 在实际系统中，这是一个公钥哈希
     */
    pub recipient: String,

    /* 交易金额
     * 以比特币为单位（在这个简化版本中使用浮点数表示）
     * 实际系统中使用整数表示聪（satoshi）
     */
    pub amount: f64,

    /* 交易创建的时间戳
     * 使用 Unix 时间戳格式
     */
    pub timestamp: i64,
}

impl Transaction {
    /* 创建新的交易
     * 
     * 参数:
     * sender - 发送方地址
     * recipient - 接收方地址
     * amount - 交易金额
     * 
     * 返回:
     * 返回一个新的交易实例
     */
    pub fn new(sender: String, recipient: String, amount: f64) -> Self {
        Transaction {
            sender,
            recipient,
            amount,
            timestamp: Utc::now().timestamp(),
        }
    }

    /* 将交易转换为字符串表示
     * 
     * 用于在区块中存储交易信息
     * 在实际系统中，这里会包含更多的交易细节
     * 
     * 返回:
     * 返回交易的字符串表示
     */
    pub fn to_string(&self) -> String {
        format!(
            "{} -> {}: {} BTC",
            self.sender, self.recipient, self.amount
        )
    }
} 