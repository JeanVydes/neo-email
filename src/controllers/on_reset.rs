use core::fmt;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::connection::SMTPConnection;

/// # OnResetController
///
/// This struct represents a controller that is called when an connection is reset.
#[derive(Clone)]
pub struct OnResetController<B>(
    pub Arc<dyn Fn(Arc<Mutex<SMTPConnection<B>>>) -> () + Send + Sync + 'static>,
);

impl <B> OnResetController<B> {
    /// # New
    ///
    /// This function creates a new OnResetController.
    pub fn new<F, T>(f: F) -> Self
    where
        F: Fn(Arc<Mutex<SMTPConnection<B>>>) -> () + Send + Sync + 'static,
        T: 'static + Clone + Send + Sync,
    {
        let wrapped_fn = move |conn: Arc<Mutex<SMTPConnection<B>>>| f(conn);

        OnResetController(Arc::new(wrapped_fn))
    }
}

impl <B> fmt::Debug for OnResetController<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure")
    }
}
