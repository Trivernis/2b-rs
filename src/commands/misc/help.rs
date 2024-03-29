use std::collections::HashSet;

use serenity::client::Context;
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{help_commands, Args};
use serenity::framework::standard::{CommandGroup, CommandResult, HelpOptions};
use serenity::model::channel::Message;
use serenity::model::id::UserId;

use crate::commands::common::handle_autodelete;

#[help]
#[max_levenshtein_distance(2)]
pub async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    tracing::debug!("Help");
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    handle_autodelete(ctx, msg).await?;
    Ok(())
}
