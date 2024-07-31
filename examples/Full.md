# Full Example

A full example with TLS and Logger an using controller to setup a simple and workable SMTP server.

```rust
use std::fs::File;
use std::io::{BufReader, Read};
use std::net::SocketAddr;
use std::sync::Arc;

use neo_email::command::Commands;
use neo_email::connection::SMTPConnection;
use neo_email::controllers::on_auth::OnAuthController;
use neo_email::controllers::on_email::OnEmailController;
use neo_email::controllers::on_mail_cmd::OnMailCommandController;
use neo_email::controllers::on_rcpt::OnRCPTCommandController;
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
    pub sender: Option<String>,
    pub recipients: Vec<String>,
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
        // Set an controller to dispatch when a mail command is received, usually is to indicate the sender of the email
        .on_mail_cmd(OnMailCommandController::new(on_mail_cmd))
        // Set an controller to dispatch when a rcpt command is received, usually is to indicate the recipient/s of the email
        .on_rcpt_cmd(OnRCPTCommandController::new(on_rcpt_cmd))

        // Other controllers
        // .on_close(OnCloseController::new(on_close))
        // .on_reset(OnResetController::new(on_reset))
        // .on_unknown_cmd(OnUnknownCommandController::new(on_unknown_command))

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

// This function is called when a mail command is received, usually is to indicate the sender of the email
// Here you can apply the SPF check
pub async fn on_mail_cmd(conn: Arc<Mutex<SMTPConnection<ConnectionState>>>, data: String) -> Result<Message, Message> {
    let conn = conn.lock().await;
    let mut state = conn.state.lock().await;
    // you should check if the last command was EHLO HELLO or another RCPT
    if let Some(command) = conn.tracing_commands.last() {
        if *command != Commands::EHLO &&
            *command != Commands::HELO &&
            *command != Commands::RCPT {
            return Err(Message::builder()
                .status(StatusCodes::TransactionFailed)
                .message("Invalid command".to_string())
                .build());
        }
    }

    // data give you the raw data, usually like this `FROM:<email@nervio.us> SIZE:123`
    let email_address = Commands::parse_mail_command_data(data).map_err(|_| Message::builder()
        .status(StatusCodes::TransactionFailed)
        .message("Invalid email".to_string())
        .build())?;

    // We can use the state to store the sender
    state.sender = Some(email_address.to_string());

    // Apply the SPF check (sender_policy_framework is onyl accessible through the `experimental` or `spf-experimental` feature)
    /*match sender_policy_framework(
        // SMTPConnection contains required information to perform the SPF check
        conn.clone(),
        // The domain to check the SPF record, usually the sender email domain
        &email_address.domain,
        // Passive allow emails that aren't send on domain behalf, but are spam.
        SPFRecordAll::Passive,
        // Max depth of redirects, the SPF record can redirect to another domain, and this domain can redirect to another domain, and so on.
        3,
        // Max includes, the SPF record can include another SPF record.
        3,
    ).await {
        // The SPF pass your check and return the SPFRecord (including, the included ones)
        Ok((pass, _dkim_dns_record, _matched_ip_pattern)) => {
            if !pass {
                return Err(Message::builder()
                    .status(StatusCodes::TransactionFailed)
                    .message("SPF failed".to_string())
                    .build());
            }
        },
        // The SPF failed, return an error, can be the DNS (1.1.1.1 by default) or a parsing error
        Err(e) => {
            log::error!("Error: {:?}", e);
            return Err(Message::builder()
                .status(StatusCodes::TransactionFailed)
                .message("SPF failed".to_string())
                .build());
        }
    }*/

    // If the email pass the SPF check, you can continue with your logic, but if it fails, you can return an error with Err(Message) and connection will be closed peacefully
    Ok(Message::builder()
        .status(StatusCodes::OK)
        .message("Mail command received".to_string())
        .build())
}

// This function is called when a RCPT command is received, usually is to indicate the recipient of the email
// Multiple recipients can be added in different RCPT commands
pub async fn on_rcpt_cmd(conn: Arc<Mutex<SMTPConnection<ConnectionState>>>, data: String) -> Result<Message, Message> {
    let conn = conn.lock().await;
    let mut state = conn.state.lock().await;

    // data give you the raw data, usually like this `TO:<email@nervio.us> SIZE:123`
    let email_address = Commands::parse_rcpt_command_data(data).map_err(|_| Message::builder()
        .status(StatusCodes::TransactionFailed)
        .message("Invalid email".to_string())
        .build())?;

    // We can use the state to store the recipients
    state.recipients.push(email_address.to_string());

    Ok(Message::builder()
        .status(StatusCodes::OK)
        .message("Mail command received".to_string())
        .build())
}

fn set_logger() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::BrightBlue,
                log::Level::Debug => Color::BrightWhite,
                log::Level::Trace => Color::BrightBlack,
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