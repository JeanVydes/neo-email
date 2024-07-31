use serde::{Deserialize, Serialize};

/// # SMTP Status Codes
///
/// This enum represents the status codes that the SMTP server can return.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StatusCodes {
    HelpMessage = 214,
    SMTPServiceReady = 220,
    ServiceClosingTransmissionChannel = 221,
    AuthenticationSuccessful = 235,
    OK = 250,
    UserNotLocalWillForward = 251,
    CannotVerifyUserButWillAcceptMessageAndAttemptDelivery = 252,

    StartMailInput = 354,

    ServiceNotAvailable = 421,
    RequestedMailActionNotTakenMailboxUnavailable = 450,
    RequestedActionAbortedLocalErrorInProcessing = 451,
    InsufficientSystemStorage = 452,
    ServerUnableToAccommodateParameters = 455,

    SyntaxError = 500,
    SyntaxErrorInParametersOrArguments = 501,
    CommandNotImplemented = 502,
    BadSequenceOfCommands = 503,
    CommandParameterNotImplemented = 504,
    ServerDoesNotAcceptMail = 521,
    AuthenticationCredetialsInvalid = 535,
    RecipientAddressRejected = 541,
    RequestedActionNotTakenMailboxUnavailable = 550,
    UserNotLocalTryForwarding = 551,
    ExceededStorageAllocation = 552,
    MailboxNameNotAllowed = 553,
    TransactionFailed = 554,
}

impl StatusCodes {
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
