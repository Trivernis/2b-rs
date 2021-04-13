use std::process;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::Duration as ChronoDuration;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;
use sysinfo::{ProcessExt, SystemExt};

use crate::commands::common::handle_autodelete;
use crate::providers::music::queue::MusicQueue;
use crate::utils::context_data::{get_database_from_context, Store};
use std::sync::Arc;

#[command]
#[description("Shows some statistics about the bot")]
#[usage("")]
#[bucket("general")]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    log::debug!("Reading system stats");
    let database = get_database_from_context(ctx).await;
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let kernel_version = system.get_kernel_version().unwrap_or("n/a".to_string());
    let own_process = system.get_process(process::id() as i32).unwrap();
    let memory_usage = own_process.memory();
    let cpu_usage = own_process.cpu_usage();
    let thread_count = own_process.tasks.len();
    let current_user = ctx.http.get_current_user().await?;
    let guild_count: usize = current_user.guilds(ctx).await?.len();
    let bot_info = ctx.http.get_current_application_info().await?;

    let current_time_seconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let uptime = current_time_seconds - Duration::from_secs(own_process.start_time());
    let uptime = ChronoDuration::from_std(uptime).unwrap();
    let total_commands_executed = database.get_total_commands_statistic().await?;
    let shard_count = ctx.cache.shard_count().await;

    let discord_info = format!(
        r#"
    Version: {}
    Compiled with: rustc {}
    Owner: <@{}>
    Guilds: {}
    Shards: {}
    Voice Connections: {}
    Times Used: {}
    "#,
        crate::VERSION,
        rustc_version_runtime::version(),
        bot_info.owner.id,
        guild_count,
        shard_count,
        get_queue_count(ctx).await,
        total_commands_executed
    );

    log::trace!("Discord info {}", discord_info);

    let system_info = format!(
        r#"
    Kernel Version: {}
    Memory Usage: {:.2} MiB
    CPU Usage: {:.2} %
    Thread Count: {}
    Uptime: {}d {}h {}m
    "#,
        kernel_version,
        memory_usage as f64 / 1024f64,
        cpu_usage,
        thread_count,
        uptime.num_days(),
        uptime.num_hours() % 24,
        uptime.num_minutes() % 60
    );
    log::trace!("System info {}", system_info);

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Bot Stats")
                    .field("Bot Information", discord_info, true)
                    .field("System information", system_info, true)
            })
        })
        .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}

/// Returns the total number of queues that are not
/// flagged to leave
async fn get_queue_count(ctx: &Context) -> usize {
    let queues: Vec<Arc<Mutex<MusicQueue>>> = {
        let data = ctx.data.read().await;
        let store = data.get::<Store>().unwrap();

        store
            .music_queues
            .iter()
            .map(|(_, q)| Arc::clone(q))
            .collect()
    };
    let mut count = 0;
    for queue in queues {
        let queue = queue.lock().await;
        if !queue.leave_flag {
            count += 1;
        }
    }

    count
}
