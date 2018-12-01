use actix_web::http::Method;
use actix_web::{http, server, App, HttpRequest, HttpResponse, Path, Responder};

fn main() {
    server::new(|| App::new().resource("/{id}", |r| r.method(Method::GET).f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

fn index(req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", req.match_info().get("name").unwrap()))
}

// fn index(_req: &HttpRequest) -> &'static str {
//     "Hello world!"
// }

// fn main() {
//     server::new(|| App::new().resource("/", |r| r.f(index)))
//         .bind("127.0.0.1:8088")
//         .unwrap()
//         .run();
// }
