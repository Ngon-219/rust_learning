use actix_web::{web, HttpResponse};
use crate::model::toDoList;
use sqlx::PgPool;
use actix_web::{Responder};
use crate::repository::to_do_list_repo::ToDoListRepository;

pub async fn create_to_do_list (pool: &PgPool, to_do_list: toDoList) -> Result<(), sqlx::Error> {
    let repo = ToDoListRepository::new(pool.clone());
    repo.create_to_do_list(to_do_list).await?;
    Ok(())
}

pub async fn create_to_do_list_handler(pool: web::Data<PgPool>, to_do_list: web::Json<toDoList>) -> impl Responder {
    match create_to_do_list(&pool, to_do_list.into_inner()).await {
        Ok(_) => HttpResponse::Created().body("To Do List created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}

pub async fn get_all_to_do_list (pool: &PgPool) -> Result<Vec<toDoList>, sqlx::Error> {
    let repo = ToDoListRepository::new(pool.clone());
    let to_do_lists = repo.get_all_to_do_list().await?;
    Ok(to_do_lists)
}

pub async fn get_all_to_do_list_handler(pool: web::Data<PgPool>) -> impl Responder {
    match get_all_to_do_list(&pool).await {
        Ok(to_do_lists) => HttpResponse::Ok().json(to_do_lists),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch to-do lists"),
    }
}