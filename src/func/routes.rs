use actix_web::{
    get,
    post,
    put,
    delete,
    web::{Json, Data, Path},
    // error::ResponseError,
    // http::{header::ContentType, StatusCode},
    Responder,
    HttpResponse,
};
mod models;
use models::{
    Book,
    PutBook
};
use sqlx::{
    self,
};
// use serde_json::Json;
// use crate::AppState;
use crate::AppState;

// use serde::{Serialize,Deserialize};
#[allow(non_snake_case)]
#[get("/")]
pub async fn first_page()->Json<String>{
    return Json("welcome".to_string());
}

#[get("/books")]
pub async fn get_books(state:Data<AppState>)->impl Responder{
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
        Ok(add_book)=>HttpResponse::Ok().json(add_book),
        Err(_)=>HttpResponse::InternalServerError().json("Fail to create book")
    }

}

#[get("/books/get/{isbn}")]
pub async fn get_book(
    state:Data<AppState>,
    path:Path<String>,
)->impl Responder{
    let isbn:String=path.into_inner();
    match sqlx::query_as::<_, Book>(
        "SELECT title, author, isbn FROM book WHERE isbn=$1"
    )
    .bind(isbn)
    .fetch_one(&state.db)
    .await
    {
        Ok(book)=>HttpResponse::Ok().json(book),
        Err(_)=>HttpResponse::NotFound().json("Failed to fetch this book")
    }

}

#[put("/books/update/{isbn}")]
pub async fn update_book(
    body:Json<PutBook>, 
    path: Path<String>, 
    state:Data<AppState>
)->impl Responder{
    let isbn:String=path.into_inner();
    let query="UPDATE book SET title=$1, author=$2 WHERE isbn=$3 RETURNING title, author";
    match sqlx::query_as::<_, PutBook>(query)
        .bind(body.title.to_string())
        .bind(body.author.to_string())
        .bind(isbn)
        .fetch_one(&state.db)
        .await

    {
        Ok(updated_book)=>HttpResponse::Ok().json(updated_book),
        Err(err)=>HttpResponse::NotFound().json(["Such book was not found",&err.to_string()])
    }
}

#[delete("/books/delete/{isbn}")]
pub async fn delete_book(
    path:Path<String>,
    state:Data<AppState>,
)->impl Responder{
    let isbn=path.into_inner();
    match sqlx::query_as::<_, Book>(
        "DELETE FROM book WHERE isbn=$1 RETURNING *"
    ) 
    .bind(isbn)
    .fetch_one(&state.db)
    .await
    {
        Ok(deleted_book)=>HttpResponse::Ok().json(deleted_book),
        Err(err)=>HttpResponse::NotFound().json(["Fail to delete, Book not found",&err.to_string()])
    }
}
