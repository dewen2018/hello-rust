use crate::dao::plan_dao::PlanDAO;
use crate::model::plan::Plan;

pub struct PlanService {
    dao: PlanDAO,
}

// Plan相关的服务操作
impl PlanService {
    pub fn new(dao: PlanDAO) -> Self {
        PlanService { dao }
    }
    
    pub fn add_plan(&self, plan: &Plan) -> Result<u32, Box<dyn std::error::Error>> {
        Ok(self.dao.add_plan(plan)?)
    }
    
    pub fn get_all_plans(&self) -> Result<Vec<Plan>, Box<dyn std::error::Error>> {
        Ok(self.dao.get_all_plans()?)
    }
    
    pub fn get_plan_by_id(&self, plan_id: u32) -> Result<Option<Plan>, Box<dyn std::error::Error>> {
        Ok(self.dao.get_plan_by_id(plan_id)?)
    }
    
    pub fn update_plan(&self, plan: &Plan) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.dao.update_plan(plan)?)
    }
    
    pub fn delete_plan(&self, plan_id: u32) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.dao.delete_plan(plan_id)?)
    }
}