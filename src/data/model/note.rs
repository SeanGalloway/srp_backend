use sea_orm::{ActiveModelBehavior, DeriveEntityModel, PrimaryKeyTrait, EntityTrait, DerivePrimaryKey, DeriveRelation, EnumIter, Related, RelationDef};
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};
use crate::data::enumeration::user_role::UserRole;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "note")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: Uuid,
    // #[sea_orm(foreign_key = "super::user::User")]
    pub owner_id: i32,
    pub title: String,
    pub body: String,
    pub created_ts: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(
    //     belongs_to = "super::user::User",
    //     from = "Column::user_id",
    //     to = "super::user::Column::id"
    // )]
    // User
}

// impl Related<super::user::User> for Note {
//     fn to() -> RelationDef {
//         Relation::User.def()
//     }
// }


impl ActiveModelBehavior for ActiveModel {}