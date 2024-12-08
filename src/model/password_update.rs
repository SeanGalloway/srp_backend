use serde::Deserialize;

#[derive(Deserialize)]
pub struct PasswordUpdate {
    pub email: String,
    pub old_password: String,
    pub new_password: String
}