use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub password: String
}