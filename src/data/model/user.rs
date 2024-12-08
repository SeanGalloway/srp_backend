use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use data::enumeration::user_role::UserRole;
use crate::data;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub role: UserRole,
    pub password: String,
    pub salt: String,
    pub created_ts: DateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(has_many = "super::note::Note")]
    // Note,
}

// impl Related<super::note::Note> for User {
//     fn to() -> RelationDef {
//         Relation::Note.def()
//     }
// }


impl ActiveModelBehavior for ActiveModel {}