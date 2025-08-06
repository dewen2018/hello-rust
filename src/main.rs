use actix_web::{web, App, HttpServer, middleware::Logger};
use mysql::*;
use dao::bill_dao::BillDAO;
use dao::plan_dao::PlanDAO;
// 添加课程表DAO
use dao::schedule_dao::ScheduleDAO;
use service::bill_service::BillService;
use service::plan_service::PlanService;
// 添加课程表服务
use service::schedule_service::ScheduleService;
use controller::bill_controller::*;
use controller::plan_controller::*;
// 添加课程表控制器
use controller::schedule_controller::*;
use model::bill::{Bill, Summary};
use model::plan::Plan;
// 添加课程表模型
use model::schedule::Schedule;

mod controller {
    pub mod bill_controller;
    pub mod plan_controller;
    // 添加课程表控制器模块
    pub mod schedule_controller;
}
mod service {
    pub mod bill_service;
    pub mod plan_service;
    // 添加课程表服务模块
    pub mod schedule_service;
}
mod dao {
    pub mod bill_dao;
    pub mod plan_dao;
    // 添加课程表DAO模块
    pub mod schedule_dao;
}
mod model{
    pub mod bill;
    pub mod plan;
    // 添加课程表模型模块
    pub mod schedule;
}
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 数据库连接URL，请根据实际情况修改
    let url = "mysql://root:root@localhost:3306/dewen_0330";
    
    let pool = Pool::new(url).expect("Failed to create database pool");
    let bill_dao = BillDAO::new(pool.clone());

    bill_dao.init_database().expect("Failed to initialize database");
    let bill_service = web::Data::new(BillService::new(bill_dao));

    let plan_dao = PlanDAO::new(pool.clone());
    plan_dao.init_plan_table().expect("Failed to initialize plan table");
    let plan_service = web::Data::new(PlanService::new(plan_dao));
    
    // 初始化课程表DAO和服务
    let schedule_dao = ScheduleDAO::new(pool.clone());
    schedule_dao.init_schedule_table().expect("Failed to initialize schedule table");
    let schedule_service = web::Data::new(ScheduleService::new(schedule_dao));
    
    println!("Starting server at http://localhost:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(bill_service.clone())
            .app_data(plan_service.clone())
            .app_data(schedule_service.clone())
            .wrap(Logger::default())
            .route("/bills", web::post().to(add_bill_handler))
            .route("/bills", web::get().to(get_all_bills_handler))
            .route("/bills/{id}", web::get().to(get_bill_handler))
            .route("/bills/{id}", web::put().to(update_bill_handler))
            .route("/bills/{id}", web::delete().to(delete_bill_handler))
            .route("/summary", web::get().to(get_summary_handler))
            .route("/plans", web::post().to(add_plan_handler))
            .route("/plans", web::get().to(get_all_plans_handler))
            .route("/plans/{id}", web::get().to(get_plan_handler))
            .route("/plans/{id}", web::put().to(update_plan_handler))
            .route("/plans/{id}", web::delete().to(delete_plan_handler))
            // 添加课程表相关路由
            .route("/schedules", web::post().to(add_schedule_handler))
            .route("/schedules", web::get().to(get_all_schedules_handler))
            .route("/schedules/{id}", web::get().to(get_schedule_handler))
            .route("/schedules/{id}", web::put().to(update_schedule_handler))
            .route("/schedules/{id}", web::delete().to(delete_schedule_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}