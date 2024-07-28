# NEO EMAIL

The modern way to build emails services with Rust

## Features

* Easy and Fast to implement
* Built on top of Tokio
* Customization
* Like build a HTTP API
* Open Source

## Examples

```rust
use std::{net::SocketAddr, sync::Arc};

use neo_email::{
    connection::SMTPConnection,
    controllers::{on_auth::OnAuthController, on_email::OnEmailController},
    headers::EmailHeaders,
    mail::Mail,
    message::Message,
    server::SMTPServer,
    status_code::StatusCodes,
};
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
// What is data?
// This send the client: `AUTH PLAIN AHlvdXJfdXNlcm5hbWUAeW91cl9wYXNzd29yZA==`
// data is `PLAIN AHlvdXJfdXNlcm5hbWUAeW91cl9wYXNzd29yZA==`, just without the command AUTH, which is stripped by the server, you have to handle the command in your controller
// Ok(Message) for successful authentication
// Err(Message) for failed authentication and the connection will be closed peacefully
pub async fn on_auth(
    conn: Arc<Mutex<SMTPConnection<ConnectionState>>>,
    _data: String,
) -> Result<Message, Message> {
    let conn = conn.lock().await;
    let mut state = conn.state.lock().await;

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
pub async fn on_email(
    conn: Arc<Mutex<SMTPConnection<ConnectionState>>>,
    mail: Mail<Vec<u8>>,
) -> Message {
    let conn = conn.lock().await;
    let state = conn.state.lock().await;

    let headers = mail.headers.clone(); // get the hashmap
    let subject = headers.get(&EmailHeaders::Subject).unwrap(); // get the Option<Subject> header
    println!("Subject: {:?}", subject);

    // Check if the user is authenticated from state set in on_auth
    if !state.authenticated {
        return Message::builder()
            .status(StatusCodes::AuthenticationCredetialsInvalid)
            .message("Authentication required".to_string())
            .build();
    }

    Message::builder()
        .status(neo_email::status_code::StatusCodes::OK)
        .message("Email received".to_string())
        .build()
}
```

### More Examples

Check out [`examples/`](https://github.com/JeanVydes/neo-email/tree/main/examples) for examples

## Sponsors

Nothing here :<

## Collaboration

Feel free to collaborate to this project