# NEO EMAIL

[![crates.io](https://img.shields.io/crates/v/neo-email.svg)](https://crates.io/crates/neo-email)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
![Downloads](https://img.shields.io/crates/d/neo-email)
![GitHub Repo stars](https://img.shields.io/github/stars/JeanVydes/neo-email)

Neo Email is a cutting-edge Rust crate designed for modern email handling, focusing on robust and secure email systems. It provides comprehensive support for crafting, sending, and validating emails, integrating the latest standards and practices in email technology.

## Install

Use terminal with Cargo

```bash
cargo add neo-email
```

or add to your Cargo.toml

```toml
neo-email = { version = "0.1", features = ["experimental"] }
```

## Features

* Easy and Fast to implement
* Built on top of Tokio
* Built-in Utilities like SPF & DKIM (Experimental)

## Examples

```rust
// # Example
// Simple SMTP Example using custom state and some controllers
// See examples/Full.md for a more complete example using more controllers and setting up a working server

use std::net::SocketAddr;
use std::sync::Arc;

use neo_email::connection::SMTPConnection;
use neo_email::controllers::on_auth::OnAuthController;
use neo_email::controllers::on_email::OnEmailController;
use neo_email::headers::EmailHeaders;
use neo_email::mail::Mail;
use neo_email::message::Message;
use neo_email::server::SMTPServer;
use neo_email::status_code::StatusCodes;

use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
pub struct ConnectionState {
    pub authenticated: bool,
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 2526));
    // Create the server
    SMTPServer::<ConnectionState>::new()
        // Set the number of workers to 1
        .workers(1)
        // Set an controller to dispatch when an authentication is received
        .on_auth(OnAuthController::new(on_auth))
        // Set an controller to dispatch when an email is received
        .on_email(OnEmailController::new(on_email))
        // Bind the server to the address
        .bind(addr)
        .await
        .unwrap()
        // Run the server
        .run()
        .await;
}

// This function is called when an authentication is received
// Ok(Message) for successful authentication
// Err(Message) for failed authentication and the connection will be closed peacefully
pub async fn on_auth(conn: Arc<Mutex<SMTPConnection<ConnectionState>>>, _data: String) -> Result<Message, Message> {
    let conn = conn.lock().await;
    let mut state = conn.state.lock().await;

    // What is data?
    // Data is the raw data after command AUTH, example
    // Original Raw Command: AUTH PLAIN AHlvdXJfdXNlcm5hbWUAeW91cl9wYXNzd29yZA==
    // Data: PLAIN AHlvdXJfdXNlcm5hbWUAeW91cl9wYXNzd29yZA==

    // Using our custom state
    state.authenticated = true;
    // We can also decide to not authenticate the user

    Ok(Message::builder()
        .status(neo_email::status_code::StatusCodes::AuthenticationSuccessful)
        .message("Authenticated".to_string())
        .build())
}

// This function is called when an email is received
// The mail is a struct that contains the email data, in this case the raw email data in a Vec<u8>
// Headers are parsed in a hashmap and the body is a Vec<u8>
pub async fn on_email(conn: Arc<Mutex<SMTPConnection<ConnectionState>>>, mail: Mail<Vec<u8>>) -> Message {
    let conn = conn.lock().await;
    let state = conn.state.lock().await;

    // Extract headers
    let headers = mail.headers.clone(); // get the hashmap
    let _subject = headers.get(&EmailHeaders::Subject).unwrap(); // get the Option<Subject> header

    // Check if the user is authenticated from state set in on_auth
    if !state.authenticated {
        return Message::builder()
            .status(StatusCodes::AuthenticationCredetialsInvalid)
            .message("Authentication required".to_string())
            .build();
    }

    log::info!("Received email: {:?}", mail);
    
    Message::builder()
        .status(neo_email::status_code::StatusCodes::OK)
        .message("Email received".to_string())
        .build()
}
```

### More Examples

Check out [`examples/`](https://github.com/JeanVydes/neo-email/tree/main/examples) for examples

## Authors

* [Jean Vides](https://github.com/JeanVydes)
* You can be here ;)

## Sponsors

Nothing here :<

## Collaboration

Feel free to collaborate to this project