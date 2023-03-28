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
};


#[tokio::main]
async fn main()->std::io::Result<()>{
    dotenv().ok();
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
    })
    .bind((server_host,8000))?
    .run()
    .await
}

// use std::error::Error;
// use sqlx::Row;

// struct Book{
//     pub title: String,
//     pub author: String,
//     pub isbn: String,
// }

// async fn create(book: &Book, pool: &sqlx::PgPool)->Result<(), Box<dyn Error>>{
//     let query="INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

//     sqlx::query(query)
//         .bind(&book.title)
//         .bind(&book.author)
//         .bind(&book.isbn)
//         .execute(pool)
//         .await?;

//     Ok(())
// }

// async fn update(book:&Book, isbn: &str, pool:&sqlx::PgPool)->Result<(), Box<dyn Error>>{
//     let query="UPDATE book SET title=$1, author=$2 WHERE isbn = $3";
//     sqlx::query(query)
//         .bind(&book.title)
//         .bind(&book.author)
//         .bind(&book.isbn)
//         .execute(pool)
//         .await?;

//     Ok(())
// }

// async fn read(conn:&sqlx::PgPool)->Result<Book, Box<dyn Error>>{
//     let q="SELECT title, author, isbn FROM book";
//     let query=sqlx::query(q);

//     let row=query.fetch_one(conn).await?;

//     let book=Book{
//         title: row.get("title"),
//         author: row.get("author"),
//         isbn: row.get("isbn"),
//     };

//     Ok(book)
// }

// #[tokio::main]
// async fn main()->Result<(), Box<dyn Error>>{
//     let url="postgres://admin:password123@0.0.0.0:6500/heallth";
//     let pool=sqlx::postgres::PgPool::connect(url).await?;

//     sqlx::migrate!("./migrations").run(&pool).await?;

//      let book=Book{
//         title:"Imran's book".to_string(),
//         author:"Imran King".to_string(),
//         isbn:"978-0-385-00751-2".to_string(),
//      };

//     // create(&book,&pool).await?; 
//     // update(&book,&book.isbn,&pool).await?;
//     // read(&pool).await?;

//     Ok(())
// }