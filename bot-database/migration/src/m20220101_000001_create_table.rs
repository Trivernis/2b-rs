use sea_orm_migration::prelude::*;

pub struct Migration;

#[derive(Iden)]
pub enum EphemeralMessages {
    Table,
    ChannelId,
    MessageId,
    Timeout,
}

#[derive(Iden)]
pub enum GuildPlaylists {
    Table,
    GuildId,
    Name,
    Url,
}

#[derive(Iden)]
pub enum GuildSettings {
    Table,
    GuildId,
    Key,
    Value,
}

#[derive(Iden)]
pub enum Media {
    Table,
    Id,
    Category,
    Name,
    Url,
}

#[derive(Iden)]
pub enum Statistics {
    Table,
    Id,
    Version,
    Command,
    ExecutedAt,
    Success,
    ErrorMsg,
}

#[derive(Iden)]
pub enum YoutubeSongs {
    Table,
    Id,
    SpotifyId,
    Artist,
    Title,
    Album,
    Url,
    Score,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(ephemeral_messages()).await?;
        manager.create_table(guild_playlists()).await?;
        manager.create_table(guild_settings()).await?;
        manager.create_table(media()).await?;
        manager.create_table(statistics()).await?;
        manager.create_table(youtube_songs()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EphemeralMessages::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GuildPlaylists::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GuildSettings::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Media::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Statistics::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(YoutubeSongs::Table).to_owned())
            .await?;

        Ok(())
    }
}

fn ephemeral_messages() -> TableCreateStatement {
    Table::create()
        .table(EphemeralMessages::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(EphemeralMessages::ChannelId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(EphemeralMessages::MessageId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(EphemeralMessages::Timeout)
                .timestamp()
                .not_null(),
        )
        .primary_key(
            Index::create()
                .col(EphemeralMessages::ChannelId)
                .col(EphemeralMessages::MessageId),
        )
        .to_owned()
}

fn guild_playlists() -> TableCreateStatement {
    Table::create()
        .table(GuildPlaylists::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(GuildPlaylists::GuildId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(GuildPlaylists::Name)
                .string_len(255)
                .not_null(),
        )
        .col(
            ColumnDef::new(GuildPlaylists::Url)
                .string_len(1204)
                .not_null(),
        )
        .primary_key(
            Index::create()
                .col(GuildPlaylists::GuildId)
                .col(GuildPlaylists::Name),
        )
        .to_owned()
}

fn guild_settings() -> TableCreateStatement {
    Table::create()
        .table(GuildSettings::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(GuildSettings::GuildId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(GuildSettings::Key)
                .string_len(255)
                .not_null(),
        )
        .col(
            ColumnDef::new(GuildSettings::Value)
                .string_len(1024)
                .not_null(),
        )
        .primary_key(
            Index::create()
                .col(GuildSettings::GuildId)
                .col(GuildSettings::Key),
        )
        .to_owned()
}

fn media() -> TableCreateStatement {
    Table::create()
        .table(Media::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Media::Id)
                .big_integer()
                .auto_increment()
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(Media::Category).string_len(128))
        .col(ColumnDef::new(Media::Name).string_len(128))
        .col(ColumnDef::new(Media::Url).string_len(128))
        .index(
            Index::create()
                .unique()
                .col(Media::Category)
                .col(Media::Name),
        )
        .to_owned()
}

fn statistics() -> TableCreateStatement {
    Table::create()
        .table(Statistics::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Statistics::Id)
                .big_integer()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(Statistics::Version)
                .string_len(32)
                .not_null(),
        )
        .col(
            ColumnDef::new(Statistics::Command)
                .string_len(255)
                .not_null(),
        )
        .col(
            ColumnDef::new(Statistics::ExecutedAt)
                .timestamp()
                .not_null(),
        )
        .col(
            ColumnDef::new(Statistics::Success)
                .boolean()
                .not_null()
                .default(true),
        )
        .col(ColumnDef::new(Statistics::ErrorMsg).string())
        .to_owned()
}

fn youtube_songs() -> TableCreateStatement {
    Table::create()
        .table(YoutubeSongs::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(YoutubeSongs::Id)
                .big_integer()
                .primary_key()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(YoutubeSongs::SpotifyId)
                .string_len(255)
                .not_null(),
        )
        .col(
            ColumnDef::new(YoutubeSongs::Artist)
                .string_len(128)
                .not_null(),
        )
        .col(
            ColumnDef::new(YoutubeSongs::Title)
                .string_len(255)
                .not_null(),
        )
        .col(
            ColumnDef::new(YoutubeSongs::Album)
                .string_len(255)
                .not_null(),
        )
        .col(ColumnDef::new(YoutubeSongs::Url).string_len(128).not_null())
        .col(
            ColumnDef::new(YoutubeSongs::Score)
                .integer()
                .default(0)
                .not_null(),
        )
        .index(
            Index::create()
                .unique()
                .col(YoutubeSongs::SpotifyId)
                .col(YoutubeSongs::Url),
        )
        .to_owned()
}
