use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{client_message::ClientMessage, connection::{SMTPConnection, SMTPConnectionStatus}, errors::SMTPError, mail::EmailAddress, message::Message, server::Controllers, status_code::StatusCodes};

/// # SMTP Commands
/// 
/// This enum represents the commands that the SMTP server can receive.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Commands {
    /// HELO Command
    /// 
    /// This command is used to identify the client to the server.
    HELO,
    /// Extended HELO
    /// 
    /// Usually used for getting the server capabilities.
    EHLO,
    /// MAIL Command
    /// 
    /// This command is used to specify the sender of the email.
    MAIL,
    /// RCPT Command
    /// 
    /// This command is used to specify the recipient of the email.
    RCPT,
    /// DATA Command
    /// This command is used to send the email data.
    DATA,
    /// RSET Command
    /// 
    /// This command is used to reset the session.
    RSET,
    /// VRFY Command
    /// 
    /// This command is used to verify the email address.
    VRFY,
    /// EXPN Command
    /// 
    /// This command is used to expand the mailing list.
    EXPN,
    /// HELP Command
    /// 
    /// This command is used to get help from the server.
    HELP,
    /// NOOP Command
    /// 
    /// This command is used to do nothing.
    NOOP,
    /// QUIT Command
    /// 
    /// This command is used to quit the session.
    QUIT,
    /// AUTH Command
    /// 
    /// This command is used to authenticate the user.
    AUTH,
    /// STARTTLS Command
    /// 
    /// This command is used to start the TLS session.
    STARTTLS,
    /// Unknown Command
    /// 
    /// This command is used when the command is not recognized.
    UNKNOWN(String),
}

impl Commands {
    /// # From Bytes
    /// 
    /// This function converts a byte array to a Commands enum.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        // Convert bytes to string, uppercase, trim and convert to string
        let bytes_to_string = String::from_utf8_lossy(bytes)
            .to_uppercase()
            .trim()
            .to_string();
        match bytes_to_string.as_str() {
            "HELO" => Commands::HELO,
            "EHLO" => Commands::EHLO,
            "MAIL" => Commands::MAIL,
            "RCPT" => Commands::RCPT,
            "DATA" => Commands::DATA,
            "RSET" => Commands::RSET,
            "VRFY" => Commands::VRFY,
            "EXPN" => Commands::EXPN,
            "HELP" => Commands::HELP,
            "NOOP" => Commands::NOOP,
            "QUIT" => Commands::QUIT,
            "AUTH" => Commands::AUTH,
            "STARTTLS" => Commands::STARTTLS,
            _ => Commands::UNKNOWN(bytes_to_string),
        }
    }

    pub fn parse_mail_command_data(data: String) -> Result<EmailAddress, SMTPError> {
        // Trim any leading or trailing whitespace
        let data = data.trim();
        
        // Extract the part between '<' and '>'
        let start = data.find('<').ok_or(SMTPError::ParseError("Invalid email address".to_string()))?;
        let end = data.find('>').ok_or(SMTPError::ParseError("Invalid email address".to_string()))?;
        
        // Extract and trim the email address part
        let email_address = &data[start + 1..end];
        EmailAddress::from_string(email_address).map_err(|_| SMTPError::ParseError("Invalid email address".to_string()))
    }

    pub fn parse_rcpt_command_data(data: String) -> Result<EmailAddress, SMTPError> {
        // Trim any leading or trailing whitespace
        let data = data.trim();
        
        // Extract the part between '<' and '>'
        let start = data.find('<').ok_or(SMTPError::ParseError("Invalid email address".to_string()))?;
        let end = data.find('>').ok_or(SMTPError::ParseError("Invalid email address".to_string()))?;
        
        // Extract and trim the email address part
        let email_address = &data[start + 1..end];
        EmailAddress::from_string(email_address).map_err(|_| SMTPError::ParseError("Invalid email address".to_string()))
    }
}

pub async fn handle_command<B>(
    conn: Arc<Mutex<SMTPConnection<B>>>,
    controllers: Controllers<B>,
    client_message: &mut ClientMessage<String>,
    allowed_commands: Vec<Commands>,
    max_size: usize,
) -> Result<(Vec<Message>, SMTPConnectionStatus), SMTPError>
where
    B: 'static + Default + Send + Sync + Clone,
{
    log::trace!("[⚙️] Handling SMTP command: {:?}", client_message.command);

    // Check if the command is allowed
    if allowed_commands
        .iter()
        .find(|&cmd| cmd == &client_message.command)
        .is_none()
    {
        return Err(SMTPError::UnknownCommand(client_message.command.clone()));
    }

    let result = match client_message.command {
        Commands::HELO => (
            vec![Message::builder()
                .status(StatusCodes::OK)
                .message(format!("Hello {}", "unknown"))
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::EHLO => {
            let mut ehlo_messages = vec![
                Message::builder()
                    .status(StatusCodes::OK)
                    .message("Hello".to_string())
                    .build(),
                Message::builder()
                    .status(StatusCodes::OK)
                    .message(format!("SIZE {}", max_size))
                    .build(),
                Message::builder()
                    .status(StatusCodes::OK)
                    .message("8BITMIME".to_string())
                    .build(),
                Message::builder()
                    .status(StatusCodes::OK)
                    .message("PIPELINING".to_string())
                    .build(),
                Message::builder()
                    .status(StatusCodes::OK)
                    .message("HELP".to_string())
                    .build(),
            ];

            let conn = conn.lock().await;
            if !conn.use_tls {
                ehlo_messages.push(
                    Message::builder()
                        .status(StatusCodes::OK)
                        .message("STARTTLS".to_string())
                        .build(),
                )
            }

            if controllers.on_auth.is_some() {
                ehlo_messages.push(
                    Message::builder()
                        .status(StatusCodes::OK)
                        .message(
                            "AUTH PLAIN LOGIN CRAM-MD5 DIGEST-MD5 GSSAPI NTLM XOAUTH2".to_string(),
                        )
                        .build(),
                );
            }

            drop(conn);

            (ehlo_messages, SMTPConnectionStatus::WaitingCommand)
        }
        Commands::MAIL => {
            if let Some(on_mail_cmd) = &controllers.on_mail_cmd {
                let on_mail_cmd = on_mail_cmd.0.clone();
                match on_mail_cmd(conn.clone(), client_message.data.clone()).await {
                    Ok(response) => {
                        return Ok((vec![response], SMTPConnectionStatus::WaitingCommand))
                    }
                    Err(response) => return Ok((vec![response], SMTPConnectionStatus::Closed)),
                }
            } else {
                (
                    vec![Message::builder()
                        .status(StatusCodes::OK)
                        .message("Ok".to_string())
                        .build()],
                    SMTPConnectionStatus::WaitingCommand,
                )
            }
        }
        Commands::RCPT => {
            if let Some(on_rcpt_cmd) = &controllers.on_rcpt_cmd {
                let on_rcpt_cmd = on_rcpt_cmd.0.clone();
                match on_rcpt_cmd(conn.clone(), client_message.data.clone()).await {
                    Ok(response) => (vec![response], SMTPConnectionStatus::WaitingCommand),
                    Err(response) => (vec![response], SMTPConnectionStatus::Closed),
                }
            } else {
                let last_command = conn.lock().await;
                let last_command = last_command
                    .tracing_commands
                    .last()
                    .unwrap_or(&Commands::HELO);

                if last_command != &Commands::MAIL && last_command != &Commands::RCPT {
                    (
                        vec![Message::builder()
                            .status(StatusCodes::BadSequenceOfCommands)
                            .message("Bad sequence of commands".to_string())
                            .build()],
                        SMTPConnectionStatus::WaitingCommand,
                    )
                } else {
                    (
                        vec![Message::builder()
                            .status(StatusCodes::OK)
                            .message("Ok".to_string())
                            .build()],
                        SMTPConnectionStatus::WaitingCommand,
                    )
                }
            }
        }
        Commands::DATA => (
            vec![Message::builder()
                .status(StatusCodes::StartMailInput)
                .message("Start mail input; end with <CRLF>.<CRLF>".to_string())
                .build()],
            SMTPConnectionStatus::WaitingData,
        ),
        Commands::RSET => (
            vec![Message::builder()
                .status(StatusCodes::OK)
                .message("Hello".to_string())
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::VRFY => (
            vec![Message::builder()
                .status(StatusCodes::CannotVerifyUserButWillAcceptMessageAndAttemptDelivery)
                .message(
                    "Cannot VRFY user, but will accept message and attempt delivery".to_string(),
                )
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::EXPN => (
            vec![Message::builder()
                .status(StatusCodes::CommandNotImplemented)
                .message(
                    "Cannot EXPN user, but will accept message and attempt delivery".to_string(),
                )
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::HELP => (
            vec![Message::builder()
                .status(StatusCodes::HelpMessage)
                .message("Help message".to_string())
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::NOOP => (
            vec![Message::builder()
                .status(StatusCodes::OK)
                .message("NOOP Command successful".to_string())
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::QUIT => (
            vec![Message::builder()
                .status(StatusCodes::ServiceClosingTransmissionChannel)
                .message("Service closing transmission channel".to_string())
                .build()],
            SMTPConnectionStatus::Closed,
        ),
        Commands::AUTH => {
            if let Some(on_auth) = &controllers.on_auth {
                let on_auth = on_auth.0.clone();
                match on_auth(conn.clone(), client_message.data.clone()).await {
                    Ok(response) => (vec![response], SMTPConnectionStatus::WaitingCommand),
                    Err(response) => return Ok((vec![response], SMTPConnectionStatus::Closed)),
                }
            } else {
                (
                    vec![Message::builder()
                        .status(StatusCodes::CommandNotImplemented)
                        .message("Command not recognized".to_string())
                        .build()],
                    SMTPConnectionStatus::WaitingCommand,
                )
            }
        }
        Commands::STARTTLS => {
            let conn = conn.lock().await;

            if conn.use_tls {
                (
                    vec![Message::builder()
                        .status(StatusCodes::TransactionFailed)
                        .message("Already using TLS".to_string())
                        .build()],
                    SMTPConnectionStatus::WaitingCommand,
                )
            } else {
                (
                    vec![Message::builder()
                        .status(StatusCodes::SMTPServiceReady)
                        .message("Ready to start TLS".to_string())
                        .build()],
                    SMTPConnectionStatus::StartTLS,
                )
            }
        }
        _ => {
            if let Some(on_unknown_cmd) = &controllers.on_unknown_cmd {
                let on_unknown_cmd = on_unknown_cmd.0.clone();
                match on_unknown_cmd(conn.clone(), client_message.command.clone()).await {
                    Ok(response) => (vec![response], SMTPConnectionStatus::WaitingCommand),
                    Err(response) => (vec![response], SMTPConnectionStatus::Closed),
                }
            } else {
                (
                    vec![Message::builder()
                        .status(StatusCodes::CommandNotImplemented)
                        .message("Command not recognized".to_string())
                        .build()],
                    SMTPConnectionStatus::WaitingCommand,
                )
            }
        }
    };

    let mut guarded_conn = conn.lock().await;
    guarded_conn
        .tracing_commands
        .push(client_message.command.clone());
    drop(guarded_conn);

    Ok(result)
}