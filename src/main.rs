use actix_web::{web, App, HttpServer, HttpResponse}

fn main() {
    let server= HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
    });
    printl!("Servering on port 8000");
    server.bind("127.0.0.1:8000").expect("error binding server to address")
     .run().expect("error running server");
}

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