use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Game::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Game::EsId).integer().not_null())
                    .col(ColumnDef::new(Game::Name).integer().not_null())
                    .col(ColumnDef::new(Game::Images).string().not_null())
                    .col(ColumnDef::new(Game::BrandId).integer().not_null())
                    .col(ColumnDef::new(Game::LibraryRegistered).boolean().not_null())
                    .col(ColumnDef::new(Game::LibraryRegisteredAt).timestamp().not_null())
                    .col(ColumnDef::new(Game::LastPlayedAt).timestamp().not_null())
                    .col(ColumnDef::new(Game::Folder).string().not_null())
                    .col(ColumnDef::new(Game::ExecutablePath).string().not_null())
                    .col(ColumnDef::new(Game::ExecutableAutoDetect).boolean().not_null())
                    .col(ColumnDef::new(Game::PlayCount).integer().not_null())
                    .col(ColumnDef::new(Game::PlayTime).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Game::Table, Game::BrandId)
                            .to(Brand::Table, Brand::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Brand::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Brand::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Brand::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Config::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Config::Key).string().not_null().primary_key())
                    .col(ColumnDef::new(Config::Value).json().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Game::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Brand::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Config::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    EsId,
    BrandId,
    Name,
    Images,
    LibraryRegistered,
    LibraryRegisteredAt,
    LastPlayedAt,
    Folder,
    ExecutablePath,
    ExecutableAutoDetect,
    PlayCount,
    PlayTime,
}

#[derive(DeriveIden)]
enum Brand {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Config {
    Table,
    Key,
    Value,
}
