use std::sync::Arc;

use serenity::builder::CreateEmbed;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;

use crate::commands::music::{get_music_player_for_guild, get_voice_manager, is_dj};
use crate::messages::add_ephemeral_handle_to_database;
use crate::providers::music::add_youtube_song_to_database;
use crate::providers::music::player::MusicPlayer;
use crate::providers::music::queue::Song;
use crate::utils::context_data::{DatabaseContainer, MusicPlayers, Store};
use crate::utils::error::*;
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::channel::Reaction;
use serenity_additions::core::MessageHandle;
use serenity_additions::menu::{Menu, MenuBuilder, Page};
use serenity_additions::Result as SerenityUtilsResult;
use std::env;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};

static DELETE_BUTTON: &str = "🗑️";
static PAUSE_BUTTON: &str = "⏯️";
static SKIP_BUTTON: &str = "⏭️";
static STOP_BUTTON: &str = "⏹️";
static GOOD_PICK_BUTTON: &str = "👍";

/// Creates a new now playing message and returns the embed for that message
pub async fn create_now_playing_msg(
    ctx: &Context,
    player: Arc<Mutex<MusicPlayer>>,
    channel_id: ChannelId,
) -> BotResult<Arc<RwLock<MessageHandle>>> {
    tracing::debug!("Creating now playing menu");
    let nsfw = channel_id.to_channel(ctx).await?.is_nsfw();
    let handle = MenuBuilder::default()
        .add_control(-1, DELETE_BUTTON, |c, m, r| {
            Box::pin(delete_action(c, m, r))
        })
        .add_help(DELETE_BUTTON, "Deletes this message")
        .add_control(0, STOP_BUTTON, |c, m, r| {
            Box::pin(stop_button_action(c, m, r))
        })
        .add_help(STOP_BUTTON, "Stops the music and leaves the channel")
        .add_control(1, PAUSE_BUTTON, |c, m, r| {
            Box::pin(play_pause_button_action(c, m, r))
        })
        .add_help(PAUSE_BUTTON, "Pauses the music")
        .add_control(2, SKIP_BUTTON, |c, m, r| {
            Box::pin(skip_button_action(c, m, r))
        })
        .add_help(SKIP_BUTTON, "Skips to the next song")
        .add_control(3, GOOD_PICK_BUTTON, |c, m, r| {
            Box::pin(good_pick_action(c, m, r))
        })
        .add_help(
            GOOD_PICK_BUTTON,
            "Remembers this video for spotify-youtube mappings",
        )
        .show_help()
        .add_page(Page::new_builder(move || {
            let player = Arc::clone(&player);
            Box::pin(async move {
                tracing::debug!("Creating now playing embed for page");
                let mut player = player.lock().await;
                tracing::debug!("player locked");
                let mut page = CreateMessage::default();

                if let Some(mut current) = player.queue().current().clone() {
                    let mut embed = CreateEmbed::default();
                    create_now_playing_embed(&mut current, &mut embed, player.is_paused(), nsfw)
                        .await;
                    page.embed(|e| {
                        e.0.clone_from(&embed.0);
                        e
                    });
                } else {
                    page.embed(|e| e.description("Queue is empty"));
                }
                tracing::debug!("Embed created");

                Ok(page)
            })
        }))
        .sticky(true)
        .timeout(Duration::from_secs(60 * 60 * 24))
        .build(ctx, channel_id)
        .await?;

    add_ephemeral_handle_to_database(ctx, *handle.read().await, Duration::from_secs(0)).await?;

    Ok(handle)
}

/// Updates the now playing message with new content
pub async fn update_now_playing_msg(
    http: &Arc<Http>,
    handle: &Arc<RwLock<MessageHandle>>,
    song: &mut Song,
    paused: bool,
) -> BotResult<()> {
    tracing::debug!("Updating now playing message");
    let handle = handle.read().await;
    let mut message = handle.get_message(http).await?;
    let nsfw = http.get_channel(handle.channel_id).await?.is_nsfw();

    let mut embed = CreateEmbed::default();
    create_now_playing_embed(song, &mut embed, paused, nsfw).await;
    message
        .edit(http, |m| {
            m.embed(|e| {
                e.0.clone_from(&embed.0);
                e
            })
        })
        .await?;
    tracing::debug!("Message updated.");

    Ok(())
}

/// Creates the embed of the now playing message
async fn create_now_playing_embed<'a>(
    song: &mut Song,
    mut embed: &'a mut CreateEmbed,
    paused: bool,
    nsfw: bool,
) -> &'a mut CreateEmbed {
    let url = song.url().await.unwrap();
    embed = embed
        .title(if paused { "Paused" } else { "Playing" })
        .description(format!(
            "[{}]({}) by {}",
            song.title().clone(),
            url,
            song.author().clone()
        ))
        .footer(|f| {
            f.text(format!(
                "Use {}play to add a song to the queue",
                env::var("BOT_PREFIX").unwrap()
            ))
        });

    if nsfw {
        if let Some(thumb) = song.thumbnail().clone() {
            embed = embed.thumbnail(thumb);
        }
    }

    embed
}

/// Toggled when the pause button is pressed
async fn play_pause_button_action(
    ctx: &Context,
    _: &mut Menu<'_>,
    reaction: Reaction,
) -> SerenityUtilsResult<()> {
    tracing::debug!("Play/Pause button pressed");
    let guild_id = reaction.guild_id.unwrap();
    let user = reaction.user(&ctx).await?;

    if !is_dj(ctx, guild_id, &user).await? {
        return Ok(());
    }
    {
        let player = get_music_player_for_guild(ctx, guild_id).await.unwrap();

        let (current, message, paused) = {
            tracing::debug!("Queue is locked");
            let mut player = player.lock().await;
            player.toggle_paused().await?;
            (
                player.queue().current().clone(),
                player.now_playing_message().clone().unwrap(),
                player.is_paused(),
            )
        };
        tracing::debug!("Queue is unlocked");

        if let Some(mut current) = current {
            update_now_playing_msg(&ctx.http, &message, &mut current, paused).await?;
        }
    }

    Ok(())
}

/// Triggered when the skip button is pressed
async fn skip_button_action(
    ctx: &Context,
    _: &mut Menu<'_>,
    reaction: Reaction,
) -> SerenityUtilsResult<()> {
    let guild_id = reaction.guild_id.unwrap();
    let user = reaction.user(&ctx).await?;

    if !is_dj(ctx, guild_id, &user).await? {
        return Ok(());
    }

    {
        let player = get_music_player_for_guild(ctx, guild_id).await.unwrap();
        let mut player = player.lock().await;
        player.skip().await?;
    }

    Ok(())
}

/// Triggered when the stop button is pressed
async fn stop_button_action(
    ctx: &Context,
    menu: &mut Menu<'_>,
    reaction: Reaction,
) -> SerenityUtilsResult<()> {
    let guild_id = reaction.guild_id.unwrap();
    let user = reaction.user(&ctx).await?;

    if !is_dj(ctx, guild_id, &user).await? {
        return Ok(());
    }
    {
        let manager = get_voice_manager(ctx).await;

        let handler = manager.get(guild_id);

        if let Some(handler) = handler {
            let mut handler_lock = handler.lock().await;
            let _ = handler_lock.leave().await;
        }

        if manager.get(guild_id).is_some() {
            manager.remove(guild_id).await.map_err(BotError::from)?;
            let mut data = ctx.data.write().await;
            let players = data.get_mut::<MusicPlayers>().unwrap();

            if let Some(player) = players.remove(&guild_id.0) {
                let mut player = player.lock().await;
                player.stop().await?;
            }

            tracing::debug!("Left the voice channel");
        } else {
            tracing::debug!("Not in a voice channel");
        }
    }
    {
        let handle = &menu.message;
        let handle = handle.read().await;
        ctx.http
            .delete_message(handle.channel_id, handle.message_id)
            .await?;
    }

    Ok(())
}

async fn good_pick_action(
    ctx: &Context,
    _menu: &mut Menu<'_>,
    reaction: Reaction,
) -> SerenityUtilsResult<()> {
    let guild_id = reaction.guild_id.unwrap();
    let player = get_music_player_for_guild(ctx, guild_id).await.unwrap();
    let mut player = player.lock().await;

    if let Some(song) = player.queue().current() {
        let data = ctx.data.read().await;
        let store = data.get::<Store>().unwrap();
        let database = data.get::<DatabaseContainer>().unwrap();
        add_youtube_song_to_database(store, database, &mut song.clone()).await?;
    }

    Ok(())
}

async fn delete_action(
    ctx: &Context,
    menu: &mut Menu<'_>,
    reaction: Reaction,
) -> SerenityUtilsResult<()> {
    let guild_id = reaction.guild_id.unwrap();
    let handle = {
        let handle = menu.message.read().await;
        handle.clone()
    };
    {
        let player = get_music_player_for_guild(ctx, guild_id).await.unwrap();
        let mut player = player.lock().await;
        player.clear_now_playing();
    }
    ctx.http
        .delete_message(handle.channel_id, handle.message_id)
        .await?;

    Ok(())
}
