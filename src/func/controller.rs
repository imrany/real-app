use actix_web::{web, HttpResponse};
use serde::Deserialize;

//form struct / type or interface
#[derive(Deserialize)]
struct GCDParameters{
    n: u64,
    m:u64,
}

//get handler
pub fn get_index() -> HttpResponse{
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
pub fn post_gcd(form: web::Form<GCDParameters>) -> HttpResponse{
    if form.n==0||form.m==0 {
        return HttpResponse::BadRequest().content_type("text/html")
        .body("Cannot find GCD of Zero values");
    }

    let response=
    format!("The greatest common divisor of {} and {} is <b>{}</b>",
    form.n, form.m , gcd(form.n, form.m));
    HttpResponse::Ok().content_type("text/html").body(response)
}

//gcd function that computes the gcd of n and m
fn gcd(mut n:u64, mut m:u64)->u64{
    assert!(m!=0&&n!=0);
    while m !=0{
        if m<n{
            let t=m;
            m=n;
            n=t;
        }
        m=m%n;
    }
    n
}