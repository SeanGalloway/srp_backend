use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::data::model::{note};
use crate::util::app_context::AppContext;
use crate::util::error_handling::application_error::{from_serializable, ApplicationError};

pub async fn find_notes_by_user_id(id: i32, ctx: &AppContext) -> Result<Vec<note::Model>, ApplicationError> {
    let notes = note::Entity::find()
        .filter(note::Column::OwnerId.eq(id)).all(&ctx.db)
        .await
        .map_err(from_serializable)?;

    Ok(notes)
}