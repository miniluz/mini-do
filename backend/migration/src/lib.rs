pub use sea_orm_migration::prelude::*;

mod m20230715_000001_create_todo_table;
mod m20231029_165550_add_done_column;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230715_000001_create_todo_table::Migration),
            Box::new(m20231029_165550_add_done_column::Migration),
        ]
    }
}
