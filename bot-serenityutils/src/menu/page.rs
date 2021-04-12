use crate::error::SerenityUtilsResult;
use serenity::builder::CreateMessage;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type MessageBuildOutput<'b> =
    Pin<Box<dyn Future<Output = SerenityUtilsResult<CreateMessage<'b>>> + Send + 'b>>;
pub type MessageBuilderFn<'b> = Arc<dyn Fn() -> MessageBuildOutput<'b> + Send + Sync>;

#[derive(Clone)]
/// A page that stores a builder function for message pages
/// or static pages
pub enum Page<'b> {
    Builder(MessageBuilderFn<'b>),
    Static(CreateMessage<'b>),
}

impl<'b> Page<'b> {
    /// Creates a new page with the given builder function
    pub fn new_builder<F: 'static>(builder_fn: F) -> Self
    where
        F: Fn() -> MessageBuildOutput<'b> + Send + Sync,
    {
        Self::Builder(Arc::new(builder_fn))
    }

    /// Creates a new page with a static message
    pub fn new_static(page: CreateMessage<'b>) -> Self {
        Self::Static(page)
    }

    /// Returns the CreateMessage of the page
    pub async fn get(&self) -> SerenityUtilsResult<CreateMessage<'b>> {
        match self {
            Page::Builder(b) => b().await,
            Page::Static(inner) => Ok(inner.clone()),
        }
    }
}
