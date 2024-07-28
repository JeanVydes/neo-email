# Full Example

A full example with TLS and Logger

```rust
use std::fs::File;
use std::io::{BufReader, Read};
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
use tokio_native_tls::native_tls::{Identity, TlsAcceptor};
use tokio_native_tls::TlsAcceptor as TokioTlsAcceptor;

use colored::*;
use fern::Dispatch;

#[derive(Debug, Clone, Default)]
pub struct ConnectionState {
    pub authenticated: bool,
}

#[tokio::main]
async fn main() {
    set_logger().unwrap();
    log::debug!("Starting server");

    let addr = SocketAddr::from(([127, 0, 0, 1], 2526));

    // Load the certificate
    // check examples/certificates/README.md for more information
    let cert_path = "server.p12"; // Adjust this path to your certificate file
    let cert_password = "your_password"; // Adjust this to your certificate password

    let file = File::open(cert_path).expect("Cannot open certificate file");
    let mut reader = BufReader::new(file);
    let mut identity_data = Vec::new();
    reader.read_to_end(&mut identity_data).expect("Cannot read certificate file");

    let identity = Identity::from_pkcs12(&identity_data, cert_password).expect("Cannot create identity from certificate");

    // Create the native_tls acceptor
    let tls_acceptor = TlsAcceptor::builder(identity).build().expect("Cannot build TLS acceptor");

    // Convert the native_tls acceptor to a tokio-native-tls acceptor
    let tokio_tls_acceptor = TokioTlsAcceptor::from(tls_acceptor);

    // Create the server
    SMTPServer::<ConnectionState>::new()
        // Set the number of workers to 1
        .workers(1)
        // Set the TLS acceptor
        .set_tls_acceptor(tokio_tls_acceptor)
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
pub async fn on_auth(conn: Arc<Mutex<SMTPConnection<ConnectionState>>>, data: String) -> Result<Message, Message> {
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
pub async fn on_email(conn: Arc<Mutex<SMTPConnection<ConnectionState>>>, mail: Mail<Vec<u8>>) -> Message {
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

    log::info!("Received email: {:?}", mail);
    Message::builder()
        .status(neo_email::status_code::StatusCodes::OK)
        .message("Email received".to_string())
        .build()
}

fn set_logger() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug => Color::Blue,
                log::Level::Trace => Color::Magenta,
            };
            
            // Format the log message
            let formatted_message = format!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            );

            // Apply color to the message based on the log level
            out.finish(format_args!(
                "{}",
                formatted_message.color(level_color)
            ));
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Trace)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout
        .chain(std::io::stdout())
        // Apply globally
        .apply()?;
    
    // Example usage
    log::info!("This is an info message");
    log::error!("This is an error message");

    Ok(())
}
```