use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Task::Table)
                    .add_column_if_not_exists(ColumnDef::new(Task::Done).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(Task::Table)
                    .drop_column(Task::Done)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    Title,
    Text,
    CreationTime,
    DueTime,
    Done,
}
