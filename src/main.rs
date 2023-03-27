use std::error::Error;
use sqlx::Row;

struct Book{
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool)->Result<(), Box<dyn Error>>{
    let query="INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn update(book:&Book, isbn: &str, pool:&sqlx::PgPool)->Result<(), Box<dyn Error>>{
    let query="UPDATE book SET title=$1, author=$2 WHERE isbn = $3";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main()->Result<(), Box<dyn Error>>{
    let url="postgres://admin:password123@0.0.0.0:6500/heallth";
    let pool=sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

     let book=Book{
        title:"Imran's book".to_string(),
        author:"Imran King".to_string(),
        isbn:"978-0-385-00751-2".to_string(),
     };

    //  create(&book,&pool).await?; 
     update(&book,&book.isbn,&pool).await?;

    Ok(())
}