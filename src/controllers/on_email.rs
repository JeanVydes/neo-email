use core::fmt;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::Mutex;

use crate::{connection::SMTPConnection, mail::{Mail, MailTrait}, message::Message};

/// # OnEmailController
/// 
/// This struct represents a controller that is called when an email is received.
#[derive(Clone)]
pub struct OnEmailController<B>(
    pub Arc<
        dyn Fn(Arc<Mutex<SMTPConnection<B>>>, Box<dyn MailTrait>) -> Pin<Box<dyn Future<Output = Message> + Send>> + Send + Sync + 'static,
    >,
);

impl<B> OnEmailController<B> {
    /// # New
    /// 
    /// This function creates a new OnEmailController.
    pub fn new<F, T, Fut>(f: F) -> Self
    where
        F: Fn(Arc<Mutex<SMTPConnection<B>>>, Mail<T>) -> Fut + Send + Sync + 'static,
        T: 'static + Clone + Send + Sync,
        Fut: Future<Output = Message> + Send + 'static,
    {
        let wrapped_fn = move |conn: Arc<Mutex<SMTPConnection<B>>>, mail_trait: Box<dyn MailTrait>| {
            let mail = mail_trait
                .as_any()
                .downcast_ref::<Mail<T>>()
                .expect("Invalid type");
            Box::pin(f(conn, mail.clone())) as Pin<Box<dyn Future<Output = Message> + Send>>
        };

        OnEmailController(Arc::new(wrapped_fn))
    }
}

impl<B> fmt::Debug for OnEmailController<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure")
    }
}