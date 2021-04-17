use serenity::prelude::TypeMapKey;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct HelpActiveContainer;

impl TypeMapKey for HelpActiveContainer {
    type Value = Arc<AtomicBool>;
}
