use std::fmt;

use super::command::Commands;

/// # SMTP Error
///
/// This enum represents the possible errors that can occur in the SMTP server.
#[derive(Debug)]
pub enum SMTPError {
    /// # IO Error
    /// 
    /// This error occurs when there is an IO error.
    IoError(std::io::Error),
    /// # Parse Error
    /// 
    /// This error occurs when there is a parsing error.
    ParseError(String),
    /// # DKIM Error
    /// 
    /// This error occurs when there is a DKIM error.
    DKIMError(String),
    /// # SPF Error
    /// 
    /// This error occurs when there is a SPF error.
    SPFError(String),
    /// # DMARC Error
    /// 
    /// This error occurs when there is a DMARC error.
    DMARCError(String),
    /// # DNS Error
    /// 
    /// This error occurs when there is a DNS error.
    DNSError(String),
    /// # Unknown Command
    /// 
    /// This error occurs when there is an unknown command.
    UnknownCommand(Commands),
    /// # Custom Error
    /// 
    /// This error occurs when there is a custom error.
    CustomError(String),
}

/// # Display implementation for SMTPError
impl fmt::Display for SMTPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SMTPError::IoError(err) => write!(f, "IO Error: {}", err),
            SMTPError::ParseError(err) => write!(f, "Parse Error: {}", err),
            SMTPError::DKIMError(err) => write!(f, "DKIM Error: {}", err),
            SMTPError::SPFError(err) => write!(f, "SPF Error: {}", err),
            SMTPError::DMARCError(err) => write!(f, "DMARC Error: {}", err),
            SMTPError::DNSError(err) => write!(f, "DNS Error: {}", err),
            SMTPError::UnknownCommand(cmd) => write!(f, "Unknown Command: {:?}", cmd),
            SMTPError::CustomError(msg) => write!(f, "Custom Error: {}", msg),
        }
    }
}

/// # Standard Error implementation for SMTPError
impl std::error::Error for SMTPError {}
