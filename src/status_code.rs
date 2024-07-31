use serde::{Deserialize, Serialize};

/// # SMTP Status Codes
///
/// This enum represents the status codes that the SMTP server can return to client.
/// 
/// ## Example
/// 
/// ```rust
/// use neo_email::status_code::StatusCodes;
/// use neo_email::message::Message;
/// 
/// Message::builder()
///     .status(StatusCodes::AuthenticationSuccessful)
///     .message("Authenticated".to_string())
///     .build()
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StatusCodes {
    /// # Help Message
    HelpMessage = 214,
    /// # SMTP Service Ready
    SMTPServiceReady = 220,
    /// # Service Closing Transmission Channel
    ServiceClosingTransmissionChannel = 221,
    /// # Authentication Successful
    AuthenticationSuccessful = 235,
    /// # OK
    OK = 250,
    /// # User Not Local Will Forward
    UserNotLocalWillForward = 251,
    /// # Cannot Verify User But Will Accept Message And Attempt Delivery
    CannotVerifyUserButWillAcceptMessageAndAttemptDelivery = 252,

    /// # Start Mail Input
    StartMailInput = 354,

    /// # Service Not Available
    ServiceNotAvailable = 421,
    /// # Requested Mail Action Not Taken: Mailbox Unavailable
    RequestedMailActionNotTakenMailboxUnavailable = 450,
    /// # Requested Action Aborted: Local Error In Processing
    RequestedActionAbortedLocalErrorInProcessing = 451,
    /// # Insufficient System Storage
    InsufficientSystemStorage = 452,
    /// # Server Unable To Accommodate Parameters
    ServerUnableToAccommodateParameters = 455,

    /// # Syntax Error
    SyntaxError = 500,
    /// # Syntax Error In Parameters Or Arguments
    SyntaxErrorInParametersOrArguments = 501,
    /// # Command Not Implemented
    CommandNotImplemented = 502,
    /// # Bad Sequence Of Commands
    BadSequenceOfCommands = 503,
    /// # Command Parameter Not Implemented
    CommandParameterNotImplemented = 504,
    /// # Server Does Not Accept Mail
    ServerDoesNotAcceptMail = 521,
    /// # Authentication Credetials Invalid
    AuthenticationCredetialsInvalid = 535,
    /// # Recipient Address Rejected
    RecipientAddressRejected = 541,
    /// # Requested Action Not Taken: Mailbox Unavailable
    RequestedActionNotTakenMailboxUnavailable = 550,
    /// # User Not Local: Try Forwarding
    UserNotLocalTryForwarding = 551,
    /// # Exceeded Storage Allocation
    ExceededStorageAllocation = 552,
    /// # Mailbox Name Not Allowed
    MailboxNameNotAllowed = 553,
    /// # Transaction Failed
    TransactionFailed = 554,
}

/// # Status Codes
/// 
/// This struct contains methods for the StatusCodes enum.
impl StatusCodes {
    /// # To String
    /// 
    /// This function converts the status code to a string.
    pub fn to_string(&self) -> String {
        match self {
            StatusCodes::HelpMessage => "214".to_string(),
            StatusCodes::SMTPServiceReady => "220".to_string(),
            StatusCodes::ServiceClosingTransmissionChannel => "221".to_string(),
            StatusCodes::AuthenticationSuccessful => "235".to_string(),
            StatusCodes::OK => "250".to_string(),
            StatusCodes::UserNotLocalWillForward => "251".to_string(),
            StatusCodes::CannotVerifyUserButWillAcceptMessageAndAttemptDelivery => {
                "252".to_string()
            }
            StatusCodes::StartMailInput => "354".to_string(),
            StatusCodes::ServiceNotAvailable => "421".to_string(),
            StatusCodes::RequestedMailActionNotTakenMailboxUnavailable => "450".to_string(),
            StatusCodes::RequestedActionAbortedLocalErrorInProcessing => "451".to_string(),
            StatusCodes::InsufficientSystemStorage => "452".to_string(),
            StatusCodes::ServerUnableToAccommodateParameters => "455".to_string(),
            StatusCodes::SyntaxError => "500".to_string(),
            StatusCodes::SyntaxErrorInParametersOrArguments => "501".to_string(),
            StatusCodes::CommandNotImplemented => "502".to_string(),
            StatusCodes::BadSequenceOfCommands => "503".to_string(),
            StatusCodes::CommandParameterNotImplemented => "504".to_string(),
            StatusCodes::ServerDoesNotAcceptMail => "521".to_string(),
            StatusCodes::AuthenticationCredetialsInvalid => "535".to_string(),
            StatusCodes::RecipientAddressRejected => "541".to_string(),
            StatusCodes::RequestedActionNotTakenMailboxUnavailable => "550".to_string(),
            StatusCodes::UserNotLocalTryForwarding => "551".to_string(),
            StatusCodes::ExceededStorageAllocation => "552".to_string(),
            StatusCodes::MailboxNameNotAllowed => "553".to_string(),
            StatusCodes::TransactionFailed => "554".to_string(),
        }
    }
}
