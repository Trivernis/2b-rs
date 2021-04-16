table! {
    ephemeral_messages (channel_id, message_id) {
        channel_id -> Int8,
        message_id -> Int8,
        timeout -> Timestamp,
    }
}

table! {
    gifs (id) {
        id -> Int8,
        category -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        url -> Varchar,
    }
}

table! {
    guild_playlists (guild_id, name) {
        guild_id -> Int8,
        name -> Varchar,
        url -> Varchar,
    }
}

table! {
    guild_settings (guild_id, key) {
        guild_id -> Int8,
        key -> Varchar,
        value -> Nullable<Varchar>,
    }
}

table! {
    statistics (id) {
        id -> Int8,
        version -> Varchar,
        command -> Varchar,
        executed_at -> Timestamp,
        success -> Bool,
        error_msg -> Nullable<Text>,
    }
}

table! {
    youtube_songs (id) {
        id -> Int8,
        spotify_id -> Varchar,
        artist -> Varchar,
        title -> Varchar,
        album -> Varchar,
        url -> Varchar,
        score -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    ephemeral_messages,
    gifs,
    guild_playlists,
    guild_settings,
    statistics,
    youtube_songs,
);
