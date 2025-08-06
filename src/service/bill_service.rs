use crate::dao::bill_dao::BillDAO;
use crate::model::bill::{Bill, Summary};

pub struct BillService {
    dao: BillDAO,
}

impl BillService {
    pub fn new(dao: BillDAO) -> Self {
        BillService { dao }
    }
    
    pub fn add_bill(&self, bill: &Bill) -> Result<u32, Box<dyn std::error::Error>> {
        Ok(self.dao.add_bill(bill)?)
    }
    
    pub fn get_all_bills(&self) -> Result<Vec<Bill>, Box<dyn std::error::Error>> {
        Ok(self.dao.get_all_bills()?)
    }
    
    pub fn get_bill_by_id(&self, bill_id: u32) -> Result<Option<Bill>, Box<dyn std::error::Error>> {
        Ok(self.dao.get_bill_by_id(bill_id)?)
    }
    
    pub fn update_bill(&self, bill: &Bill) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.dao.update_bill(bill)?)
    }
    
    pub fn delete_bill(&self, bill_id: u32) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.dao.delete_bill(bill_id)?)
    }
    
    pub fn get_summary(&self) -> Result<Summary, Box<dyn std::error::Error>> {
        Ok(self.dao.get_summary()?)
    }
}