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

use argon2::{
    password_hash::{rand_core::OsRng,PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

mod models;
use models::{
    User,
    LoginUser,
    CreateUser,
    UpdateUser,
};

#[get("/users")]
pub async fn get_users(
    state:Data<AppState>,
)->impl Responder{
    match sqlx::query_as::<_,User>(
        "SELECT username,email FROM users"
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(users)=>HttpResponse::Ok().json(users),
        Err(err)=>HttpResponse::BadRequest().json(["Users not found",&err.to_string()])
    }
}

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

#[post("/auth/register")]
pub async fn register_user(
    body:Json<CreateUser>,
    state:Data<AppState>,
)->impl Responder{
    // let exists=sqlx::query("SELECT 1 FROM users WHERE email=$1")
    // .bind(body.email.to_string())
    // .fetch_one(&state.db)
    // .await;

    // {
    //     Ok(_)=>HttpResponse::Conflict().json("User with that email already exists"),
    //     Err(err)=>HttpResponse::Conflict().json(&err.to_string())
    // }

    let salt=SaltString::generate(&mut OsRng);
    let argon2=Argon2::default();
    
    let hashed_password=argon2.hash_password(body.pass_word.as_bytes(),&salt)
    .expect("Error while hashing password")
    .to_string();

    match sqlx::query_as::<_,User>(
        "INSERT INTO users (username, email ,pass_word) VALUES ($1, $2 ,$3) RETURNING username, email"
    )
    .bind(body.username.to_string())
    .bind(body.email.to_string().to_lowercase())
    .bind(hashed_password)
    .fetch_one(&state.db)
    .await
    {
        Ok(created_user)=>HttpResponse::Ok().json(created_user),
        Err(err)=>HttpResponse::InternalServerError().json(["Failed to register user",&err.to_string()])
    }
}

#[post("/auth/login")]
pub async fn login_user(
    body:Json<LoginUser>,
    state:Data<AppState>,
)->impl Responder{
    let query_result= sqlx::query_as::<_,CreateUser>(
        "SELECT username, email, pass_word FROM users WHERE email=$1"
    )
    .bind(body.email.to_string())
    .bind(body.pass_word.to_string())
    .fetch_one(&state.db)
    .await;

    let is_valid=query_result.to_owned().map_or(false, |CreateUser|{
        let parsed_hash=PasswordHash::new(&CreateUser.pass_word).unwrap();
        Argon2::default().verify_password(body.pass_word.as_bytes(),&parsed_hash)
        .map_or(false,|_| true)
    });
    if !is_valid{
        return HttpResponse::BadRequest().json("Invalid email or password");
    }
    let user=query_result.unwrap();
    HttpResponse::Ok().json(query_result)
       

    // {
    //     Ok(login_user)=>HttpResponse::Ok().json(login_user),
    //      // let parsed_hash=PasswordHash::new(login_user.pass_word.to_string());
    //         // Argon2::default().verify_password(login_user.pass_word.as_bytes(), &parsed_hash).is_ok()
            
    //         // if parsed_hash{
    //         // }else{
    //         //     HttpResponse::NotFound().json("Not authorized")
    //         // }
    //     Err(err)=>HttpResponse::InternalServerError().json(["Failed to login",&err.to_string()])
    // }
}

#[put("/users/update/{email}")]
pub async fn update_user_details(
    body:Json<UpdateUser>,
    state:Data<AppState>,
    path:Path<String>,
)->impl Responder{
    let email=path.into_inner();
    let query="UPDATE users SET username=$1, pass_word=$2 WHERE email=$3 RETURNING username, pass_word";
    match sqlx::query_as::<_,UpdateUser>(query)
    .bind(body.username.to_string())
    .bind(body.pass_word.to_string())
    .bind(email)
    .fetch_one(&state.db)
    .await
    {
        Ok(user_detail)=>HttpResponse::Ok().json(user_detail),
        Err(err)=>HttpResponse::NotFound().json(["Failed to update, User not found",&err.to_string()])
    }
}

#[delete("/users/delete/{email}")]
pub async fn delete_user(
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