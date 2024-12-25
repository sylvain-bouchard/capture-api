use aide::{
    axum::{
        routing::{get_with, post_with},
        ApiRouter, IntoApiResponse,
    },
    transform::TransformOperation,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{application::ApplicationState, service::ServiceType};

use super::{
    user_dto::{get_user_dto, get_user_from_dto, UserCreateDto, UserDto},
    user_service::{UserService, UserServiceError},
};

impl IntoResponse for UserServiceError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UserServiceError::UserNotFound(_) => StatusCode::NOT_FOUND,
            UserServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::LockPoisoned => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = axum::Json(serde_json::json!({ "error": self.to_string() }));
        (status_code, body).into_response()
    }
}

pub fn routes(state: ApplicationState) -> ApiRouter {
    let user_service = match state.service_provider.get_service("UserService") {
        Some(ServiceType::UserService(user_service)) => user_service,
        None => panic!("UserService not found in ServiceProvider"),
    };

    ApiRouter::new()
        .api_route(
            "/",
            post_with(handle_create_user, handle_create_user_docs)
                .get_with(handle_list_users, handle_list_users_docs),
        )
        .api_route(
            "/:id",
            get_with(handle_read_user, handle_read_user_docs)
                .delete_with(handle_delete_user, handle_delete_user_docs),
        )
        .with_state(user_service)
}

async fn handle_create_user(
    State(service): State<UserService>,
    Json(user_dto): Json<UserCreateDto>,
) -> impl IntoApiResponse {
    let hashed_password = match crypto_utils::hash_password(&user_dto.password) {
        Ok(hash) => hash,
        Err(_) => {
            eprintln!("Error hashing password");
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid input", "message": "Password hashing failed" })),
            )
                .into_response();
        }
    };

    match service
        .create_db_user(get_user_from_dto(user_dto, hashed_password))
        .await
    {
        Ok(created_user) => {
            let user_dto = get_user_dto(created_user);
            (StatusCode::CREATED, Json(user_dto)).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

fn handle_create_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new User.")
        .response::<201, Json<UserDto>>()
}

async fn handle_list_users(State(service): State<UserService>) -> impl IntoApiResponse {
    match service.list_users().await {
        Ok(users) => {
            let user_dtos: Vec<UserDto> = users.into_iter().map(get_user_dto).collect();
            Json(user_dtos).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

fn handle_list_users_docs(op: TransformOperation) -> TransformOperation {
    op.description("List User(s).")
        .response::<200, Json<Vec<UserDto>>>()
}

async fn handle_read_user(
    State(service): State<UserService>,
    Path(id): Path<u64>,
) -> impl IntoApiResponse {
    match service.read_user(id).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

fn handle_read_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Read the specified User.")
        .response::<200, Json<Vec<UserDto>>>()
}

async fn handle_delete_user(
    State(service): State<UserService>,
    Path(id): Path<u64>,
) -> impl IntoApiResponse {
    match service.delete_user(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

fn handle_delete_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete the specified User.")
        .response::<204, Json<Vec<UserDto>>>()
}

pub mod crypto_utils {
    use argon2::{
        self,
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hashed_password = argon2.hash_password(password.as_bytes(), &salt)?;

        Ok(hashed_password.to_string())
    }
}
