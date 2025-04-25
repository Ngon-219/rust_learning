use crate::model::toDoList;
use sqlx::PgPool;
use actix_web::{Responder};
use crate::repository::to_do_list_repo::ToDoListRepository;
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct TaskId {
    pub task_id: String,
}

pub async fn create_to_do_list_handler(pool: web::Data<PgPool>, to_do_list: web::Json<toDoList>) -> impl Responder {
    let repo = ToDoListRepository::new(pool.get_ref().clone());
    match repo.create_to_do_list(to_do_list.into_inner()).await {
        Ok(_) => HttpResponse::Created().body("To Do List created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create to-do list"),
    }
}

pub async fn get_all_to_do_list_handler(pool: web::Data<PgPool>) -> impl Responder {
    let repo = ToDoListRepository::new(pool.get_ref().clone());
    match repo.get_all_to_do_list().await {
        Ok(to_do_lists) => HttpResponse::Ok().json(to_do_lists),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch to-do lists"),
    }
}

pub async fn update_to_do_list_handler(pool: web::Data<PgPool>, to_do_list: web::Json<toDoList>) -> impl Responder {
    let repo = ToDoListRepository::new(pool.get_ref().clone());
    match repo.update_to_do_list(to_do_list.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("To Do List updated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update to-do list"),
    }
}

pub async fn delete_to_do_list(pool: web::Data<PgPool>, task_id: web::Path<String>) -> impl Responder {
    let repo = ToDoListRepository::new(pool.get_ref().clone());
    match repo.delete_to_do_list(&task_id).await {
        Ok(_) => HttpResponse::Ok().body("To Do List deleted successfully"),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("To do list not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete to-do list"),
    }
}

pub async fn update_to_do_status_handlers(pool: web::Data<PgPool>, task_id: web::Json<TaskId>) -> impl Responder {
    let repo = ToDoListRepository::new(pool.get_ref().clone());
    let task_id = &task_id.task_id;
    
    match repo.update_to_do_list_status(task_id).await {
        Ok(_) => HttpResponse::Ok().body("To Do List status updated successfully"),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("To do list not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update to-do list status"),
    }
}

pub async fn get_to_do_list_id_handlers(pool: web::Data<PgPool>, task_id: web::Path<String>) -> impl Responder {
    let repo = ToDoListRepository::new(pool.get_ref().clone());
    match repo.get_to_do_list_by_id(&task_id).await {
        Ok(to_do_list) => HttpResponse::Ok().json(to_do_list),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("To do list not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch to-do list"),
    }
}

