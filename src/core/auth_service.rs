use crate::data::enumeration::user_role::UserRole;
use crate::data::model::user;
use crate::util::app_context::AppContext;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::model::user_claim::UserClaim;
use crate::util::error_handling::application_error::{from_serializable, ApplicationError};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use hmac::Mac;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use sea_orm::ActiveValue::Set;
use crate::data::user_repository::{find_user, save_user, update_user};
use crate::util::error_handling::error_type::ErrorType;
use crate::util::model::password_salt::PasswordSalt;

pub async fn login(
    email: &str,
    password: &str,
    ctx: &AppContext,
) -> Result<String, ApplicationError> {
    let argon2 = Argon2::default();

    match user::Entity::find()
        .filter(user::Column::Email.contains(email))
        .one(&ctx.db)
        .await
        .map_err(from_serializable)?
    {
        Some(user) => {
            let password_hash = user.password.clone();

            let parsed_hash = PasswordHash::new(&password_hash).unwrap();
            if argon2
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                generate_token(&user)
            } else {
                Err(ApplicationError::new(&"Invalid password"))
            }
        }
        None => Err(ApplicationError::new(&"user not found")),
    }
}

pub async fn register(
    first_name: &str,
    last_name: &str,
    email: &str,
    password: &str,
    ctx: &AppContext,
) -> Result<String, ApplicationError> {

    let password_salt = match(hash_password(&password)) {
        Ok(hashed_password) => hashed_password,
        Err(_) => return Err(ApplicationError::new("Error hashing password")),
    };

    let user = user::Model {
        id: 0,
        name_first: first_name.to_string(),
        name_last: last_name.to_string(),
        email: email.to_string(),
        role: UserRole::Editor,
        password: password_salt.password,
        salt: password_salt.salt.to_string(),
        created_ts: Default::default(),
    };

    let saved_model = save_user(user, ctx).await?;

    let token = generate_token(&saved_model)?;

    Ok(token)
}

pub async fn update(
    email: &str,
    old_password: &str,
    new_password: &str,
    ctx: &AppContext
) -> Result<String, ApplicationError> {
    match login(email, old_password, ctx).await {
        Ok(token) => token,
        Err(_) => return Err(ApplicationError::new("Incorrect password")),
    };

    match find_user(email, ctx).await {
        Ok(user) => {
            let password_salt = match(hash_password(&new_password)) {
                Ok(hashed_password) => hashed_password,
                Err(_) => return Err(ApplicationError::new("Error hashing password")),
            };
            let mut active_user: user::ActiveModel = user.into();
            active_user.password = Set(password_salt.password);
            active_user.salt = Set(password_salt.salt.to_string());


            let saved_user = update_user(active_user, ctx).await?;

            let token = generate_token(&saved_user)?;

            Ok(token)
        },
        Err(_) => Err(ApplicationError::new("User not found"))
    }
}

pub fn validate_token(token: &str) -> Result<UserClaim, ApplicationError> {
    let decoded_token = decode::<UserClaim>(
        token,
        &DecodingKey::from_secret("secretKeyWith$pec:alCh@rs".as_ref()),
        &Validation::default(),
    );

    match decoded_token {
        Ok(data) => Ok(data.claims),
        Err(e) => Err(ApplicationError::new_with_type(&e.to_string(), ErrorType::Unauthorized)),
    }
}

fn generate_token(user: &user::Model) -> Result<String, ApplicationError> {
    let exp = Utc::now() + Duration::hours(6);

    let user_claim = UserClaim {
        id: user.id.clone(),
        first_name: user.name_first.clone(),
        last_name: user.name_last.clone(),
        exp,
    };
    let token = encode(&Header::default(), &user_claim.to_map(), &EncodingKey::from_secret("secretKeyWith$pec:alCh@rs".as_ref())).map_err(from_serializable)?;

    Ok(token)
}

fn hash_password(password: &str) -> Result<PasswordSalt, ApplicationError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(&password.as_bytes(), &salt)
        .map_err(from_serializable)?;

    Ok(PasswordSalt { password: password_hash.to_string(), salt })
}

