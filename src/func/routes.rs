use actix_web::{
    get,
    post,
    web,
    web::Path,
    web::Json,
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    Responder,
    HttpResponse,
};
mod models;
use models::Book;
use sqlx::Row;
// use serde_json::Json;
// use crate::AppState;

use serde::{Serialize,Deserialize};
// use derive_more::{Display};
#[allow(non_snake_case)]
#[get("/")]
pub async fn first_page()->Json<String>{
    return Json("welcome".to_string());
}

#[get("/books")]
pub async fn get_books()->Json<String>{
    return Json("hello world".to_string());
} 

#[post("/books/add")]
pub async fn add_book(
    body:web::Json<Book>,
    // data:web::Data<AppState>,
)-> impl Responder{
    let db=std::env::var("DATABASE_URL");
    let add_a_book=sqlx::query_as!(Book,
        "INSERT INTO book(title, author, isbn) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.author.to_string(),
        body.isbn.to_string(),
    )
    .fetch_one(db)
    .await;

    match add_a_book{
        Ok(Book)=>{
            let response=Json(add_a_book);
            return HttpResponse::Ok().json(response);
        }
    }

}