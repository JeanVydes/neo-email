use std::fmt;

use super::command::Commands;

/// # SMTP Error
///
/// This enum represents the possible errors that can occur in the SMTP server.
#[derive(Debug)]
pub enum Error {
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

/// # Display implementation for Error
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "IO Error: {}", err),
            Error::ParseError(err) => write!(f, "Parse Error: {}", err),
            Error::DKIMError(err) => write!(f, "DKIM Error: {}", err),
            Error::SPFError(err) => write!(f, "SPF Error: {}", err),
            Error::DMARCError(err) => write!(f, "DMARC Error: {}", err),
            Error::DNSError(err) => write!(f, "DNS Error: {}", err),
            Error::UnknownCommand(cmd) => write!(f, "Unknown Command: {:?}", cmd),
            Error::CustomError(msg) => write!(f, "Custom Error: {}", msg),
        }
    }
}

/// # Standard Error implementation for Error
impl std::error::Error for Error {}
