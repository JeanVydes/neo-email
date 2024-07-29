use crate::{command::Commands, connection::SMTPConnection, message::Message};
use core::fmt;
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::Mutex;

/// # OnUnknownCommandController
///
/// This struct represents a controller that is called when auth command is received.
#[derive(Clone)]
pub struct OnUnknownCommandController<B>(
    pub  Arc<
        dyn Fn(
                Arc<Mutex<SMTPConnection<B>>>,
                Commands,
            ) -> Pin<Box<dyn Future<Output = Result<Message, Message>> + Send>>
            + Send
            + Sync
            + 'static,
    >,
);

impl<B> OnUnknownCommandController<B> {
    /// # New
    ///
    /// This function creates a new OnUnknownCommandController.
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn(Arc<Mutex<SMTPConnection<B>>>, Commands) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Message, Message>> + Send + 'static,
    {
        let wrapped_fn = move |conn: Arc<Mutex<SMTPConnection<B>>>, data: Commands| {
            Box::pin(f(conn, data))
                as Pin<Box<dyn Future<Output = Result<Message, Message>> + Send>>
        };

        OnUnknownCommandController(Arc::new(wrapped_fn))
    }
}

impl<B> fmt::Debug for OnUnknownCommandController<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure")
    }
}
