use std::sync::{Arc, Mutex};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::models::user::{User, UserForCreate};

#[derive(Debug, Error)]
pub enum UserControllerError {
    #[error("User not found with id {0}")]
    UserNotFound(u64),
    #[allow(dead_code)]
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for UserControllerError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UserControllerError::UserNotFound(_) => StatusCode::NOT_FOUND,
            UserControllerError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = axum::Json(serde_json::json!({ "error": self.to_string() }));
        (status_code, body).into_response()
    }
}

#[derive(Clone)]
pub struct UserController {
    user_store: Arc<Mutex<Vec<Option<User>>>>,
}

impl UserController {
    pub async fn new() -> Result<Self, UserControllerError> {
        Ok(Self {
            user_store: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn create_user(
        &self,
        user_for_create: UserForCreate,
    ) -> Result<User, UserControllerError> {
        let mut store = self.user_store.lock().unwrap();

        let id = store.len() as u64;
        let user = User {
            id,
            username: user_for_create.username,
        };
        store.push(Some(user.clone()));

        Ok(user)
    }

    pub async fn read_user(&self, id: u64) -> Result<User, UserControllerError> {
        let store = self.user_store.lock().unwrap();

        let user = store.get(id as usize).unwrap().clone();

        user.ok_or(UserControllerError::UserNotFound(id))
    }

    pub async fn list_users(&self) -> Result<Vec<User>, UserControllerError> {
        let store = self.user_store.lock().unwrap();

        let users = store.iter().filter_map(|user| user.clone()).collect();

        Ok(users)
    }

    pub async fn delete_user(&self, id: u64) -> Result<User, UserControllerError> {
        let mut store = self.user_store.lock().unwrap();

        let user = store.get_mut(id as usize).and_then(|user| user.take());

        user.ok_or(UserControllerError::UserNotFound(id))
    }
}
