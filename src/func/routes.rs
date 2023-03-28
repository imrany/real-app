use actix_web::{
    get,
    post,
    web::{Json, Data},
    // error::ResponseError,
    // http::{header::ContentType, StatusCode},
    Responder,
    HttpResponse,
};
mod models;
use models::Book;
use sqlx::{FromRow, self};
// use serde_json::Json;
// use crate::AppState;
use crate::AppState;

// use serde::{Serialize,Deserialize};
// use derive_more::{Display};
#[allow(non_snake_case)]
#[get("/")]
pub async fn first_page()->Json<String>{
    return Json("welcome".to_string());
}

#[get("/books")]
pub async fn get_books(state:Data<AppState>)->impl Responder{
    // return Json("hello world".to_string());
    match sqlx::query_as::<_, Book>("SELECT title, author, isbn FROM book")
     .fetch_all(&state.db)
     .await
     {
        Ok(books)=>HttpResponse::Ok().json(books),
        Err(_)=>HttpResponse::NotFound().json("No books found")
     }
} 

#[post("/books/add")]
pub async fn add_book(
    state:Data<AppState>,
    body:Json<Book>,
    // data:web::Data<AppState>,
)-> impl Responder{
    match sqlx::query_as::<_,Book>(
        "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(body.title.to_string())
    .bind(body.author.to_string())
    .bind(body.isbn.to_string())
    .fetch_one(&state.db)
    .await
    {
        Ok(book)=>HttpResponse::Ok().json(book),
        Err(_)=>HttpResponse::InternalServerError().json("Fail to create book")
    }

}