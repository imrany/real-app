use serde::{
    Serialize,
    Deserialize,
};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Book{
    pub title:String,
    pub author: String,
    pub isbn: String,
}