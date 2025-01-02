use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use crate::service::Service;

use super::{user_entity::User, user_record::ActiveModel, user_record::Entity as UserRecord};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("User with id {0} not found")]
    UserNotFound(Uuid),
    #[allow(dead_code)]
    #[error("Internal server error")]
    InternalServerError,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[derive(Clone)]
pub struct UserService {
    pub name: String,
    connection: Option<Arc<DatabaseConnection>>,
}

impl Service for UserService {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl UserService {
    pub fn new(connection: Option<Arc<DatabaseConnection>>) -> Self {
        Self {
            name: String::from("UserService"),
            connection,
        }
    }

    pub async fn create_user(&self, user: User) -> Result<User, UserServiceError> {
        if let Some(conn) = &self.connection {
            let new_user = ActiveModel {
                id: Set(user.id),
                username: Set(user.username),
                password_hash: Set(user.password_hash),
                created_at: NotSet,
                updated_at: NotSet,
            };

            let inserted_user = new_user.insert(conn.as_ref()).await.map_err(|err| {
                println!("A database error occurred: {}", err.to_string());
                UserServiceError::DatabaseError(err.to_string())
            })?;

            Ok(User {
                id: inserted_user.id,
                username: inserted_user.username,
                password_hash: inserted_user.password_hash,
            })
        } else {
            Err(UserServiceError::InternalServerError)
        }
    }

    pub async fn read_user(&self, id: Uuid) -> Result<User, UserServiceError> {
        let connection = self
            .connection
            .as_ref()
            .ok_or(UserServiceError::InternalServerError)?;

        let user_record = UserRecord::find()
            .one(connection.as_ref())
            .await
            .map_err(|err| UserServiceError::DatabaseError(err.to_string()))?
            .ok_or(UserServiceError::UserNotFound(id))?;

        let user = User {
            id: user_record.id,
            username: user_record.username,
            password_hash: user_record.password_hash,
        };

        Ok(user)
    }

    pub async fn list_users(&self) -> Result<Vec<User>, UserServiceError> {
        let connection = self
            .connection
            .as_ref()
            .ok_or(UserServiceError::InternalServerError)?;

        let user_records = UserRecord::find()
            .all(connection.as_ref())
            .await
            .map_err(|err| UserServiceError::DatabaseError(err.to_string()))?;

        let users = user_records
            .into_iter() // Consume the records directly, no need for `iter()`
            .map(|user| User {
                id: user.id,
                username: user.username,
                password_hash: user.password_hash,
            })
            .collect();

        Ok(users)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<User, UserServiceError> {
        Err(UserServiceError::UserNotFound(id))
    }
}
