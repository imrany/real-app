use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[path="/func/sub.rs"]
mod sub;
use sub::{
    get_books
};


#[tokio::main]
async fn main()->std::io::Result<()>{
    std::env::set_var("RUST_LOG","debug");
    std::env::set_var("RUST_BACKTRACE","1");
    env_logger::init();

    HttpServer::new(move ||{
        let logger=Logger::default();
        App::new()
        .wrap(logger)
        .service(get_books)
    })
    .bind(("127.0.0.1",8000))?
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