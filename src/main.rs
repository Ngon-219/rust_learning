use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::PgPool;
mod  handlers;
mod model;
mod repository;
use model::NewUser;
use sqlx::postgres::PgPoolOptions;
use repository::to_do_list_repo::ToDoListRepository;

#[actix_web::main] 
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    println!("✅ Kết nối thành công đến database!");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/create-to-do-list", web::post().to(handlers::create_to_do_list_handler))
            .route("/get-to-do-list", web::get().to(handlers::get_all_to_do_list_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
