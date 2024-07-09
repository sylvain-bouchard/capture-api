use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::{
    controllers::user_controller::{UserController, UserControllerError},
    models::user::{User, UserForCreate},
};

pub fn routes(controller: UserController) -> Router {
    Router::new()
        .route("/users", post(handle_create_user).get(handle_list_users))
        .route(
            "/users/:id",
            delete(handle_delete_user).get(handle_read_user),
        )
        .with_state(controller)
}

async fn handle_read_user(
    State(controller): State<UserController>,
    Path(id): Path<u64>,
) -> Result<Json<User>, UserControllerError> {
    let user = controller.read_user(id).await?;

    Ok(Json(user))
}

async fn handle_create_user(
    State(controller): State<UserController>,
    Json(user): Json<UserForCreate>,
) -> Result<Json<User>, UserControllerError> {
    let user = controller.create_user(user).await?;

    Ok(Json(user))
}

async fn handle_list_users(
    State(controller): State<UserController>,
) -> Result<Json<Vec<User>>, UserControllerError> {
    let users = controller.list_users().await?;

    Ok(Json(users))
}

async fn handle_delete_user(
    State(controller): State<UserController>,
    Path(id): Path<u64>,
) -> Result<Json<User>, UserControllerError> {
    let user = controller.delete_user(id).await?;

    Ok(Json(user))
}
