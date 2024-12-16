use std::sync::{Arc, Mutex};
use thiserror::Error;

use super::{user_dto::UserDto, user_entity::User};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("User not found with id {0}")]
    UserNotFound(u64),
    #[allow(dead_code)]
    #[error("Internal server error")]
    InternalServerError,
    #[error("Lock poisoned")]
    LockPoisoned,
}

#[derive(Clone)]
pub struct UserService {
    user_store: Arc<Mutex<Vec<User>>>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            user_store: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn create_user(&self, user_dto: UserDto) -> Result<User, UserServiceError> {
        let mut store = self.user_store.lock().unwrap();

        let id = user_dto.id.unwrap_or_else(|| store.len() as u64);
        let user = User {
            id,
            username: user_dto.username,
        };
        store.push(user.clone());

        Ok(user)
    }

    pub async fn read_user(&self, id: u64) -> Result<User, UserServiceError> {
        let store = self
            .user_store
            .lock()
            .map_err(|_| UserServiceError::LockPoisoned)?;

        match store.get(id as usize) {
            Some(user) => Ok(user.clone()),
            None => Err(UserServiceError::UserNotFound(id)),
        }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, UserServiceError> {
        let store = self.user_store.lock().unwrap();

        let users = store.iter().cloned().collect();

        Ok(users)
    }

    pub async fn delete_user(&self, id: u64) -> Result<User, UserServiceError> {
        let mut store = self
            .user_store
            .lock()
            .map_err(|_| UserServiceError::LockPoisoned)?;

        if (id as usize) < store.len() {
            Ok(store.remove(id as usize)) // Remove the user and return it
        } else {
            Err(UserServiceError::UserNotFound(id))
        }
    }
}
