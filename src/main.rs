use std::{net::SocketAddr, str::FromStr};

use axum::{debug_handler, http::StatusCode, routing::get, Json, Router, Server};

#[tokio::main]
async fn main() {
    let users_router = Router::new()
        .route("/", get(list_users))
        .fallback(|| async { (StatusCode::NOT_FOUND, "404 in User Router") });

    let app = Router::new()
        .nest("/users", users_router)
        .fallback(get(|| async { (StatusCode::NOT_FOUND, "404 Generic") }));

    let addr = SocketAddr::from_str("127.0.0.1:5000").unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn list_users() -> Json<Vec<String>> {
    Json(vec!["Bob".into(), "George".into()])
}
