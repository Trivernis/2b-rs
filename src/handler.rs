use crate::commands::music::get_queue_for_guild;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("Connected as {}", ready.user.name);
        let prefix = dotenv::var("BOT_PREFIX").unwrap_or("~!".to_string());
        ctx.set_activity(Activity::listening(format!("{}help", prefix).as_str()))
            .await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        log::info!("Reconnected to gateway")
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

        if let Some(old_id) = old_state.and_then(|c| c.channel_id) {
            member_count = get_own_channel_member_count(&ctx, &old_id).await;
        }
        if member_count.is_none() {
            if let Some(new_id) = new_state.channel_id {
                member_count = get_own_channel_member_count(&ctx, &new_id).await;
            }
        }

        if let Some(count) = member_count {
            log::debug!("{} Members in channel", count);
            let queue = get_queue_for_guild(&ctx, &guild_id).await.unwrap();
            let mut queue_lock = queue.lock().await;
            log::debug!("Setting leave flag to {}", count == 0);
            queue_lock.leave_flag = count == 0;
        }
    }
}

/// Returns the number of members in the channel if it's the bots voice channel
async fn get_own_channel_member_count(ctx: &Context, channel_id: &ChannelId) -> Option<usize> {
    let channel = ctx.http.get_channel(channel_id.0).await.ok()?;
    let guild_channel = channel.guild()?;
    let current_user = ctx.http.get_current_user().await.ok()?;

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
