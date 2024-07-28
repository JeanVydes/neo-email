use std::fmt;

use super::command::Commands;

/// # SMTP Error
/// 
/// This enum represents the possible errors that can occur in the SMTP server.
#[derive(Debug)]
pub enum SMTPError<'a> {
    IoError(std::io::Error),
    ParseError(&'a str),
    UnknownCommand(Commands),
    CustomError(&'a str),
}

impl <'a> fmt::Display for SMTPError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SMTPError::IoError(err) => write!(f, "IO Error: {}", err),
            SMTPError::ParseError(err) => write!(f, "Parse Error: {}", err),
            SMTPError::UnknownCommand(cmd) => write!(f, "Unknown Command: {:?}", cmd),
            SMTPError::CustomError(msg) => write!(f, "Custom Error: {}", msg),
        }
    }
}

impl <'a> std::error::Error for SMTPError<'a> {}