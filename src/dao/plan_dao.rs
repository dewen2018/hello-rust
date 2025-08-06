use mysql::*;
use mysql::prelude::*;
use crate::model::plan::Plan;
use crate::utils::date_to_string;

pub struct PlanDAO {
    pool: Pool,
}

impl PlanDAO {
    pub fn new(pool: Pool) -> Self {
        PlanDAO { pool }
    }
    
    pub fn init_plan_table(&self) -> mysql::Result<()> {
        let mut conn = self.pool.get_conn()?;
        
        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS plans (
                id INT AUTO_INCREMENT PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                description TEXT NOT NULL,
                status ENUM('pending', 'in_progress', 'completed') NOT NULL DEFAULT 'pending',
                due_date DATE NOT NULL
            )"
        )?;
        
        println!("Plan table initialized successfully!");
        Ok(())
    }
    
    pub fn add_plan(&self, plan: &Plan) -> mysql::Result<u32> {
        let mut conn = self.pool.get_conn()?;
        
        conn.exec_drop(
            r"INSERT INTO plans (title, description, status, due_date)
              VALUES (:title, :description, :status, :due_date)",
            params! {
                "title" => &plan.title,
                "description" => &plan.description,
                "status" => &plan.status,
                "due_date" => &plan.due_date,
            },
        )?;
        
        let last_id: u32 = conn.last_insert_id() as u32;
        Ok(last_id)
    }
    
    pub fn get_all_plans(&self) -> mysql::Result<Vec<Plan>> {
        let mut conn = self.pool.get_conn()?;
        
        let plans = conn.query_map(
            "SELECT id, title, description, status, due_date FROM plans ORDER BY due_date ASC",
            |(id, title, description, status, due_date): (u32, String, String, String, mysql::Value)| {
                Plan {
                    id: Some(id),
                    title,
                    description,
                    status,
                    due_date: date_to_string(due_date),
                }
            },
        )?;
        
        Ok(plans)
    }
    
    pub fn get_plan_by_id(&self, plan_id: u32) -> mysql::Result<Option<Plan>> {
        let mut conn = self.pool.get_conn()?;
        
        let plan = conn.exec_map(
            "SELECT id, title, description, status, due_date FROM plans WHERE id = :id",
            params! { "id" => plan_id },
            |(id, title, description, status, due_date): (u32, String, String, String, mysql::Value)| {
                Plan {
                    id: Some(id),
                    title,
                    description,
                    status,
                    due_date: date_to_string(due_date),
                }
            },
        )?;
        
        Ok(plan.into_iter().next())
    }
    
    pub fn update_plan(&self, plan: &Plan) -> mysql::Result<bool> {
        let mut conn = self.pool.get_conn()?;
        
        conn.exec_drop(
            r"UPDATE plans SET title = :title, description = :description, 
              status = :status, due_date = :due_date WHERE id = :id",
            params! {
                "id" => plan.id,
                "title" => &plan.title,
                "description" => &plan.description,
                "status" => &plan.status,
                "due_date" => &plan.due_date,
            },
        )?;
        
        let affected_rows = conn.affected_rows();
        Ok(affected_rows > 0)
    }
    
    pub fn delete_plan(&self, plan_id: u32) -> mysql::Result<bool> {
        let mut conn = self.pool.get_conn()?;
        
        conn.exec_drop(
            "DELETE FROM plans WHERE id = :id",
            params! { "id" => plan_id },
        )?;
        
        let affected_rows = conn.affected_rows();
        Ok(affected_rows > 0)
    }
}