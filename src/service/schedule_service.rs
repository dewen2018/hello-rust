use crate::dao::schedule_dao::ScheduleDAO;
use crate::model::schedule::Schedule;
use std::sync::Arc;

pub struct ScheduleService {
    schedule_dao: Arc<ScheduleDAO>,
}

impl ScheduleService {
    pub fn new(schedule_dao: ScheduleDAO) -> Self {
        Self {
            schedule_dao: Arc::new(schedule_dao),
        }
    }

    pub async fn add_schedule(&self, schedule: Schedule) -> Result<Schedule, Box<dyn std::error::Error>> {
        self.schedule_dao.create_schedule(schedule).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    pub async fn get_all_schedules(&self) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        self.schedule_dao.get_all_schedules().await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    pub async fn get_schedule_by_id(&self, id: i32) -> Result<Schedule, Box<dyn std::error::Error>> {
        self.schedule_dao.get_schedule_by_id(id).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    pub async fn update_schedule(&self, id: i32, schedule: Schedule) -> Result<Schedule, Box<dyn std::error::Error>> {
        self.schedule_dao.update_schedule(id, schedule).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    pub async fn delete_schedule(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.schedule_dao.delete_schedule(id).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}