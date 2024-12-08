use crate::data::model::note;
use crate::data::note_repository::find_notes_by_user_id;
use crate::util::app_context::AppContext;
use crate::util::error_handling::application_error::ApplicationError;

pub async fn get_all(id: i32, ctx: &AppContext) -> Result<Vec<note::Model>, ApplicationError> {
    Ok(find_notes_by_user_id(id, ctx).await?)
}