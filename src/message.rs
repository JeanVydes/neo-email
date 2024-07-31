use super::status_code::StatusCodes;

/// # Message
/// 
/// This struct represents a message that the SMTP server can return to the client.
///
/// ```rust
/// use neo_email::status_code::StatusCodes;
/// use neo_email::message::Message;
/// 
/// Message::builder()
///     .status(StatusCodes::AuthenticationSuccessful)
///     .message("Authenticated".to_string())
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// # Status
    /// 
    /// The status code of the message.
    /// 
    /// ## Example
    /// 
    /// `StatusCodes::AuthenticationSuccessful`
    pub status: StatusCodes,
    /// # Message
    /// 
    /// The message to be sent.
    pub message: String,
}

/// # Message Builder
///
/// This struct is a builder for the Message struct.
/// 
/// ```rust
/// use neo_email::status_code::StatusCodes;
/// use neo_email::message::Message;
/// 
/// Message::builder()
///     .status(StatusCodes::AuthenticationSuccessful)
///     .message("Authenticated".to_string())
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct MessageBuilder {
    status: Option<StatusCodes>,
    message: Option<String>,
}

impl Message {
    /// # New
    /// 
    /// This function creates a new message.
    pub fn new(status: StatusCodes, message: String) -> Self {
        Self { status, message }
    }

    /// # Builder
    /// 
    /// This function returns a MessageBuilder.
    pub fn builder() -> MessageBuilder {
        MessageBuilder::default()
    }

    /// # To String
    ///
    /// This function converts the message to a string.
    pub fn to_string(&self, is_last: bool) -> String {
        // If it is the last message, return the status code and message with a space
        // If it is not the last message, return the status code and message with a dash
        if is_last {
            format!("{} {}\r\n", self.status.to_string(), self.message)
        } else {
            format!("{}-{}\r\n", self.status.to_string(), self.message)
        }
    }

    /// # As Bytes
    ///
    /// This function converts the message to bytes.
    pub fn as_bytes(&self, is_last: bool) -> Vec<u8> {
        self.to_string(is_last).as_bytes().to_vec()
    }
}

impl MessageBuilder {
    /// # Set Status
    /// 
    /// This function sets the status of the message.
    pub fn status(mut self, status: StatusCodes) -> Self {
        self.status = Some(status);
        self
    }

    /// # Set Message
    /// 
    /// This function sets the message of the message.
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// # Build
    /// 
    /// This function builds the message.
    pub fn build(self) -> Message {
        Message {
            status: self.status.unwrap(),
            message: self.message.unwrap(),
        }
    }
}
