use actix_web::{
    get,
    web::Path,
    web::Json,
    error::ResponseError,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize,Deserialize};
// use derive_more::{Display};

#[get("/")]
pub async fn first_page()->Json<String>{
    return Json("welcome".to_string());
}

#[get("/books")]
pub async fn get_books()->Json<String>{
    return Json("hello world".to_string());
} 