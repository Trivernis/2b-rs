use std::collections::VecDeque;

use rand::Rng;
use std::io;
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

pub(crate) mod context_data;
pub(crate) mod error;
pub(crate) mod logging;
pub(crate) mod messages;

/// Fisher-Yates shuffle for VecDeque
pub fn shuffle_vec_deque<T>(deque: &mut VecDeque<T>) {
    let mut rng = rand::thread_rng();
    let mut i = deque.len();
    while i >= 2 {
        i -= 1;
        deque.swap(i, rng.gen_range(0..i + 1))
    }
}

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
