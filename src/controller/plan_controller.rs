use actix_web::{web, HttpResponse, Result};
use crate::{service::plan_service::PlanService, Plan};

pub async fn add_plan_handler(plan: web::Json<Plan>, data: web::Data<PlanService>) -> Result<HttpResponse, actix_web::Error> {
    match data.add_plan(&plan) {
        Ok(id) => Ok(HttpResponse::Ok().json(serde_json::json!({ "id": id }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

pub async fn get_all_plans_handler(data: web::Data<PlanService>) -> Result<HttpResponse, actix_web::Error> {
    match data.get_all_plans() {
        Ok(plans) => Ok(HttpResponse::Ok().json(plans)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

pub async fn get_plan_handler(path: web::Path<u32>, data: web::Data<PlanService>) -> Result<HttpResponse, actix_web::Error> {
    let plan_id = path.into_inner();
    match data.get_plan_by_id(plan_id) {
        Ok(Some(plan)) => Ok(HttpResponse::Ok().json(plan)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({ "error": "Plan not found" }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

pub async fn update_plan_handler(
    path: web::Path<u32>,
    plan: web::Json<Plan>,
    data: web::Data<PlanService>,
) -> Result<HttpResponse, actix_web::Error> {
    let plan_id = path.into_inner();
    let mut plan = plan.into_inner();
    plan.id = Some(plan_id);
    
    match data.update_plan(&plan) {
        Ok(true) => Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Plan updated successfully" }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({ "error": "Plan not found" }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

pub async fn delete_plan_handler(path: web::Path<u32>, data: web::Data<PlanService>) -> Result<HttpResponse, actix_web::Error> {
    let plan_id = path.into_inner();
    match data.delete_plan(plan_id) {
        Ok(true) => Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Plan deleted successfully" }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({ "error": "Plan not found" }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))),
    }
}