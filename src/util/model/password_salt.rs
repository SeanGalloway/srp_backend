use argon2::password_hash::SaltString;
use serde::Deserialize;

pub struct PasswordSalt {
    pub password: String,
    pub salt: SaltString,
}