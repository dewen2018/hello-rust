use actix_web::{web, HttpResponse, Responder};
use crate::service::schedule_service::ScheduleService;
use crate::model::schedule::Schedule;

pub async fn add_schedule_handler(
    schedule_service: web::Data<ScheduleService>,
    schedule: web::Json<Schedule>,
) -> impl Responder {
    match schedule_service.add_schedule(schedule.into_inner()).await {
        Ok(schedule) => HttpResponse::Created().json(schedule),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_all_schedules_handler(
    schedule_service: web::Data<ScheduleService>,
) -> impl Responder {
    match schedule_service.get_all_schedules().await {
        Ok(schedules) => HttpResponse::Ok().json(schedules),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_schedule_handler(
    schedule_service: web::Data<ScheduleService>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match schedule_service.get_schedule_by_id(id).await {
        Ok(schedule) => HttpResponse::Ok().json(schedule),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_schedule_handler(
    schedule_service: web::Data<ScheduleService>,
    path: web::Path<i32>,
    schedule: web::Json<Schedule>,
) -> impl Responder {
    let id = path.into_inner();
    match schedule_service.update_schedule(id, schedule.into_inner()).await {
        Ok(schedule) => HttpResponse::Ok().json(schedule),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_schedule_handler(
    schedule_service: web::Data<ScheduleService>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match schedule_service.delete_schedule(id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}