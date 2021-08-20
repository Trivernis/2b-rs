use crate::utils::error::BotResult;

static INSPIROBOT_ENDPOINT: &str = "https://inspirobot.me/api?generate=true";

pub async fn get_inspirobot_image() -> BotResult<String> {
    let response = reqwest::get(INSPIROBOT_ENDPOINT).await?;
    let url = response.text().await?;

    Ok(url)
}
