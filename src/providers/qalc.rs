use crate::utils::error::BotResult;
use bot_coreutils::process::run_command_async;

/// Runs the qalc command with the given expression
pub async fn qalc(expression: &str) -> BotResult<String> {
    let result = run_command_async(
        "qalc",
        &["-m", "1000", format!("\"{}\"", &*expression).as_str()],
    )
    .await?;
    Ok(result)
}
