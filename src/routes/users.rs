use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};

use crate::models::user::{User, UserForCreate};

pub fn routes() -> Router {
    Router::new()
        .route("/users/:name", get(handle_get_user))
        .route("/users", post(handle_create_user))
}

async fn handle_get_user(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello2 <strong>{name}</strong>"))
}

async fn handle_create_user(Json(payload): Json<UserForCreate>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
