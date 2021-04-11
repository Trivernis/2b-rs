use crate::menu::traits::EventDrivenMessage;

pub type MessageHandle = (u64, u64);
pub type BoxedEventDrivenMessage = Box<dyn EventDrivenMessage>;
