use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: i32,
	pub nickname: String,
	pub email: String,
	pub password: String,
	pub permission_code: u8,
	pub activated_at: NaiveDateTime,
	pub is_deleted: bool,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::article::Entity")]
	Article,
}

impl RelationTrait for Relation {
	fn def(&self) -> RelationDef {
		match self {
			Self::Fruit => Entity::has_many(super::article::Entity).into(),
		}
	}
}

impl Related<super::article::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Article.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
