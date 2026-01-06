use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    msg: String,
    code: i32,
    data: Object,
}

#[derive(Serialize)]
struct Object {}

async fn index() -> Result<HttpResponse> {
    let response = Message {
        msg: "OK".into(),
        code: 0,
        data: Object {},
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
