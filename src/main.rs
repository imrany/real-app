use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

fn main() {
    let server= HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(get_index))
        .route("/gcd", web::post().to(post_gcd))
    });
    printl!("Servering on port 8000");
    server.bind("127.0.0.1:8000").expect("error binding server to address")
     .run().expect("error running server");
}

//form struct / type or interface
#[derive(Deserialize)]
struct GCDParameters{
    n: u64,
    m:u64,
}

//get handler
fn get_index() -> HttpResponse{
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD Calculator</title>
        <form method="POST" action="/gcd">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button>Compute GCD</button>
        </form>
        "#,
    )
}

//post handler (form post)
fn post_gcd(form: web::Form<GCDParameters>) -> HttpResponse{
    if form.n==0||form.m==0 {
        return HttpResponse::BadRequest().content_type("text/html")
        .body("Cannot find GCD of Zero values");
    }

    let response=
    format!("The greatest common divisor of {} and {} is <b>{}</b>",
    form.n, form.m , gcd(form.n, form.m));
    HttpResponse::Ok().content_type("text/html").body(response)
}