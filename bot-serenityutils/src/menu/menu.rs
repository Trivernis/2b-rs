use crate::core::MessageHandle;
use crate::error::{SerenityUtilsError, SerenityUtilsResult};
use crate::menu::container::get_listeners_from_context;
use crate::menu::controls::{close_menu, next_page, previous_page};
use crate::menu::traits::EventDrivenMessage;
use crate::menu::{EventDrivenMessagesRef, Page};
use futures::FutureExt;
use serenity::async_trait;
use serenity::client::Context;
use serenity::http::Http;
use serenity::model::channel::{Message, Reaction, ReactionType};
use serenity::model::id::ChannelId;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};

pub static NEXT_PAGE_EMOJI: &str = "➡️";
pub static PREVIOUS_PAGE_EMOJI: &str = "⬅️";
pub static CLOSE_MENU_EMOJI: &str = "❌";

pub type ControlActionResult<'b> =
    Pin<Box<dyn Future<Output = SerenityUtilsResult<()>> + Send + 'b>>;

pub type ControlActionArc = Arc<
    dyn for<'b> Fn(&'b Context, &'b mut Menu<'_>, Reaction) -> ControlActionResult<'b>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub struct ActionContainer {
    inner: ControlActionArc,
    position: usize,
}

impl ActionContainer {
    /// Creates a new control action
    pub fn new<F: 'static>(position: usize, callback: F) -> Self
    where
        F: for<'b> Fn(&'b Context, &'b mut Menu<'_>, Reaction) -> ControlActionResult<'b>
            + Send
            + Sync,
    {
        Self {
            inner: Arc::new(callback),
            position,
        }
    }

    /// Runs the action
    pub async fn run(
        &self,
        ctx: &Context,
        menu: &mut Menu<'_>,
        reaction: Reaction,
    ) -> SerenityUtilsResult<()> {
        self.inner.clone()(ctx, menu, reaction).await?;
        Ok(())
    }
}

/// A menu message
#[derive(Clone)]
pub struct Menu<'a> {
    pub message: Arc<RwLock<MessageHandle>>,
    pub pages: Vec<Page<'a>>,
    pub current_page: usize,
    pub controls: HashMap<String, ActionContainer>,
    pub timeout: Instant,
    pub sticky: bool,
    closed: bool,
    listeners: EventDrivenMessagesRef,
}

impl Menu<'_> {
    /// Removes all reactions from the menu
    pub(crate) async fn close(&mut self, http: &Http) -> SerenityUtilsResult<()> {
        log::debug!("Closing menu...");
        let handle = self.message.read().await;
        http.delete_message_reactions(handle.channel_id, handle.message_id)
            .await?;
        self.closed = true;
        Ok(())
    }

    /// Returns the message of the menu
    pub async fn get_message(&self, http: &Http) -> SerenityUtilsResult<Message> {
        let handle = self.message.read().await;
        let msg = http
            .get_message(handle.channel_id, handle.message_id)
            .await?;

        Ok(msg)
    }

    /// Recreates the message completely
    pub async fn recreate(&self, http: &Http) -> SerenityUtilsResult<()> {
        log::debug!("Recreating message");

        let old_handle = {
            let handle = self.message.read().await;
            (*handle).clone()
        };
        log::debug!("Getting current page");
        let current_page = self
            .pages
            .get(self.current_page)
            .cloned()
            .ok_or(SerenityUtilsError::PageNotFound(self.current_page))?
            .get()
            .await?;

        log::debug!("Creating new message");
        let message = http
            .send_message(
                old_handle.channel_id,
                &serde_json::to_value(current_page.0).unwrap(),
            )
            .await?;
        let mut controls = self
            .controls
            .clone()
            .into_iter()
            .collect::<Vec<(String, ActionContainer)>>();
        controls.sort_by_key(|(_, a)| a.position);

        for emoji in controls.into_iter().map(|(e, _)| e) {
            http.create_reaction(
                message.channel_id.0,
                message.id.0,
                &ReactionType::Unicode(emoji.clone()),
            )
            .await?;
        }
        log::trace!("New message is {:?}", message);

        let new_handle = {
            let mut handle = self.message.write().await;
            handle.message_id = message.id.0;
            (*handle).clone()
        };
        {
            log::debug!("Changing key of message");
            let mut listeners_lock = self.listeners.lock().await;
            let menu = listeners_lock.remove(&old_handle).unwrap();
            listeners_lock.insert(new_handle, menu);
        }
        log::debug!("Deleting original message");
        http.delete_message(old_handle.channel_id, old_handle.message_id)
            .await?;
        log::debug!("Message recreated");

        Ok(())
    }
}

#[async_trait]
impl<'a> EventDrivenMessage for Menu<'a> {
    fn is_frozen(&self) -> bool {
        self.closed
    }

    async fn update(&mut self, http: &Http) -> SerenityUtilsResult<()> {
        log::trace!("Checking for menu timeout");
        if Instant::now() >= self.timeout {
            log::debug!("Menu timout reached. Closing menu.");
            self.close(http).await?;
        } else if self.sticky {
            log::debug!("Message is sticky. Checking for new messages in channel...");
            let handle = {
                let handle = self.message.read().await;
                (*handle).clone()
            };

            let channel_id = ChannelId(handle.channel_id);
            let messages = channel_id
                .messages(http, |p| p.after(handle.message_id).limit(1))
                .await?;
            log::trace!("Messages are {:?}", messages);
            if messages.len() > 0 {
                log::debug!("New messages in channel. Recreating...");
                self.recreate(http).await?;
            }
        }

        Ok(())
    }

    async fn on_deleted(&mut self, _: &Context) -> SerenityUtilsResult<()> {
        Ok(())
    }

    async fn on_reaction_add(
        &mut self,
        ctx: &Context,
        reaction: Reaction,
    ) -> SerenityUtilsResult<()> {
        log::debug!("Reaction to menu added");
        let current_user = ctx.http.get_current_user().await?;

        if reaction.user_id.unwrap().0 == current_user.id.0 {
            log::debug!("Reaction is from current user.");
            return Ok(());
        }
        let emoji_string = reaction.emoji.as_data();

        log::debug!("Deleting user reaction.");
        reaction.delete(ctx).await?;
        if let Some(control) = self.controls.get(&emoji_string).cloned() {
            log::debug!("Running control");
            control.run(ctx, self, reaction).await?;
        }

        Ok(())
    }

    async fn on_reaction_remove(&mut self, _: &Context, _: Reaction) -> SerenityUtilsResult<()> {
        Ok(())
    }
}

/// A builder for messages
pub struct MenuBuilder {
    pages: Vec<Page<'static>>,
    current_page: usize,
    controls: HashMap<String, ActionContainer>,
    timeout: Duration,
    sticky: bool,
}

impl Default for MenuBuilder {
    fn default() -> Self {
        Self {
            pages: vec![],
            current_page: 0,
            controls: HashMap::new(),
            timeout: Duration::from_secs(60),
            sticky: false,
        }
    }
}

impl MenuBuilder {
    /// Creates a new paginaton menu
    pub fn new_paginator() -> Self {
        log::debug!("Creating new paginator");
        let mut controls = HashMap::new();
        controls.insert(
            PREVIOUS_PAGE_EMOJI.to_string(),
            ActionContainer::new(0, |c, m, r| previous_page(c, m, r).boxed()),
        );
        controls.insert(
            CLOSE_MENU_EMOJI.to_string(),
            ActionContainer::new(1, |c, m, r| close_menu(c, m, r).boxed()),
        );
        controls.insert(
            NEXT_PAGE_EMOJI.to_string(),
            ActionContainer::new(2, |c, m, r| next_page(c, m, r).boxed()),
        );

        Self {
            controls,
            ..Default::default()
        }
    }

    /// Adds a page to the message builder
    pub fn add_page(mut self, page: Page<'static>) -> Self {
        self.pages.push(page);

        self
    }

    /// Adds multiple pages to the message
    pub fn add_pages<I>(mut self, pages: I) -> Self
    where
        I: IntoIterator<Item = Page<'static>>,
    {
        let mut pages = pages.into_iter().collect();
        self.pages.append(&mut pages);

        self
    }

    /// Adds a single control to the message
    pub fn add_control<S, F: 'static>(mut self, position: usize, emoji: S, action: F) -> Self
    where
        S: ToString,
        F: for<'b> Fn(&'b Context, &'b mut Menu<'_>, Reaction) -> ControlActionResult<'b>
            + Send
            + Sync,
    {
        self.controls
            .insert(emoji.to_string(), ActionContainer::new(position, action));

        self
    }

    /// Adds a single control to the message
    pub fn add_controls<S, I>(mut self, controls: I) -> Self
    where
        S: ToString,
        I: IntoIterator<Item = (usize, S, ControlActionArc)>,
    {
        for (position, emoji, action) in controls {
            self.controls.insert(
                emoji.to_string(),
                ActionContainer {
                    position,
                    inner: action,
                },
            );
        }

        self
    }

    /// Sets the timeout for the message
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;

        self
    }

    /// Sets the start page of the message
    pub fn start_page(mut self, page: usize) -> Self {
        self.current_page = page;

        self
    }

    /// If the message should be sticky and always be
    /// the last one in the channel
    pub fn sticky(mut self, value: bool) -> Self {
        self.sticky = value;

        self
    }

    /// builds the menu
    pub async fn build(
        self,
        ctx: &Context,
        channel_id: ChannelId,
    ) -> SerenityUtilsResult<Arc<RwLock<MessageHandle>>> {
        log::debug!("Building menu...");
        let mut current_page = self
            .pages
            .get(self.current_page)
            .ok_or(SerenityUtilsError::PageNotFound(self.current_page))?
            .clone()
            .get()
            .await?;

        let message = channel_id.send_message(ctx, |_| &mut current_page).await?;
        log::trace!("Message is {:?}", message);
        let listeners = get_listeners_from_context(ctx).await?;
        log::debug!("Sorting controls...");
        let mut controls = self
            .controls
            .clone()
            .into_iter()
            .collect::<Vec<(String, ActionContainer)>>();
        controls.sort_by_key(|(_, a)| a.position);

        log::debug!("Creating menu...");
        let message_handle = MessageHandle::new(message.channel_id, message.id);
        let handle_lock = Arc::new(RwLock::new(message_handle));

        let menu = Menu {
            message: Arc::clone(&handle_lock),
            pages: self.pages,
            current_page: self.current_page,
            controls: self.controls,
            timeout: Instant::now() + self.timeout,
            closed: false,
            listeners: Arc::clone(&listeners),
            sticky: self.sticky,
        };

        log::debug!("Storing menu to listeners...");
        {
            let mut listeners_lock = listeners.lock().await;
            log::trace!("Listeners locked.");
            listeners_lock.insert(message_handle, Arc::new(Mutex::new(Box::new(menu))));
        }

        log::debug!("Adding controls...");
        for (emoji, _) in controls {
            message
                .react(ctx, ReactionType::Unicode(emoji.clone()))
                .await?;
        }
        log::debug!("Menu successfully created.");

        Ok(handle_lock)
    }
}
