use crate::utils::error::{BotError, BotResult};
use regex::Regex;
use std::io;
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

/// Asynchronously runs a given command and returns the output
pub async fn run_command_async(command: &str, args: &[&str]) -> io::Result<String> {
    log::trace!("Running command '{}' with args {:?}", command, args);
    let cmd = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stderr = String::new();
    let mut output = String::new();
    cmd.stderr.unwrap().read_to_string(&mut stderr).await?;
    if stderr.len() != 0 {
        log::debug!("STDERR of command {}: {}", command, stderr);
    }
    cmd.stdout.unwrap().read_to_string(&mut output).await?;
    log::trace!("Command output is {}", output);

    Ok(output)
}

/// Sanitizes a command line argument and throws an error
/// on a possible injection attempt
pub fn sanitize_argument(arg: &str, detect_help: bool) -> BotResult<String> {
    log::debug!("Sanitizing argument '{}'", arg);
    lazy_static::lazy_static! {
        static ref HELP_FLAG: Regex = Regex::new(r"^\s*(-*)h(elp)?\s*$").unwrap();
        static ref FLAG_REGEX: Regex = Regex::new(r"^\s*-\w*\s*$").unwrap();
    }
    if FLAG_REGEX.is_match(arg) {
        log::debug!("Detected STDIN injection");
        return Err(BotError::CliInject);
    }
    if detect_help && HELP_FLAG.is_match(arg) {
        log::debug!("Detected help injection");
        return Err(BotError::CliInject);
    }
    let arg = arg.replace("--", "\\-\\-");
    if arg.is_empty() {
        return Err(BotError::CliInject);
    }
    log::debug!("Sanitized argument is '{}'", arg);

    Ok(arg)
}
