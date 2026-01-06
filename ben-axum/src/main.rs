use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::{self, net::TcpListener};

#[derive(Serialize)]
struct Message {
    code: i32,
    msg: String,
    data: Object,
}

#[derive(Serialize)]
struct Object {

}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Json<Message> {
    Json(Message {
        code: 0,
        msg : "OK".into(),
        data: Object {},
    })
}
