use actix_web::{web::Data, HttpServer, App, middleware::Logger};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;

pub struct AppState{
    db:Pool<Postgres>
} 

#[path="func/routes.rs"]
mod routes;
use routes::{
    get_books,
    first_page,
    add_book,
    update_book,
    get_book,
    delete_book,
};

#[path="func/user_routes.rs"]
mod user_routes;
use user_routes::{
    register_user,
    login_user,
    get_user_info,
    get_users,
    update_user_details,
    delete_user,
};

#[tokio::main]
async fn main()->std::io::Result<()>{
    dotenv().ok();
    // std::env::set_var("RUST_LOG","debug");
    std::env::set_var("RUST_LOG","actix_web=info");
    std::env::set_var("RUST_BACKTRACE","1");
    env_logger::init();

    let database_url=std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool=PgPoolOptions::new()
     .max_connections(5)
     .connect(&database_url)
     .await
     .expect("Error build a postgres connection pool");
    // let server_host=std::env::var("POSTGRES_HOST").expect("SERVER_HOST is must be set");
    let server_host:&str="127.0.0.1";

    HttpServer::new(move ||{
        let logger=Logger::default();
        App::new()
        .wrap(logger)
        .app_data(Data::new(AppState {db:pool.clone()}))
        .service(get_books)
        .service(first_page)
        .service(add_book)
        .service(get_book)
        .service(update_book)
        .service(delete_book)
        // user routes
        .service(get_users)
        .service(get_user_info)
        .service(register_user)
        .service(login_user)
        .service(update_user_details)
        .service(delete_user)
    })
    .bind((server_host,8000))?
    .run()
    .await
}