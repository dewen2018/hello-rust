use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: Option<u32>,
    pub title: String,
    pub description: String,
    pub status: String, // "pending", "in_progress", "completed"
    pub due_date: String, // YYYY-MM-DD format
}

impl Plan {
    pub fn new(title: String, description: String, status: String, due_date: String) -> Self {
        Plan {
            id: None,
            title,
            description,
            status,
            due_date,
        }
    }
}