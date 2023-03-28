use serde::{
    Serialize,
    Deserialize,
};
use sqlx::FromRow;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct Book{
    pub title:String,
    pub author: String,
    pub isbn: String,
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct PutBook{
    pub title:String,
    pub author: String,
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct User{
    pub username:String,
    pub email:String,
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct CreateUser{
    pub username:String,
    pub email:String,
    pub pass_word:String
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct LoginUser{
    pub email:String,
    pub pass_word:String
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct UpdateUser{
    pub username:String,
    pub pass_word:String
}
