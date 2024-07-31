#![deny(missing_docs)]
#![deny(warnings)]
#![deny(unused_imports)]
#![deny(unused_must_use)]
#![deny(unused_variables)]
#![deny(unused_mut)]

//! # Neo Email
//! 
//! `neo-email` is a library for build email services in a modern and safe way.
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use std::net::SocketAddr;
//! use std::sync::Arc;
//! use neo_email::connection::SMTPConnection;
//! use neo_email::controllers::on_auth::OnAuthController;
//! use neo_email::controllers::on_email::OnEmailController;
//! use neo_email::headers::EmailHeaders;
//! use neo_email::mail::Mail;
//! use neo_email::message::Message;
//! use neo_email::server::SMTPServer;
//! use neo_email::status_code::StatusCodes;
//!
//! use tokio::sync::Mutex;
//!
//! #[derive(Debug, Clone, Default)]
//! pub struct ConnectionState {
//!    pub authenticated: bool,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!    let addr = SocketAddr::from(([127, 0, 0, 1], 2526));
//!    // Create the server
//!    SMTPServer::<ConnectionState>::new()
//!        // Set the number of workers to 1
//!        .workers(1)
//!        // Set an controller to dispatch when an authentication is received
//!        .on_auth(OnAuthController::new(on_auth))
//!        // Set an controller to dispatch when an email is received
//!        .on_email(OnEmailController::new(on_email))
//!        // Bind the server to the address
//!        .bind(addr)
//!        .await
//!        .unwrap()
//!        // Run the server
//!        .run()
//!        .await;
//! }
//!
//! // This function is called when an authentication is received
//! // Ok(Message) for successful authentication
//! // Err(Message) for failed authentication and the connection will be closed peacefully
//! pub async fn on_auth(conn: Arc<Mutex<SMTPConnection<ConnectionState>>>, _data: String) -> Result<Message, Message> {
//!    let conn = conn.lock().await;
//!    let mut state = conn.state.lock().await;
//!
//!    // What is data?
//!    // Data is the raw data after command AUTH, example
//!    // Original Raw Command: AUTH PLAIN AHlvdXJfdXNlcm5hbWUAeW91cl9wYXNzd29yZA==
//!    // Data: PLAIN AHlvdXJfdXNlcm5hbWUAeW91cl9wYXNzd29yZA==
//!
//!    // Using our custom state
//!    state.authenticated = true;
//!    // We can also decide to not authenticate the user
//!
//!    Ok(Message::builder()
//!        .status(neo_email::status_code::StatusCodes::AuthenticationSuccessful)
//!        .message("Authenticated".to_string())
//!        .build())
//! }
//! 
//! // This function is called when an email is received
//! // The mail is a struct that contains the email data, in this case the raw email data in a Vec<u8>
//! // Headers are parsed in a hashmap and the body is a Vec<u8>
//! pub async fn on_email(conn: Arc<Mutex<SMTPConnection<ConnectionState>>>, mail: Mail<Vec<u8>>) -> Message {
//!    let conn = conn.lock().await;
//!    let state = conn.state.lock().await;
//!
//!    // Extract headers
//!    let headers = mail.headers.clone(); // get the hashmap
//!    let _subject = headers.get(&EmailHeaders::Subject).unwrap(); // get the Option<Subject> header
//!
//!    // Check if the user is authenticated from state set in on_auth
//!    if !state.authenticated {
//!        return Message::builder()
//!            .status(StatusCodes::AuthenticationCredetialsInvalid)
//!            .message("Authentication required".to_string())
//!            .build();
//!    }
//!
//!    log::info!("Received email: {:?}", mail);
//!    
//!    Message::builder()
//!        .status(neo_email::status_code::StatusCodes::OK)
//!        .message("Email received".to_string())
//!        .build()
//! }
//! ```
//! 
//! ## Features
//! 
//! - Modern and safe
//! - Easy to use
//! - Customizable
//! - Async
//! - Multi-threaded
//! - Custom controllers
//! - Custom states
//! 
//! ## Features Flags
//! 
//! - `smtp-experimental-headers` - Enable experimental mail headers feature
//! - `smtp-experimental` - Enable SMTP experimental features (includes `smtp-experimental-headers`)
//! - `spf-experimental` - Enable Sender Policy Framework experimental features
//! - `dkim-experimental` - Enable DomainKeys Identified Mail experimental features (includes `sha1`, `sha2`, `base64`)` (NOT AVAILABLE)
//! - `utilities-experimental` - Enable utilities experimental features (includes `spf-experimental` and `dkim-experimental`)
//! - `experimental` - Enable all experimental features (includes `utilities-experimental`)
//! 
//! ## License
//! 
//! Licensed under the MIT license. See LICENSE for more information.
//! 

/// # Client Message
pub mod client_message;
/// # Command
pub mod command;
/// # Connection
pub mod connection;
/// # Controllers
pub mod controllers;
/// # Errors
pub mod errors;
/// # Handle Connection
pub mod handle_connection;
/// # Headers
/// 
/// This module contains the headers for the email, this headers are used to parse the email headers.
/// 
/// ## Example
/// 
/// ```rust,no_run
/// use neo_email::mail::Mail;
/// use neo_email::headers::EmailHeaders;
/// 
/// let raw_email = b"From: Jean <jean@nervio.com>\nSubject: Hello\n\nHello, World!";
/// let mail = Mail::<Vec<u8>>::from_bytes(raw_email.to_vec()).unwrap();
/// let subject = mail.headers.get(&EmailHeaders::Subject).unwrap();
pub mod headers;
/// # Mail
/// 
/// This module contains the mail object, that is divided in two parts, Headers that is a HashMap of provided EmailHeaders->RawHeader and the body that is a T, and commonly used as Vec<u8>.
/// 
/// ## Example
/// 
/// ```rust,no_run
/// use neo_email::mail::Mail;
/// 
/// let raw_email = b"From: Jean <jean@nervio.com>\nSubject: Hello\n\nHello, World!";
/// let mail = Mail::<Vec<u8>>::from_bytes(raw_email.to_vec()).unwrap();
pub mod mail;
/// # Message
/// 
/// This module contains the message struct, this struct is used to send messages to the client.
/// 
/// ## Example
/// 
/// ```rust,no_run
/// use neo_email::message::Message;
/// use neo_email::status_code::StatusCodes;
/// 
/// let message = Message::builder()
///     .status(StatusCodes::OK)
///     .message("OK".to_string())
///     .build();
pub mod message;
/// # Server
/// 
/// This module contains the SMTP server, from this you can create a fully customizable SMTP server with Commands, Controllers, States and more.
/// 
/// ## Example
/// 
/// ```rust,no_run
/// use std::net::SocketAddr;
/// use neo_email::server::SMTPServer;
/// 
/// #[tokio::main]
/// async fn main() {
///     let addr = SocketAddr::from(([127, 0, 0, 1], 2526));
///     // Create the server
///     SMTPServer::new()
///         // Set the number of workers to 1
///         .workers(1)
///         // Bind the server to the address
///         .bind(addr)
///         .await
///         .unwrap()
///         // Run the server
///         .run()
///         .await;
/// }
pub mod server;
/// # Status Code
/// 
/// This module contains the status codes for the SMTP server.
/// 
/// ## Example
/// 
/// ```rust,no_run
/// use neo_email::status_code::StatusCodes;
/// use neo_email::message::Message;
/// 
/// let message = Message::builder()
///     .status(StatusCodes::OK)
///     .message("OK".to_string())
///     .build();
/// ```
pub mod status_code;
/// # Utilities
/// 
/// This module contains utilities for the SMTP server for example SPF, DKIM and DMARC
pub mod utilities;