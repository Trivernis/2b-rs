use crate::utils::error::BotResult;
use crate::utils::process::{run_command_async, sanitize_argument};

/// Runs the qalc command with the given expression
pub async fn qalc(expression: &str) -> BotResult<String> {
    let expression = sanitize_argument(expression, true)?;
    let result = run_command_async("qalc", &[&*expression]).await?;
    Ok(result)
}
