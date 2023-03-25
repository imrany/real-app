use actix_web::middleware::Logger;
use actix_web::{
    get,
    App,
    HttpResponse,
    HttpServer,
    Responder
};

#[get("/api/healthchecker")]
 fn health_checker_handle()->impl Responder{
    const MESSAGE: &str= "JWT Authentication in Rust using Actix-web, Postgres, and SQLX";

    HttpResponse::Ok().json(serde_json::json!({"status":"success","message":MESSAGE}))
}

// #[actix_web::main]
fn main()->std::io::Result<()>{
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG","actix-web=info");
    }
    let port=std::env::var("PORT")|8080;

    env_logger::init();
    println!("Server running on port {}",port);
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .service(health_checker_handle)
    })
    .bind(("127.0.0.1",port))?
    .run()
}