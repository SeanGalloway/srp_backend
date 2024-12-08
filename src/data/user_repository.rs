use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use crate::data::model::user;
use crate::util::app_context::AppContext;
use crate::util::error_handling::application_error::{from_serializable, ApplicationError};

pub async fn find_user(email: &str, ctx: &AppContext) -> Result<user::Model, ApplicationError> {
    match user::Entity::find()
        .filter(user::Column::Email.contains(email))
        .one(&ctx.db)
        .await
        .map_err(from_serializable)?
    {
        Some(user) => Ok(user),
        None => Err(ApplicationError::new(&"user not found")),
    }
}

pub async fn save_user(user: user::Model, ctx: &AppContext) -> Result<user::Model, ApplicationError> {
    let mut active_user: user::ActiveModel = user.into();

    active_user.id = ActiveValue::<i32>::not_set();

    let saved_model = active_user
        .insert(&ctx.db)
        .await
        .map_err(|e| ApplicationError::new(&e.to_string()))?;

    Ok(saved_model)
}

pub async fn update_user(user: user::ActiveModel, ctx: &AppContext) -> Result<user::Model, ApplicationError> {
    let saved_model = user
        .update(&ctx.db)
        .await
        .map_err(|e| ApplicationError::new(&e.to_string()))?;

    Ok(saved_model)
}