use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Articles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Articles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Articles::Title).string().not_null())
					.col(ColumnDef::new(Articles::Introduction).string().not_null())
                    .col(ColumnDef::new(Articles::Content).text().not_null())
					.col(ColumnDef::new(Articles::UserId).integer().unique_key().not_null())
                    .col(
                        ColumnDef::new(Articles::ViewCount)
                            .unsigned()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Articles::Status)
                            .small_unsigned()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Articles::IsDeleted)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Articles::CreatedAt)
                            .date_time()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Articles::UpdatedAt)
                            .date_time()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Articles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Articles {
    Table,
    Id,
    Title,
	Introduction,
    Content,
	UserId,
	ViewCount,
    Status,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}
