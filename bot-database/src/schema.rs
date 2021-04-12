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

allow_tables_to_appear_in_same_query!(
    gifs,
    guild_playlists,
    guild_settings,
);
