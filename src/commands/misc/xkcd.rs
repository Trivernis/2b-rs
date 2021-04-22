use crate::messages::xkcd::create_xkcd_menu;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use xkcd_search::get_comic;

#[command]
#[description("Retrieves xkcd comics")]
#[usage("[(<id>|<query..>)]")]
#[bucket("general")]
async fn xkcd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let comics = if let Ok(id) = args.single::<u32>() {
        if let Ok(comic) = xkcd_search::get_comic(id).await {
            vec![comic]
        } else {
            vec![]
        }
    } else if !args.is_empty() {
        let query = args.message();
        let results = xkcd_search::search(query).await?;
        let comics =
            futures::future::join_all(results.into_iter().map(|(_, id)| get_comic(id))).await;
        comics
            .into_iter()
            .filter_map(|result| result.ok())
            .collect()
    } else {
        vec![xkcd_search::get_latest_comic().await?]
    };

    create_xkcd_menu(ctx, msg.channel_id, comics).await?;

    Ok(())
}
