use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserForCreate {
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Clone, Serialize, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
}
