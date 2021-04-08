use std::collections::HashSet;

use serenity::client::Context;
use serenity::framework::standard::{Args, help_commands};
use serenity::framework::standard::{CommandGroup, CommandResult, HelpOptions};
use serenity::framework::standard::macros::help;
use serenity::model::channel::Message;
use serenity::model::id::UserId;

#[help]
pub async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;

    Ok(())
}
