pub use sea_orm_migration::prelude::*;

mod m20240630_023910_create_users_table;
mod m20240630_030928_create_articles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240630_023910_create_users_table::Migration),
            Box::new(m20240630_030928_create_articles_table::Migration),
        ]
    }
}
