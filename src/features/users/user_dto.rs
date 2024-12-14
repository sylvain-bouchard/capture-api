use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::user_entity::User;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UserDto {
    pub id: Option<u64>,
    pub username: String,
}

pub fn get_user_dto(user: User) -> UserDto {
    UserDto {
        id: Some(user.id),
        username: user.username,
    }
}
