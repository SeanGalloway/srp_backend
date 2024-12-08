use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub user_name: String,
    pub password: String
}