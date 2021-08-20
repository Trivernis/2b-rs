use crate::providers::music::inspirobot::get_inspirobot_image;
use crate::utils::error::BotResult;
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::id::ChannelId;
use serenity_rich_interaction::core::EXTRA_LONG_TIMEOUT;
use serenity_rich_interaction::menu::{display_page, MenuBuilder, Page};

static REFRESH_CONTROL: &str = "🔄";

pub async fn create_inspirobot_menu(ctx: &Context, channel_id: ChannelId) -> BotResult<()> {
    MenuBuilder::default()
        .add_control(0, REFRESH_CONTROL, |ctx, menu, _r| {
            Box::pin(async move {
                display_page(ctx, menu).await?;
                Ok(())
            })
        })
        .add_help(REFRESH_CONTROL, "Creates a new inspiring image.")
        .show_help()
        .add_page(Page::new_builder(|| {
            Box::pin(async {
                let message = create_inspirobot_page()
                    .await
                    .map_err(|e| serenity_rich_interaction::Error::Msg(format!("{}", e)))?;
                Ok(message)
            })
        }))
        .timeout(EXTRA_LONG_TIMEOUT)
        .build(ctx, channel_id)
        .await?;

    Ok(())
}

async fn create_inspirobot_page<'a>() -> BotResult<CreateMessage<'a>> {
    let image = get_inspirobot_image().await?;
    let mut message = CreateMessage::default();
    message.embed(|e| e.image(image).title("Be inspired"));

    Ok(message)
}
