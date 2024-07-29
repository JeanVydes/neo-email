use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::{command::Commands, errors::SMTPError};

/// # Client Message
/// 
/// This struct represents a message from the client to the server.
/// It contains the command and the data.
/// Usually they are like this:
/// 
/// ```
/// HELO example.com
/// MAIL FROM: <...>
/// RCPT TO: <...>
/// DATA
/// ...
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ClientMessage<T> {
    /// The command that the client is sending.
    pub command: Commands,
    /// The data that the client is sending.
    pub data: T,
}

/// # Client Message
/// 
/// This implementation is for the ClientMessage struct.
impl<T> ClientMessage<T> {
    /// # From Bytes
    /// 
    /// This function converts a byte array to a ClientMessage struct.
    pub fn from_bytes<'a>(bytes: Vec<u8>) -> Result<ClientMessage<T>, SMTPError>
    where
        // The data must be able to be converted from a Vec<u8>
        T: std::convert::From<std::string::String> + Debug,
    {
        // Convert bytes to string
        let message = match String::from_utf8(bytes.to_vec()) {
            Ok(cmd) => cmd,
            // If it fails, return an error
            Err(_) => return Err(SMTPError::ParseError("Cannot convert to String from bytes".to_owned())),
        };

        // Split the message by spaces
        let mut parts = message.split(" ");
    
        // Get the command
        let cmd = match parts.next() {
            Some(cmd) => cmd.to_string(),
            None => {
                // If there is no command, return an error
                return Err(SMTPError::ParseError(
                    "Invalid Message, Message doesn't contain COMMAND".to_owned(),
                ))
            }
        };

        // Rejoin with spaces
        let data = parts.collect::<Vec<&str>>().join(" ");
        // remove \r\n
        let data = data.trim_end_matches("\r\n").to_string();
        // Convert the command to a Commands enum
        let command = Commands::from_bytes(cmd.as_bytes());
        // Return the ClientMessage
        Ok(ClientMessage { command, data: data.into() })
    }
}