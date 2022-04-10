use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::GuildChannel;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

use crate::commands::music::get_music_player_for_guild;
use crate::utils::context_data::MusicPlayers;
use crate::utils::delete_messages_from_database;
use serenity::model::event;
use serenity_rich_interaction::events::RichEventHandler;
use serenity_rich_interaction::Result;

/// Returns the raw event handler built from a rich event handler
pub fn get_raw_event_handler() -> RichEventHandler {
    let mut handler = RichEventHandler::default();
    handler
        .add_event(|ctx, e: &event::ReadyEvent| Box::pin(ready(ctx, &e.ready)))
        .add_event(|_ctx, _: &event::ResumedEvent| {
            Box::pin(async {
                tracing::info!("Reconnected to Gateway");
                Ok(())
            })
        });

    handler
}

async fn ready(ctx: &Context, _: &Ready) -> Result<()> {
    tracing::info!("Ready");
    delete_messages_from_database(&ctx).await?;
    let prefix = std::env::var("BOT_PREFIX").unwrap_or("~!".to_string());
    ctx.set_activity(Activity::listening(format!("{}help", prefix).as_str()))
        .await;
    Ok(())
}

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn resume(&self, _: Context, _: ResumedEvent) {
        tracing::info!("Reconnected to gateway")
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        guild_id: Option<GuildId>,
        old_state: Option<VoiceState>,
        new_state: VoiceState,
    ) {
        let mut member_count = None;

        let guild_id = if let Some(gid) = guild_id {
            gid
        } else {
            return;
        };

        if let Some(old_id) = old_state.clone().and_then(|c| c.channel_id) {
            member_count = get_own_channel_member_count(&ctx, old_id).await;
        }
        if member_count.is_none() {
            if let Some(new_id) = new_state.channel_id {
                member_count = get_own_channel_member_count(&ctx, new_id).await;
            }
        }

        if let Some(count) = member_count {
            tracing::debug!("{} Members in channel", count);
            if let Some(player) = get_music_player_for_guild(&ctx, guild_id).await {
                let mut player = player.lock().await;
                tracing::debug!("Setting leave flag to {}", count == 0);
                player.set_leave_flag(count == 0);
            }
        }
        // handle disconnects
        if let (Some(state), None) = (old_state, new_state.channel_id) {
            let current_user = ctx.cache.current_user().await;

            if state.user_id == current_user.id {
                let mut data = ctx.data.write().await;
                let players = data.get_mut::<MusicPlayers>().unwrap();

                if let Some(player) = players.remove(&guild_id.0) {
                    let mut player = player.lock().await;
                    let _ = player.delete_now_playing().await;
                    let _ = player.stop().await;
                }
            }
        }
    }
}

/// Returns the number of members in the channel if it's the bots voice channel
async fn get_own_channel_member_count(ctx: &Context, channel_id: ChannelId) -> Option<usize> {
    let guild_channel = get_guild_channel(ctx, channel_id).await?;

    let current_user = ctx.cache.current_user().await;

    let members = guild_channel.members(&ctx).await.ok()?;
    let own_channel = members
        .iter()
        .find(|m| m.user.id == current_user.id)
        .is_some();

    if !own_channel {
        return None;
    }
    let members: Vec<Member> = members.into_iter().filter(|m| !m.user.bot).collect();

    Some(members.len())
}

/// Returns the guild channel for a guild ID
async fn get_guild_channel(ctx: &Context, channel_id: ChannelId) -> Option<GuildChannel> {
    if let Some(channel) = ctx.cache.channel(channel_id).await {
        return channel.guild();
    }
    let channel = ctx.http.get_channel(channel_id.0).await.ok()?;
    channel.guild()
}
