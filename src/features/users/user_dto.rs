use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user_entity::User;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserCreateDto {
    pub id: Option<Uuid>,
    pub username: String,
    pub password: String,
}

pub fn get_user_from_dto(user_dto: UserCreateDto, password_hash: String) -> User {
    User {
        id: user_dto.id.unwrap_or(Uuid::new_v4()),
        username: user_dto.username,
        password_hash,
    }
}

pub fn get_user_dto(user: User) -> UserDto {
    UserDto {
        id: user.id,
        username: user.username,
    }
}
