use crate::menu::traits::EventDrivenMessage;
use serenity::model::id::{ChannelId, MessageId};

pub type BoxedEventDrivenMessage = Box<dyn EventDrivenMessage>;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct MessageHandle {
    pub channel_id: u64,
    pub message_id: u64,
}

impl MessageHandle {
    /// Creates a new message handle
    pub fn new(channel_id: ChannelId, message_id: MessageId) -> Self {
        Self {
            message_id: message_id.0,
            channel_id: channel_id.0,
        }
    }

    /// Creates a new message handle from raw ids
    pub fn from_raw_ids(channel_id: u64, message_id: u64) -> Self {
        Self {
            message_id,
            channel_id,
        }
    }
}
