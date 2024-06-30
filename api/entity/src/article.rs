use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "articles")]
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: i32,
	pub title: String,
	pub introduction: String,
	#[sea_orm(column_type = "Text")]
	pub content: String,
	pub user_id: i32,
	pub view_count: u32,
	pub status: u16,
	pub is_deleted: bool,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::user::Entity",
		from = "Column::UserId",
		to = "super::user::Column::Id"
	)]
	User,
}

impl RelationTrait for Relation {
	fn def(&self) -> RelationDef {
		match self {
			Self::Cake => Entity::belongs_to(super::user::Entity)
				.from(Column::UserId)
				.to(super::user::Column::Id)
				.into(),
		}
	}
}

impl Related<super::user::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
