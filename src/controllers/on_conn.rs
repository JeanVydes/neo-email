use core::fmt;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::connection::SMTPConnection;

/// # OnConnController
///
/// This struct represents a controller that is called when an connection is opened.
#[derive(Clone)]
pub struct OnConnController<B>(
    pub Arc<dyn Fn(Arc<Mutex<SMTPConnection<B>>>) -> () + Send + Sync + 'static>,
);

impl<B> OnConnController<B> {
    /// # New
    ///
    /// This function creates a new OnConnController.
    pub fn new<F, T>(f: F) -> Self
    where
        F: Fn(Arc<Mutex<SMTPConnection<B>>>) -> () + Send + Sync + 'static,
        T: 'static + Clone + Send + Sync,
    {
        let wrapped_fn = move |conn: Arc<Mutex<SMTPConnection<B>>>| f(conn);

        OnConnController(Arc::new(wrapped_fn))
    }
}

impl<B> fmt::Debug for OnConnController<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure")
    }
}
