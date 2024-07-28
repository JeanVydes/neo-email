# Full Example

A full example with TLS and Logger

```rust
use std::fs::File;
use std::io::{BufReader, Read};
use std::net::SocketAddr;

use neo_email::controllers::on_email::OnEmailController;
use neo_email::mail::Mail;
use neo_email::message::Message;
use neo_email::status_code::StatusCodes;
use neo_email::server::SMTPServer;

use tokio_native_tls::native_tls::{Identity, TlsAcceptor};
use tokio_native_tls::TlsAcceptor as TokioTlsAcceptor;

use colored::*;
use fern::Dispatch;

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
    SMTPServer::new()
        // Set the number of workers to 1
        .workers(1)
        // Set the TLS acceptor
        .set_tls_acceptor(tokio_tls_acceptor)
        // Set an controller to dispatch when an email is received
        .on_email(OnEmailController::new(|_, mail: Mail<Vec<u8>>| {
            log::info!("Received email: {:?}", mail);
            Message::builder()
                .status(StatusCodes::OK)
                .message("Email received".to_string())
                .build()
        }))
        // Bind the server to the address
        .bind(addr)
        .await
        .unwrap()
        // Run the server
        .run()
        .await;
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