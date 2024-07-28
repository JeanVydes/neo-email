use core::fmt;
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::Mutex;
use crate::{connection::SMTPConnection, message::Message};

/// # OnAuthController
///
/// This struct represents a controller that is called when auth command is received.
#[derive(Clone)]
pub struct OnAuthController<B>(
    pub Arc<dyn Fn(Arc<Mutex<SMTPConnection<B>>>, String) -> Pin<Box<dyn Future<Output = Result<Message, Message>> + Send>> + Send + Sync + 'static>,
);

impl<B> OnAuthController<B> {
    /// # New
    ///
    /// This function creates a new OnAuthController.
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn(Arc<Mutex<SMTPConnection<B>>>, String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Message, Message>> + Send + 'static,
    {
        let wrapped_fn = move |conn: Arc<Mutex<SMTPConnection<B>>>, data: String| {
            Box::pin(f(conn, data)) as Pin<Box<dyn Future<Output = Result<Message, Message>> + Send>>
        };

        OnAuthController(Arc::new(wrapped_fn))
    }
}

impl<B> fmt::Debug for OnAuthController<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure")
    }
}
