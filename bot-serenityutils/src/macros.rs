/// Forwards the error directly to the user
/// without having to accept it in any handler.
/// Can only be used in async functions that return a Result.
#[macro_export]
macro_rules! forward_error {
    ($ctx:expr,$channel_id:expr,$result:expr) => {
        match $result {
            Err(e) => {
                use bot_serenityutils::{core::SHORT_TIMEOUT, ephemeral_message::EphemeralMessage};
                $channel_id.say($ctx, format!("‼️ {}", e)).await?;
                return Ok(());
            }
            Ok(v) => v,
        }
    };
}
