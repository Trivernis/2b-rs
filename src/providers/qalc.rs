use crate::utils::error::BotResult;
use crate::utils::run_command_async;

/// Runs the qalc command with the given expression
pub async fn qalc(expression: &str) -> BotResult<String> {
    let result = run_command_async("qalc", &[expression]).await?;
    Ok(result)
}
