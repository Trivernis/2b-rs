use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Displays information about the bot")]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("About").description(format!(
                    "\
            I'm a general purpose discord bot written in rusty Rust. \
            My main focus is providing information about all kinds of stuff and playing music.\
             Use `{}help` to get an overview of the commands I provide.",
                    std::env::var("BOT_PREFIX").unwrap()
                ))
            })
        })
        .await?;

    Ok(())
}
