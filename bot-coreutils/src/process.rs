use std::io;
use tokio::process::Command;

/// Asynchronously runs a given command and returns the output
pub async fn run_command_async(command: &str, args: &[&str]) -> io::Result<String> {
    log::trace!("Running command '{}' with args {:?}", command, args);
    let process_output: std::process::Output = Command::new(command).args(args).output().await?;

    log::trace!("Reading from stderr...");
    let stderr = String::from_utf8_lossy(&process_output.stderr[..]);
    let stdout = String::from_utf8_lossy(&process_output.stdout[..]);

    if stderr.len() != 0 {
        log::debug!("STDERR of command {}: {}", command, stderr);
    }
    log::trace!("Command output is {}", stdout);

    Ok(stdout.to_string())
}
