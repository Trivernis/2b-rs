use crate::commands::music::is_dj;
use crate::providers::music::player::MusicPlayer;
use crate::utils::error::BotResult;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::client::Context;
use serenity::model::channel::Reaction;
use serenity::model::id::ChannelId;
use serenity_rich_interaction::core::EXTRA_LONG_TIMEOUT;
use serenity_rich_interaction::menu::{display_page, Menu, MenuBuilder, Page};
use serenity_rich_interaction::Result as SerenityUtilsResult;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use typemap_rev::TypeMapKey;

static DELETE_BUTTON: &str = "🗑️";
static NEXT_BAND_BUTTON: &str = "➡️";
static PREVIOUS_BAND_BUTTON: &str = "⬅️";
static ADD_BUTTON: &str = "➕";
static SUB_BUTTON: &str = "➖";

struct SelectedBand;

impl TypeMapKey for SelectedBand {
    type Value = Arc<AtomicU8>;
}

struct Player;

impl TypeMapKey for Player {
    type Value = Arc<Mutex<MusicPlayer>>;
}

/// Creates a new equalizer message
pub async fn create_equalizer_message(
    ctx: &Context,
    channel_id: ChannelId,
    player: Arc<Mutex<MusicPlayer>>,
) -> BotResult<()> {
    let selected_band = Arc::new(AtomicU8::new(0));
    let selected_band_clone = Arc::clone(&selected_band);
    let player_clone = Arc::clone(&player);

    MenuBuilder::default()
        .add_page(Page::new_builder(move || {
            let player = Arc::clone(&player_clone);
            let selected_band = Arc::clone(&selected_band_clone);
            Box::pin(async move {
                let mut page = CreateMessage::default();
                let mut embed = CreateEmbed::default();
                create_equalizer_embed(selected_band.load(Ordering::Relaxed), &mut embed, &player)
                    .await;

                page.embed(|e| {
                    e.0.clone_from(&embed.0);
                    e
                });

                Ok(page)
            })
        }))
        .add_control(-1, DELETE_BUTTON, |c, m, r| Box::pin(delete_menu(c, m, r)))
        .add_help(DELETE_BUTTON, "Deletes this message.")
        .add_control(0, PREVIOUS_BAND_BUTTON, |c, m, r| {
            Box::pin(previous_band(c, m, r))
        })
        .add_help(PREVIOUS_BAND_BUTTON, "Selects the previous band.")
        .add_control(1, NEXT_BAND_BUTTON, |c, m, r| Box::pin(next_band(c, m, r)))
        .add_help(NEXT_BAND_BUTTON, "Selects the next band.")
        .add_control(3, ADD_BUTTON, |c, m, r| Box::pin(add_to_band(c, m, r)))
        .add_help(ADD_BUTTON, "Adds to the selected band.")
        .add_control(2, SUB_BUTTON, |c, m, r| {
            Box::pin(subtract_from_band(c, m, r))
        })
        .add_help(SUB_BUTTON, "Subtracts from the selected band")
        .show_help()
        .add_data::<SelectedBand>(selected_band)
        .add_data::<Player>(player)
        .timeout(EXTRA_LONG_TIMEOUT)
        .build(ctx, channel_id)
        .await?;
    Ok(())
}

/// Creates a new equalizer embed
async fn create_equalizer_embed<'a>(
    selected_band: u8,
    embed: &'a mut CreateEmbed,
    player: &Arc<Mutex<MusicPlayer>>,
) -> &'a mut CreateEmbed {
    let mut description = String::new();
    let bands = {
        let player = player.lock().await;
        player.get_equalizer().clone()
    };
    for i in 0..bands.len() {
        if i as u8 == selected_band {
            description += "⤋"
        } else {
            description += " ";
        }
    }
    description += "\n";
    for i in (0..11).rev() {
        let eq_value = (i as f64) / 20.0 - 0.25;

        for band in &bands {
            if (eq_value > 0. && band >= &eq_value) || (eq_value < 0. && band <= &eq_value) {
                description += "█";
            } else if eq_value == 0. {
                description += format!("-").as_str();
            } else {
                description += " ";
            }
        }
        description += "\n";
    }
    for i in 0..bands.len() {
        if i as u8 == selected_band {
            description += "⤊"
        } else {
            description += " ";
        }
    }
    embed
        .title("Equalizer")
        .description(format!("```\n{}\n```", description));

    embed
}

/// Selects the previous band
async fn next_band(
    ctx: &Context,
    menu: &mut Menu<'_>,
    reaction: Reaction,
) -> SerenityUtilsResult<()> {
    let guild_id = reaction.guild_id.unwrap();
    let user = reaction.user(&ctx).await?;

    if !is_dj(ctx, guild_id, &user).await? {
        return Ok(());
    }
    let selected_band = menu.data.get::<SelectedBand>().unwrap();
    if selected_band.load(Ordering::SeqCst) >= 14 {
        selected_band.store(0, Ordering::SeqCst);
    } else {
        selected_band.fetch_add(1, Ordering::SeqCst);
    }
    display_page(ctx, menu).await?;

    Ok(())
}

/// Selects the previous band
async fn previous_band(
    ctx: &Context,
    menu: &mut Menu<'_>,
    reaction: Reaction,
) -> SerenityUtilsResult<()> {
    let guild_id = reaction.guild_id.unwrap();
    let user = reaction.user(&ctx).await?;

    if !is_dj(ctx, guild_id, &user).await? {
        return Ok(());
    }
    let selected_band = menu.data.get::<SelectedBand>().unwrap();
    if selected_band.load(Ordering::SeqCst) <= 0 {
        selected_band.store(14, Ordering::SeqCst);
    } else {
        selected_band.fetch_sub(1, Ordering::SeqCst);
    }
    display_page(ctx, menu).await?;

    Ok(())
}

/// Adds to the selected band
async fn add_to_band(
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
        let selected_band = menu
            .data
            .get::<SelectedBand>()
            .unwrap()
            .load(Ordering::Relaxed);
        let player = menu.data.get::<Player>().unwrap();
        let mut player = player.lock().await;
        let equalizer = player.get_equalizer();
        let current_value = equalizer[selected_band as usize];

        if current_value < 0.25 {
            player.equalize(selected_band, current_value + 0.05).await?;
        }
    }

    display_page(ctx, menu).await?;

    Ok(())
}

/// Substracts from the selected band
async fn subtract_from_band(
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
        let selected_band = menu
            .data
            .get::<SelectedBand>()
            .unwrap()
            .load(Ordering::Relaxed);
        let player = menu.data.get::<Player>().unwrap();
        let mut player = player.lock().await;
        let equalizer = player.get_equalizer();
        let current_value = equalizer[selected_band as usize];

        if current_value > -0.25 {
            player.equalize(selected_band, current_value - 0.05).await?;
        }
    }

    display_page(ctx, menu).await?;

    Ok(())
}

/// Deletes the menu
async fn delete_menu(
    ctx: &Context,
    menu: &mut Menu<'_>,
    reaction: Reaction,
) -> SerenityUtilsResult<()> {
    let guild_id = reaction.guild_id.unwrap();
    let user = reaction.user(&ctx).await?;

    if !is_dj(ctx, guild_id, &user).await? {
        return Ok(());
    }
    let handle = menu.message.read().await;
    ctx.http
        .delete_message(handle.channel_id, handle.message_id)
        .await?;

    Ok(())
}
