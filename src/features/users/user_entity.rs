use serde::Serialize;
use uuid::Uuid;

// the output to our `create_user` handler
#[derive(Clone, Serialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}
