use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserForCreate {
    pub username: String,
}
