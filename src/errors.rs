use std::fmt;

use super::command::Commands;

/// # SMTP Error
/// 
/// This enum represents the possible errors that can occur in the SMTP server.
#[derive(Debug)]
pub enum SMTPError {
    IoError(std::io::Error),
    ParseError(String),
    DKIMError(String),
    SPFError(String),
    DNSError(String),
    UnknownCommand(Commands),
    CustomError(String),
}

impl fmt::Display for SMTPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SMTPError::IoError(err) => write!(f, "IO Error: {}", err),
            SMTPError::ParseError(err) => write!(f, "Parse Error: {}", err),
            SMTPError::DKIMError(err) => write!(f, "DKIM Error: {}", err),
            SMTPError::SPFError(err) => write!(f, "SPF Error: {}", err),
            SMTPError::DNSError(err) => write!(f, "DNS Error: {}", err),
            SMTPError::UnknownCommand(cmd) => write!(f, "Unknown Command: {:?}", cmd),
            SMTPError::CustomError(msg) => write!(f, "Custom Error: {}", msg),
        }
    }
}

impl std::error::Error for SMTPError {}