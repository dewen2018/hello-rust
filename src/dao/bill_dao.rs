use mysql::*;
use mysql::prelude::*;
use crate::model::bill::{Bill, Summary};
use crate::utils::{value_to_f64, date_to_string};

pub struct BillDAO {
    pool: Pool,
}

impl BillDAO {
    pub fn new(pool: Pool) -> Self {
        BillDAO { pool }
    }
    
    pub fn init_database(&self) -> mysql::Result<()> {
        let mut conn = self.pool.get_conn()?;
        
        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS bills (
                id INT AUTO_INCREMENT PRIMARY KEY,
                description TEXT NOT NULL,
                amount DECIMAL(10,2) NOT NULL,
                bill_type ENUM('income', 'expense') NOT NULL,
                date DATE NOT NULL
            )"
        )?;
        
        println!("Database initialized successfully!");
        Ok(())
    }
    
    pub fn add_bill(&self, bill: &Bill) -> mysql::Result<u32> {
        let mut conn = self.pool.get_conn()?;
        
        conn.exec_drop(
            r"INSERT INTO bills (description, amount, bill_type, date)
              VALUES (:description, :amount, :bill_type, :date)",
            params! {
                "description" => &bill.description,
                "amount" => bill.amount,
                "bill_type" => &bill.bill_type,
                "date" => &bill.date,
            },
        )?;
        
        let last_id: u32 = conn.last_insert_id() as u32;
        Ok(last_id)
    }
    
    pub fn get_all_bills(&self) -> mysql::Result<Vec<Bill>> {
        let mut conn = self.pool.get_conn()?;
        
        let bills = conn.query_map(
            "SELECT id, description, amount, bill_type, date FROM bills ORDER BY date DESC",
            |(id, description, amount, bill_type, date): (u32, String, mysql::Value, String, mysql::Value)| {
                Bill {
                    id: Some(id),
                    description,
                    amount: value_to_f64(amount),
                    bill_type,
                    date: date_to_string(date),
                }
            },
        )?;
        
        Ok(bills)
    }
    
    pub fn get_bill_by_id(&self, bill_id: u32) -> mysql::Result<Option<Bill>> {
        let mut conn = self.pool.get_conn()?;
        
        let bill = conn.exec_map(
            "SELECT id, description, amount, bill_type, date FROM bills WHERE id = :id",
            params! { "id" => bill_id },
            |(id, description, amount, bill_type, date): (u32, String, mysql::Value, String, mysql::Value)| {
                Bill {
                    id: Some(id),
                    description,
                    amount: value_to_f64(amount),
                    bill_type,
                    date: date_to_string(date),
                }
            },
        )?;
        
        Ok(bill.into_iter().next())
    }
    
    pub fn update_bill(&self, bill: &Bill) -> mysql::Result<bool> {
        let mut conn = self.pool.get_conn()?;
        
        conn.exec_drop(
            r"UPDATE bills SET description = :description, amount = :amount, 
              bill_type = :bill_type, date = :date WHERE id = :id",
            params! {
                "id" => bill.id,
                "description" => &bill.description,
                "amount" => bill.amount,
                "bill_type" => &bill.bill_type,
                "date" => &bill.date,
            },
        )?;
        
        let affected_rows = conn.affected_rows();
        Ok(affected_rows > 0)
    }
    
    pub fn delete_bill(&self, bill_id: u32) -> mysql::Result<bool> {
        let mut conn = self.pool.get_conn()?;
        
        conn.exec_drop(
            "DELETE FROM bills WHERE id = :id",
            params! { "id" => bill_id },
        )?;
        
        let affected_rows = conn.affected_rows();
        Ok(affected_rows > 0)
    }
    
    pub fn get_summary(&self) -> mysql::Result<Summary> {
        let mut conn = self.pool.get_conn()?;
        
        let summary: Option<(mysql::Value, mysql::Value, mysql::Value)> = conn.query_first(
            "SELECT 
                COALESCE(SUM(CASE WHEN bill_type = 'income' THEN amount ELSE 0 END), 0) as income,
                COALESCE(SUM(CASE WHEN bill_type = 'expense' THEN amount ELSE 0 END), 0) as expense,
                COALESCE(SUM(CASE WHEN bill_type = 'income' THEN amount ELSE -amount END), 0) as balance
            FROM bills"
        )?;
        
        let (income, expense, balance) = if let Some((inc, exp, bal)) = summary {
            (
                value_to_f64(inc),
                value_to_f64(exp),
                value_to_f64(bal)
            )
        } else {
            (0.0, 0.0, 0.0)
        };
        
        Ok(Summary { income, expense, balance })
    }
}