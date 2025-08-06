use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bill {
    pub id: Option<u32>,
    pub description: String,
    pub amount: f64,
    pub bill_type: String, // "income" or "expense"
    pub date: String,      // YYYY-MM-DD format
}

impl Bill {
    pub fn new(description: String, amount: f64, bill_type: String, date: String) -> Self {
        Bill {
            id: None,
            description,
            amount,
            bill_type,
            date,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub income: f64,
    pub expense: f64,
    pub balance: f64,
}