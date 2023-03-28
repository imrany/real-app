use actix_web::{
    get,
    post,
    put,
    delete,
    web::{Path,Json,Data},
    HttpResponse,
    Responder,
};
use sqlx::{self};
use crate::AppState;
mod models;
use models::{
    User,
    LoginUser,
    CreateUser,
    UpdateUser,
};

#[get("/users/{email}")]
pub async  fn get_user_info(
    path:Path<String>,
    state:Data<AppState>,
)->impl Responder{
    let email=path.into_inner();
    match sqlx::query_as::<_,User>(
        "SELECT username, email FROM users WHERE email=$1"
    )
    .bind(email)
    .fetch_one(&state.db)
    .await
    {
        Ok(user_info)=>HttpResponse::Ok().json(user_info),
        Err(err)=>HttpResponse::NotFound().json(["User not found",&err.to_string()])
    }
}

#[post("/users/add")]
pub async fn add_user(
    body:Json<CreateUser>,
    state:Data<AppState>,
)->impl Responder{
    match sqlx::query_as::<_,User>(
        "INSERT INTO users (username, email ,pass_word) VALUES ($1, $2 ,$3) RETURNING username, email"
    )
    .bind(body.username.to_string())
    .bind(body.email.to_string())
    .bind(body.pass_word.to_string())
    .fetch_one(&state.db)
    .await
    {
        Ok(created_user)=>HttpResponse::Ok().json(created_user),
        Err(err)=>HttpResponse::InternalServerError().json(["Failed to create user",&err.to_string()])
    }
}

#[put("/users/update/{email}")]
pub async update_user_details(
    body:Json<UpdateUser>,
    state:Data<AppState>,
    path:Path<String>,
)->impl Responder{
    let email=path.into_inner();
    match sqlx::query_as::<_,UpdateUser>(
        "UPDATE users SET username=$1, pass_word=$2 WHERE email=$1 RETURNING *"
    )
    .bind(body.username.to_string())
    .bind(body.pass_word.to_string())
    .bind(email)
    .fetch_one(&state.db)
    .await
    {
        Ok(user_detail)=>HttpResponse::Ok().json(user_detail),
        Err(err)=>HttpResponse::NotFound().json(["User with that email, wasn't found",&err.to_string()])
    }
}

#[delete("/users/delete/{email}")]
pub async delete_user(
    state:Data<AppState>,
    path:Path<String>,
)->impl Responder{
    let email=path.into_inner();
    match sqlx::query_as::<_,CreateUser>(
        "DELETE FROM users WHERE email=$1 RETURNING *"
    )
    .bind(email)
    .fetch_one(&state.db)
    .await
    {
        Ok(deleted_user)=>HttpResponse::Ok().json(deleted_user),
        Err(err)=>HttpResponse::NotFound().json(["Cannot delete, such email does not exist",&err.to_string()])
    }
}