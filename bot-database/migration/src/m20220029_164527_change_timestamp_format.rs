use crate::{DbErr, Table};
use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Statistics {
    Table,
    ExecutedAt,
}

#[derive(Iden)]
pub enum EphemeralMessages {
    Table,
    Timeout,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220029_164527_change_timestamp_format"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Statistics::Table)
                    .modify_column(
                        ColumnDef::new(Statistics::ExecutedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(EphemeralMessages::Table)
                    .modify_column(
                        ColumnDef::new(EphemeralMessages::Timeout)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Statistics::Table)
                    .modify_column(
                        ColumnDef::new(Statistics::ExecutedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(EphemeralMessages::Table)
                    .modify_column(
                        ColumnDef::new(EphemeralMessages::Timeout)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
