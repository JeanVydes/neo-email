use super::status_code::StatusCodes;

/// # Message
///
/// This struct represents a message to be sent to the client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub status: StatusCodes,
    pub message: String,
}

/// # Message Builder
///
/// This struct is a builder for the Message struct.
#[derive(Debug, Clone, Default)]
pub struct MessageBuilder {
    status: Option<StatusCodes>,
    message: Option<String>,
}

impl Message {
    pub fn new(status: StatusCodes, message: String) -> Self {
        Self { status, message }
    }

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
    pub fn status(mut self, status: StatusCodes) -> Self {
        self.status = Some(status);
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn build(self) -> Message {
        Message {
            status: self.status.unwrap(),
            message: self.message.unwrap(),
        }
    }
}
