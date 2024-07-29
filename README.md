# NEO EMAIL

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
use std::{net::SocketAddr, sync::Arc};

use neo_email::{
    connection::SMTPConnection,
    controllers::{on_auth::OnAuthController, on_email::OnEmailController, on_mail::OnMailCommandController},
    headers::EmailHeaders,
    mail::Mail,
    message::Message,
    server::SMTPServer,
    status_code::StatusCodes, utilities::spf::{sender_policy_framework, SPFRecordAll},
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
        // Set an controller to dispatch when a mail command is received
        .on_mail_cmd(OnMailCommandController::new(on_mail_cmd))
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
        .status(StatusCodes::AuthenticationSuccessful)
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

// This function is called when a mail command is received, usually is to indicate the sender of the email
// Here you can apply the SPF check
pub async fn on_mail_cmd(conn: Arc<Mutex<SMTPConnection<ConnectionState>>>, data: String) -> Result<Message, Message> {
    // `data` give you the raw data, usually like this `FROM:<email@nervio.us> SIZE:123`
    let email_domain = "gmail.com"; // example domain

    // Apply the SPF check
    match sender_policy_framework(
        conn.clone(),
        // The domain to check the SPF record, usually the sender email domain
        email_domain,
        // Passive allow emails that aren't send on domain behalf, but are spam.
        SPFRecordAll::Passive,
        // Max depth of redirects, the SPF record can redirect to another domain, and this domain can redirect to another domain, and so on.
        3,
        // Max includes, the SPF record can include another SPF record.
        3,
    ).await {
        // The SPF pass your check and return the SPFRecord (including, the included ones)
        Ok((pass, _record)) => {
            if !pass {
                return Err(Message::builder()
                    .status(StatusCodes::TransactionFailed)
                    .message("SPF failed".to_string())
                    .build());
            }
        },
        // The SPF failed, return an error, can be the DNS (1.1.1.1 by default) or a parsing error
        Err(e) => {
            // Decline the transaction and close connection
            return Err(Message::builder()
                .status(StatusCodes::TransactionFailed)
                .message("SPF failed".to_string())
                .build());
        }
    }

    // If the email pass the SPF check, you can continue with your logic, but if it fails, you can return an error with Err(Message) and connection will be closed peacefully
    Ok(Message::builder()
        .status(StatusCodes::OK)
        .message("Mail command received".to_string())
        .build())
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