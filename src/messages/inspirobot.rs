use crate::providers::music::inspirobot::get_inspirobot_image;
use crate::utils::error::BotResult;
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::id::{ChannelId, UserId};
use serenity_additions::core::EXTRA_LONG_TIMEOUT;
use serenity_additions::menu::{close_menu, display_page, MenuBuilder, Page, CLOSE_MENU_EMOJI};

static REFRESH_EMOJI: &str = "🔄";

pub async fn create_inspirobot_menu(
    ctx: &Context,
    channel_id: ChannelId,
    owner: UserId,
) -> BotResult<()> {
    MenuBuilder::default()
        .add_control(0, REFRESH_EMOJI, |ctx, menu, _r| {
            Box::pin(async move {
                display_page(ctx, menu).await?;
                Ok(())
            })
        })
        .add_help(REFRESH_EMOJI, "Creates a new inspiring image.")
        .add_control(1, CLOSE_MENU_EMOJI, |c, m, r| Box::pin(close_menu(c, m, r)))
        .add_help(CLOSE_MENU_EMOJI, "Closes this menu.")
        .show_help()
        .add_page(Page::new_builder(|| {
            Box::pin(async {
                let message = create_inspirobot_page()
                    .await
                    .map_err(|e| serenity_additions::Error::Msg(format!("{}", e)))?;
                Ok(message)
            })
        }))
        .owner(owner)
        .timeout(EXTRA_LONG_TIMEOUT)
        .build(ctx, channel_id)
        .await?;

    Ok(())
}

async fn create_inspirobot_page<'a>() -> BotResult<CreateMessage<'a>> {
    let image = get_inspirobot_image().await?;
    let mut message = CreateMessage::default();
    message.embed(|e| {
        e.image(image)
            .title("Be inspired")
            .footer(|f| f.text("Powered by inspirobot.me"))
    });

    Ok(message)
}
