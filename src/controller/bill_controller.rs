use actix_web::{web, HttpResponse, Result};
use crate::{service::bill_service::BillService, Bill};

pub async fn add_bill_handler(bill: web::Json<Bill>, data: web::Data<BillService>) -> Result<HttpResponse, actix_web::Error> {
    match data.add_bill(&bill) {
        Ok(id) => Ok(HttpResponse::Ok().json(serde_json::json!({ "id": id }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

pub async fn get_all_bills_handler(data: web::Data<BillService>) -> Result<HttpResponse, actix_web::Error> {
    match data.get_all_bills() {
        Ok(bills) => Ok(HttpResponse::Ok().json(bills)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

pub async fn get_bill_handler(path: web::Path<u32>, data: web::Data<BillService>) -> Result<HttpResponse, actix_web::Error> {
    let bill_id = path.into_inner();
    match data.get_bill_by_id(bill_id) {
        Ok(Some(bill)) => Ok(HttpResponse::Ok().json(bill)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({ "error": "Bill not found" }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

pub async fn update_bill_handler(
    path: web::Path<u32>,
    bill: web::Json<Bill>,
    data: web::Data<BillService>,
) -> Result<HttpResponse, actix_web::Error> {
    let bill_id = path.into_inner();
    let mut bill = bill.into_inner();
    bill.id = Some(bill_id);
    
    match data.update_bill(&bill) {
        Ok(true) => Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Bill updated successfully" }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({ "error": "Bill not found" }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

pub async fn delete_bill_handler(path: web::Path<u32>, data: web::Data<BillService>) -> Result<HttpResponse, actix_web::Error> {
    let bill_id = path.into_inner();
    match data.delete_bill(bill_id) {
        Ok(true) => Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Bill deleted successfully" }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({ "error": "Bill not found" }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

pub async fn get_summary_handler(data: web::Data<BillService>) -> Result<HttpResponse, actix_web::Error> {
    match data.get_summary() {
        Ok(summary) => Ok(HttpResponse::Ok().json(summary)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}