use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, post},
    Json, Router,
};

use super::{
    user_dto::UserForCreate,
    user_entity::User,
    user_service::{UserService, UserServiceError},
};

impl IntoResponse for UserServiceError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UserServiceError::UserNotFound(_) => StatusCode::NOT_FOUND,
            UserServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = axum::Json(serde_json::json!({ "error": self.to_string() }));
        (status_code, body).into_response()
    }
}

pub fn routes(service: UserService) -> Router {
    Router::new()
        .route("/users", post(handle_create_user).get(handle_list_users))
        .route(
            "/users/:id",
            delete(handle_delete_user).get(handle_read_user),
        )
        .with_state(service)
}

async fn handle_read_user(
    State(service): State<UserService>,
    Path(id): Path<u64>,
) -> Result<Json<User>, UserServiceError> {
    let user = service.read_user(id).await?;

    Ok(Json(user))
}

async fn handle_create_user(
    State(service): State<UserService>,
    Json(user): Json<UserForCreate>,
) -> Result<Json<User>, UserServiceError> {
    let user = service.create_user(user).await?;

    Ok(Json(user))
}

async fn handle_list_users(
    State(service): State<UserService>,
) -> Result<Json<Vec<User>>, UserServiceError> {
    let users = service.list_users().await?;

    Ok(Json(users))
}

async fn handle_delete_user(
    State(service): State<UserService>,
    Path(id): Path<u64>,
) -> Result<Json<User>, UserServiceError> {
    let user = service.delete_user(id).await?;

    Ok(Json(user))
}
