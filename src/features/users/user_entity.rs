use serde::Serialize;

// the output to our `create_user` handler
#[derive(Clone, Serialize, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
}
