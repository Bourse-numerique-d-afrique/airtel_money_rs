use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionRefundResponse {
    pub data: Data,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub transaction: Transaction,
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    pub airtel_money_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    pub code: String,
    pub message: String,
    pub result_code: String,
    pub success: bool,
}