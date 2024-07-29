use serde::{Deserialize, Serialize};

use crate::{errors::SMTPError, mail::EmailAddress};

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
}