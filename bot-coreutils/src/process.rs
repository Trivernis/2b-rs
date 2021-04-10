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
