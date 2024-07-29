use core::fmt;
use std::str::{from_utf8, FromStr};
use serde::{Deserialize, Serialize};

/// # Email Headers
/// 
/// The headers that a email can contain.
/// [https://www.iana.org/assignments/message-headers/message-headers.xhtml](https://www.iana.org/assignments/message-headers/message-headers.xhtml)
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EmailHeaders {
    #[serde(rename = "Accept-Language")]
    AcceptLanguage, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Alternate-Recipient")]
    AlternateRecipient, // https://www.iana.org/go/rfc4021
    #[cfg(feature = "smtp-experimental")]
    #[serde(rename = "ARC-Authentication-Results")]
    ARCAuthenticationResults, // https://www.iana.org/go/rfc8617
    #[cfg(feature = "smtp-experimental")]
    #[serde(rename = "ARC-Message-Signature")]
    ARCMessageSignature, // https://www.iana.org/go/rfc8617
    #[cfg(feature = "smtp-experimental")]
    #[serde(rename = "ARC-Seal")]
    ARCSeal, // https://www.iana.org/go/rfc8617
    #[serde(rename = "Archived-At")]
    ArchivedAt, // https://www.iana.org/go/rfc5064
    #[serde(rename = "Authentication-Results")]
    AuthenticationResults, // https://www.iana.org/go/rfc8601
    #[serde(rename = "Auto-Submitted")]
    AutoSubmitted, // (Auto-Submitted) https://www.iana.org/go/rfc3834 (Section 5)
    #[serde(rename = "Autoforwarded")]
    AutoForwarded, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Autosubmitted")]
    Autosubmitted, // (Autosubmitted) https://www.iana.org/go/rfc4021

    #[serde(rename = "Bcc")]
    Bcc, // https://www.iana.org/go/rfc5322

    #[serde(rename = "Cc")]
    Cc, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Comments")]
    Comments, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Content-Identifier")]
    ContentIdentifier, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Content-Return")]
    ContentReturn, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Conversion")]
    Conversion, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Conversion-With-Loss")]
    ConversionWithLoss, // https://www.iana.org/go/rfc4021

    #[serde(rename = "DL-Expansion-History")]
    DLExpansionHistory, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Date")]
    Date, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Deferred-Delivery")]
    DeferredDelivery, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Delivery-Date")]
    DeliveryDate, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Discarded-X400-IPMS-Extensions")]
    DiscardedX400IPMSExtensions, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Discarded-X400-MTS-Extensions")]
    DiscardedX400MTSExtensions, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Disclose-Recipients")]
    DiscloseRecipients, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Disposition-Notification-Options")]
    DispositionNotificationOptions, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Disposition-Notification-To")]
    DispositionNotificationTo, // https://www.iana.org/go/rfc4021
    #[serde(rename = "DKIM-Signature")]
    DKIMSignature, // https://www.iana.org/go/rfc6376
    #[serde(rename = "Downgraded-Final-Recipient")]
    DowngradedFinalRecipient, // https://www.iana.org/go/rfc6857
    #[serde(rename = "Downgraded-In-Reply-To")]
    DowngradedInReplyTo, // https://www.iana.org/go/rfc6857
    #[serde(rename = "Downgraded-Message-Id")]
    DowngradedMessageId, // https://www.iana.org/go/rfc6857
    #[serde(rename = "Downgraded-Original-Recipient")]
    DowngradedOriginalRecipient, // https://www.iana.org/go/rfc6857
    #[serde(rename = "Downgraded-References")]
    DowngradedReferences, // https://www.iana.org/go/rfc6857

    #[serde(rename = "Encoding")]
    Encoding, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Encrypted")]
    Encrypted, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Expires")]
    Expires, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Expiry-Date")]
    ExpiryDate, // https://www.iana.org/go/rfc4021

    #[serde(rename = "From")]
    From, // https://www.iana.org/go/rfc5322 & https://www.iana.org/go/rfc6854

    #[serde(rename = "Generate-Delivery-Report")]
    GenerateDeliveryReport, // https://www.iana.org/go/rfc4021

    #[serde(rename = "Importance")]
    Importance, // https://www.iana.org/go/rfc4021
    #[serde(rename = "In-Reply-To")]
    InReplyTo, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Incomplete-Copy")]
    IncompleteCopy, // https://www.iana.org/go/rfc4021

    #[serde(rename = "Keywords")]
    Keywords, // https://www.iana.org/go/rfc5322

    #[serde(rename = "Language")]
    Language, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Latest-Delivery-Time")]
    LatestDeliveryTime, // https://www.iana.org/go/rfc4021
    #[serde(rename = "List-Archive")]
    ListArchive, // https://www.iana.org/go/rfc4021
    #[serde(rename = "List-Help")]
    ListHelp, // https://www.iana.org/go/rfc4021
    #[serde(rename = "List-Id")]
    ListId, // https://www.iana.org/go/rfc4021
    #[serde(rename = "List-Owner")]
    ListOwner, // https://www.iana.org/go/rfc4021
    #[serde(rename = "List-Post")]
    ListPost, // https://www.iana.org/go/rfc4021
    #[serde(rename = "List-Subscribe")]
    ListSubscribe, // https://www.iana.org/go/rfc4021
    #[serde(rename = "List-Unsubscribe")]
    ListUnsubscribe, // https://www.iana.org/go/rfc4021
    #[serde(rename = "List-Unsubscribe-Post")]
    ListUnsubscribePost, // https://www.iana.org/go/rfc8058

    #[serde(rename = "Message-Context")]
    MessageContext, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Message-Id")]
    MessageId, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Message-Type")]
    MessageType, // https://www.iana.org/go/rfc4021
    #[serde(rename = "MT-Priority")]
    MTPriority, // https://www.iana.org/go/rfc6758

    #[serde(rename = "Obsoletes")]
    Obsoletes, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Organization")]
    Organization, // https://www.iana.org/go/rfc7681
    #[serde(rename = "Original-Encoded-Information-Types")]
    OriginalEncodedInformationTypes, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Original-From")]
    OriginalFrom, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Original-Message-Id")]
    OriginalMessageId, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Original-Recipient")]
    OriginalRecipient, // https://www.iana.org/go/rfc3798 & https://www.iana.org/go/rfc3798
    #[serde(rename = "Originator-Return-Address")]
    OriginatorReturnAddress, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Original-Subject")]
    OriginalSubject, // https://www.iana.org/go/rfc5703

    #[serde(rename = "PICS-Label")]
    PICSLabel, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Prevent-NonDelivery-Report")]
    PreventNonDeliveryReport, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Priority")]
    Priority, // https://www.iana.org/go/rfc4021

    #[serde(rename = "Received")]
    Received, // https://www.iana.org/go/rfc5321
    #[serde(rename = "Received-SPF")]
    ReceivedSPF, // https://www.iana.org/go/rfc7208
    #[serde(rename = "References")]
    References, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Reply-By")]
    ReplyBy, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Reply-To")]
    ReplyTo, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Require-Recipient-Valid-Since")]
    RequireRecipientValidSince, // https://www.iana.org/go/rfc7293
    #[serde(rename = "Resent-Bcc")]
    ResentBcc, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Resent-Cc")]
    ResentCc, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Resent-Date")]
    ResentDate, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Resent-From")]
    ResentFrom, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Resent-Message-Id")]
    ResentMessageId, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Resent-Reply-To")]
    ResentReplyTo, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Resent-Sender")]
    ResentSender, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Resent-To")]
    ResentTo, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Return-Path")]
    ReturnPath, // https://www.iana.org/go/rfc5321

    #[serde(rename = "Sender")]
    Sender, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Sensitivity")]
    Sensitivity, // https://www.iana.org/go/rfc4021
    #[serde(rename = "Solicitation")]
    Solicitation, // https://www.iana.org/go/rfc3865
    #[serde(rename = "Subject")]
    Subject, // https://www.iana.org/go/rfc5322
    #[serde(rename = "Supersedes")]
    Supersedes, // https://www.iana.org/go/rfc4021

    #[serde(rename = "TLS-Report-Domain")]
    TLSReportDomain, // https://www.iana.org/go/rfc8460
    #[serde(rename = "TLS-Report-Submitter")]
    TLSReportSubmitter, // https://www.iana.org/go/rfc8460
    #[serde(rename = "TlS-Required")]
    TLSRequired, // https://www.iana.org/go/rfc8689
    #[serde(rename = "To")]
    To, // https://www.iana.org/go/rfc5322

    #[serde(rename = "VBR-Info")]
    VBRInfo, // https://www.iana.org/go/rfc5518

    #[serde(rename = "X400-Content-Identifier")]
    X400ContentIdentifier, // https://www.iana.org/go/rfc4021
    #[serde(rename = "X400-Content-Return")]
    X400ContentReturn, // https://www.iana.org/go/rfc4021
    #[serde(rename = "X400-Content-Type")]
    X400ContentType, // https://www.iana.org/go/rfc4021
    #[serde(rename = "X400-MTS-Identifier")]
    X400MTSIdentifier, // https://www.iana.org/go/rfc4021
    #[serde(rename = "X400-Originator")]
    X400Originator, // https://www.iana.org/go/rfc4021
    #[serde(rename = "X400-Received")]
    X400Received, // https://www.iana.org/go/rfc4021
    #[serde(rename = "X400-Recipients")]
    X400Recipients, // https://www.iana.org/go/rfc4021
    #[serde(rename = "X400-Trace")]
    X400Trace, // https://www.iana.org/go/rfc4021
}

impl EmailHeaders {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        // Convert bytes to string and trim any whitespace
        let s = from_utf8(bytes).map_err(|_| "Invalid header")?.trim().to_string();

        // Use serde_json to deserialize the string into EmailHeaders enum
        serde_json::from_str(&format!("\"{}\"", s)).map_err(|_| "Unknown header".to_string())
    }
}

impl FromStr for EmailHeaders {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Use serde_json to deserialize the string into EmailHeaders enum
        serde_json::from_str(&format!("\"{}\"", s))
    }
}

// Implement fmt::Display trait to convert EmailHeaders enum to string
impl fmt::Display for EmailHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use serde_json to serialize the EmailHeaders enum to a string
        let serialized = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        // Remove the surrounding quotes from the serialized string
        write!(f, "{}", &serialized[1..serialized.len() - 1])
    }
}