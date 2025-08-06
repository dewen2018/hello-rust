use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub id: Option<i32>,
    pub course_name: String,
    pub teacher: String,
    pub classroom: String,
    pub day_of_week: i32, // 1-7 表示周一到周日
    pub start_time: String, // 格式: "HH:MM:SS"
    pub end_time: String,   // 格式: "HH:MM:SS"
}