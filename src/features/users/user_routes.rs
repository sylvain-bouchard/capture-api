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

use crate::{application::ApplicationState, service::ServiceType};

use super::{
    user_dto::{get_user_dto, UserDto},
    user_service::{UserService, UserServiceError},
};

impl IntoResponse for UserServiceError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UserServiceError::UserNotFound(_) => StatusCode::NOT_FOUND,
            UserServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::LockPoisoned => StatusCode::INTERNAL_SERVER_ERROR,
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
    Json(user): Json<UserDto>,
) -> impl IntoApiResponse {
    match service.create_user(user).await {
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
