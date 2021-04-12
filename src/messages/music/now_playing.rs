use std::sync::Arc;

use serenity::builder::CreateEmbed;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use songbird::input::Metadata;

use crate::commands::music::{get_queue_for_guild, get_voice_manager, is_dj};
use crate::providers::music::queue::MusicQueue;
use crate::utils::error::*;
use bot_serenityutils::core::MessageHandle;
use bot_serenityutils::error::SerenityUtilsResult;
use bot_serenityutils::menu::{Menu, MenuBuilder, Page};
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::channel::Reaction;
use std::env;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};

static PAUSE_BUTTON: &str = "⏯️";
static SKIP_BUTTON: &str = "⏭️";
static STOP_BUTTON: &str = "⏹️";

/// Creates a new now playing message and returns the embed for that message
pub async fn create_now_playing_msg(
    ctx: &Context,
    queue: Arc<Mutex<MusicQueue>>,
    channel_id: ChannelId,
) -> BotResult<Arc<RwLock<MessageHandle>>> {
    log::debug!("Creating now playing menu");
    let handle = MenuBuilder::default()
        .add_control(0, STOP_BUTTON, |c, m, r| {
            Box::pin(stop_button_action(c, m, r))
        })
        .add_control(1, PAUSE_BUTTON, |c, m, r| {
            Box::pin(play_pause_button_action(c, m, r))
        })
        .add_control(2, SKIP_BUTTON, |c, m, r| {
            Box::pin(skip_button_action(c, m, r))
        })
        .add_page(Page::new_builder(move || {
            let queue = Arc::clone(&queue);
            Box::pin(async move {
                log::debug!("Creating now playing embed for page");
                let queue = queue.lock().await;
                log::debug!("Queue locked");
                let mut page = CreateMessage::default();

                if let Some(current) = queue.current() {
                    page.embed(|e| create_now_playing_embed(current.metadata(), e, queue.paused()));
                } else {
                    page.embed(|e| e.description("Queue is empty"));
                }
                log::debug!("Embed created");

                Ok(page)
            })
        }))
        .sticky(true)
        .timeout(Duration::from_secs(60 * 60 * 24))
        .build(ctx, channel_id)
        .await?;

    Ok(handle)
}

/// Updates the now playing message with new content
pub async fn update_now_playing_msg(
    http: &Arc<Http>,
    handle: &Arc<RwLock<MessageHandle>>,
    meta: &Metadata,
    paused: bool,
) -> BotResult<()> {
    log::debug!("Updating now playing message");
    let handle = handle.read().await;
    let mut message = handle.get_message(http).await?;

    message
        .edit(http, |m| {
            m.embed(|e| create_now_playing_embed(meta, e, paused))
        })
        .await?;
    log::debug!("Message updated.");

    Ok(())
}

/// Creates the embed of the now playing message
fn create_now_playing_embed<'a>(
    meta: &Metadata,
    mut embed: &'a mut CreateEmbed,
    paused: bool,
) -> &'a mut CreateEmbed {
    embed = embed
        .title(if paused { "Paused" } else { "Playing" })
        .description(format!(
            "[{}]({}) by {}",
            meta.title.clone().unwrap(),
            meta.source_url.clone().unwrap(),
            meta.artist.clone().unwrap()
        ))
        .footer(|f| {
            f.text(format!(
                "Use {}play to add a song to the queue",
                env::var("BOT_PREFIX").unwrap()
            ))
        });

    if let Some(thumb) = meta.thumbnail.clone() {
        embed = embed.thumbnail(thumb);
    }

    embed
}

/// Toggled when the pause button is pressed
async fn play_pause_button_action(
    ctx: &Context,
    _: &mut Menu<'_>,
    reaction: Reaction,
) -> SerenityUtilsResult<()> {
    log::debug!("Play/Pause button pressed");
    let guild_id = reaction.guild_id.unwrap();
    let user = reaction.user(&ctx).await?;

    if !is_dj(ctx, guild_id, &user).await? {
        return Ok(());
    }
    {
        let queue = get_queue_for_guild(ctx, &guild_id).await?;

        let (current, message, paused) = {
            log::debug!("Queue is locked");
            let mut queue = queue.lock().await;
            queue.pause();
            (
                queue.current().clone(),
                queue.now_playing_msg.clone().unwrap(),
                queue.paused(),
            )
        };
        log::debug!("Queue is unlocked");

        if let Some(current) = current {
            update_now_playing_msg(&ctx.http, &message, current.metadata(), paused).await?;
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
        let current = {
            let queue = get_queue_for_guild(ctx, &guild_id).await?;
            let queue = queue.lock().await;
            queue.current().clone()
        };

        if let Some(current) = current {
            let _ = current.stop();
        }
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
        let queue = get_queue_for_guild(ctx, &guild_id).await?;
        let queue = queue.lock().await;

        let handler = manager.get(guild_id);

        if let Some(handler) = handler {
            let mut handler_lock = handler.lock().await;
            handler_lock.remove_all_global_events();
        }
        if let Some(current) = queue.current() {
            current.stop().map_err(BotError::from)?;
        }

        if manager.get(guild_id).is_some() {
            manager.remove(guild_id).await.map_err(BotError::from)?;
            log::debug!("Left the voice channel");
        } else {
            log::debug!("Not in a voice channel");
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
