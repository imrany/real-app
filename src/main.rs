use actix_web::{
    web, App, HttpServer
};

#[path="func/controller.rs"]
mod controller;
use controller::{post_gcd, get_index};

fn main() {
    let server= HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(get_index))
        .route("/gcd", web::post().to(post_gcd))
    });
    println!("Servering on port 8080");
    server.bind("127.0.0.1:8080").expect("error binding server to address")
     .run().expect("error running server");
}

