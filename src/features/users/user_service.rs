use std::sync::{Arc, Mutex};
use thiserror::Error;

use super::{user_dto::UserForCreate, user_entity::User};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("User not found with id {0}")]
    UserNotFound(u64),
    #[allow(dead_code)]
    #[error("Internal server error")]
    InternalServerError,
}

#[derive(Clone)]
pub struct UserService {
    user_store: Arc<Mutex<Vec<Option<User>>>>,
}

impl UserService {
    pub async fn new() -> Result<Self, UserServiceError> {
        Ok(Self {
            user_store: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn create_user(
        &self,
        user_for_create: UserForCreate,
    ) -> Result<User, UserServiceError> {
        let mut store = self.user_store.lock().unwrap();

        let id = store.len() as u64;
        let user = User {
            id,
            username: user_for_create.username,
        };
        store.push(Some(user.clone()));

        Ok(user)
    }

    pub async fn read_user(&self, id: u64) -> Result<User, UserServiceError> {
        let store = self.user_store.lock().unwrap();

        let user = store.get(id as usize).unwrap().clone();

        user.ok_or(UserServiceError::UserNotFound(id))
    }

    pub async fn list_users(&self) -> Result<Vec<User>, UserServiceError> {
        let store = self.user_store.lock().unwrap();

        let users = store.iter().filter_map(|user| user.clone()).collect();

        Ok(users)
    }

    pub async fn delete_user(&self, id: u64) -> Result<User, UserServiceError> {
        let mut store = self.user_store.lock().unwrap();

        let user = store.get_mut(id as usize).and_then(|user| user.take());

        user.ok_or(UserServiceError::UserNotFound(id))
    }
}
