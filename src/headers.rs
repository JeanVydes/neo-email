use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::{from_utf8, FromStr};

/// # Email Headers
///
/// The headers that a email can contain.
/// Reference: [https://www.iana.org/assignments/message-headers/message-headers.xhtml](https://www.iana.org/assignments/message-headers/message-headers.xhtml)
/// 
/// ## Example
/// 
/// ```rust
/// use neo_email::mail::Mail;
/// use neo_email::headers::EmailHeaders;
/// 
/// let raw_email = b"From: jean@nervio\nSubject: Hello\n\nHello, World!";
/// let mail = Mail::<Vec<u8>>::from_bytes(raw_email.to_vec()).unwrap();
/// let from = mail.headers.get(&EmailHeaders::From).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EmailHeaders {
    /// # Accept-Language
    /// 
    /// The Accept-Language header field can be used by user agents to indicate the set of natural languages that are preferred in the response.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Accept-Language")]
    AcceptLanguage, // https://www.iana.org/go/rfc4021
    /// # Alternate-Recipient
    /// 
    /// The Alternate-Recipient header field can be used to specify an alternative recipient for the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Alternate-Recipient")]
    AlternateRecipient, // https://www.iana.org/go/rfc4021
    /// # ARC-Authentication-Results
    /// 
    /// The ARC-Authentication-Results header field can be used to convey the results of message authentication checks.
    /// Reference: [https://www.iana.org/go/rfc8617](https://www.iana.org/go/rfc8617)
    #[cfg(feature = "smtp-experimental-headers")]
    #[serde(rename = "ARC-Authentication-Results")]
    /// # ARC-Message-Signature
    /// 
    /// The ARC-Message-Signature header field can be used to convey the message signature.
    /// Reference: [https://www.iana.org/go/rfc8617](https://www.iana.org/go/rfc8617)
    ARCAuthenticationResults, // https://www.iana.org/go/rfc8617
    /// # ARC-Seal
    /// 
    /// The ARC-Seal header field can be used to convey the message seal.
    /// Reference: [https://www.iana.org/go/rfc8617](https://www.iana.org/go/rfc8617)
    #[cfg(feature = "smtp-experimental-headers")]
    #[serde(rename = "ARC-Message-Signature")]
    ARCMessageSignature, // https://www.iana.org/go/rfc8617
    /// # ARC-Seal
    /// 
    /// The ARC-Seal header field can be used to convey the message seal.
    /// Reference: [https://www.iana.org/go/rfc8617](https://www.iana.org/go/rfc8617)
    #[cfg(feature = "smtp-experimental-headers")]
    #[serde(rename = "ARC-Seal")]
    ARCSeal, // https://www.iana.org/go/rfc8617
    /// # Archived-At
    /// 
    /// The Archived-At header field can be used to specify the date and time at which the message was archived.
    /// Reference: [https://www.iana.org/go/rfc5064](https://www.iana.org/go/rfc5064)
    #[serde(rename = "Archived-At")]
    ArchivedAt, // https://www.iana.org/go/rfc5064
    /// # Authentication-Results
    /// 
    /// The Authentication-Results header field can be used to convey the results of message authentication checks.
    /// Reference: [https://www.iana.org/go/rfc8601](https://www.iana.org/go/rfc8601)
    #[serde(rename = "Authentication-Results")]
    AuthenticationResults, // https://www.iana.org/go/rfc8601
    /// # Auto-Submitted
    /// 
    /// The Auto-Submitted header field can be used to indicate that the message was generated automatically.
    /// Reference: [https://www.iana.org/go/rfc3834](https://www.iana.org/go/rfc3834)
    #[serde(rename = "Auto-Submitted")]
    AutoSubmitted, // (Auto-Submitted) https://www.iana.org/go/rfc3834 (Section 5)
    /// # Autoforwarded
    /// 
    /// The Autoforwarded header field can be used to indicate that the message was automatically forwarded.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Autoforwarded")]
    AutoForwarded, // https://www.iana.org/go/rfc4021
    /// # Autosubmitted
    /// 
    /// The Autosubmitted header field can be used to indicate that the message was generated automatically.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Autosubmitted")]
    Autosubmitted, // (Autosubmitted) https://www.iana.org/go/rfc4021

    /// # Bcc
    /// 
    /// The Bcc header field can be used to specify recipients who are not to be included in the delivery of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Bcc")]
    Bcc, // https://www.iana.org/go/rfc5322

    /// # Cc
    /// 
    /// The Cc header field can be used to specify recipients who are to receive copies of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Cc")]
    Cc, // https://www.iana.org/go/rfc5322
    /// # Comments
    /// 
    /// The Comments header field can be used to provide additional information about the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Comments")]
    Comments, // https://www.iana.org/go/rfc5322
    /// # Content-Identifier
    /// 
    /// The Content-Identifier header field can be used to specify a unique identifier for the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Content-Identifier")]
    ContentIdentifier, // https://www.iana.org/go/rfc4021
    /// # Content-Return
    /// 
    /// The Content-Return header field can be used to specify the return address for non-delivery reports.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Content-Return")]
    ContentReturn, // https://www.iana.org/go/rfc4021
    /// # Content-Type
    /// 
    /// The Content-Type header field can be used to specify the media type of the message content.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Content-Type")]
    ContentType, // https://www.iana.org/go/rfc4021
    /// # Content-Transfer-Encoding
    /// 
    /// The Content-Transfer-Encoding header field can be used to specify the encoding used for the message content.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Content-Transfer-Encoding")]
    ContentTransferEncoding, // https://www.iana.org/go/rfc4021
    /// # Conversion
    /// 
    /// The Conversion header field can be used to specify the conversion information for the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Conversion")]
    Conversion, // https://www.iana.org/go/rfc4021
    /// # Conversion-With-Loss
    /// 
    /// The Conversion-With-Loss header field can be used to specify the conversion information for the message with loss.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Conversion-With-Loss")]
    ConversionWithLoss, // https://www.iana.org/go/rfc4021

    /// # DL-Expansion-History
    /// 
    /// The DL-Expansion-History header field can be used to specify the expansion history of a distribution list.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "DL-Expansion-History")]
    DLExpansionHistory, // https://www.iana.org/go/rfc4021
    /// # Date
    /// 
    /// The Date header field can be used to specify the date and time at which the message was sent.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Date")]
    Date, // https://www.iana.org/go/rfc5322
    /// # Deferred-Delivery
    /// 
    /// The Deferred-Delivery header field can be used to specify the date and time at which the message is to be delivered.
    #[serde(rename = "Deferred-Delivery")]
    DeferredDelivery, // https://www.iana.org/go/rfc4021
    /// # Delivery-Date
    /// 
    /// The Delivery-Date header field can be used to specify the date and time at which the message was delivered.
    #[serde(rename = "Delivery-Date")]
    DeliveryDate, // https://www.iana.org/go/rfc4021
    /// # Discarded-X400-IPMS-Extensions
    /// 
    /// The Discarded-X400-IPMS-Extensions header field can be used to specify the IPMS extensions that were discarded.
    #[serde(rename = "Discarded-X400-IPMS-Extensions")]
    DiscardedX400IPMSExtensions, // https://www.iana.org/go/rfc4021
    /// # Discarded-X400-MTS-Extensions
    /// 
    /// The Discarded-X400-MTS-Extensions header field can be used to specify the MTS extensions that were discarded.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Discarded-X400-MTS-Extensions")]
    DiscardedX400MTSExtensions, // https://www.iana.org/go/rfc4021
    /// # Disclose-Recipients
    /// 
    /// The Disclose-Recipients header field can be used to specify whether the recipients of the message are disclosed.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Disclose-Recipients")]
    DiscloseRecipients, // https://www.iana.org/go/rfc4021
    /// # Disposition-Notification-Options
    /// 
    /// The Disposition-Notification-Options header field can be used to specify the options for disposition notifications.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Disposition-Notification-Options")]
    DispositionNotificationOptions, // https://www.iana.org/go/rfc4021
    /// # Disposition-Notification-To
    /// 
    /// The Disposition-Notification-To header field can be used to specify the recipient for disposition notifications.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Disposition-Notification-To")]
    DispositionNotificationTo, // https://www.iana.org/go/rfc4021
    /// # DKIM-Signature
    /// 
    /// The DKIM-Signature header field can be used to specify the DKIM signature for the message.
    /// Reference: [https://www.iana.org/go/rfc6376](https://www.iana.org/go/rfc6376)
    #[serde(rename = "DKIM-Signature")]
    DKIMSignature, // https://www.iana.org/go/rfc6376
    /// # Downgraded-Final-Recipient
    /// 
    /// The Downgraded-Final-Recipient header field can be used to specify the final recipient of the message.
    /// Reference: [https://www.iana.org/go/rfc6857](https://www.iana.org/go/rfc6857)
    #[serde(rename = "Downgraded-Final-Recipient")]
    DowngradedFinalRecipient, // https://www.iana.org/go/rfc6857
    /// # Downgraded-In-Reply-To
    /// 
    /// The Downgraded-In-Reply-To header field can be used to specify the message that this message is a reply to.
    /// Reference: [https://www.iana.org/go/rfc6857](https://www.iana.org/go/rfc6857)
    #[serde(rename = "Downgraded-In-Reply-To")]
    DowngradedInReplyTo, // https://www.iana.org/go/rfc6857
    /// # Downgraded-Message-Id
    /// 
    /// The Downgraded-Message-Id header field can be used to specify the message identifier of the message.
    /// Reference: [https://www.iana.org/go/rfc6857](https://www.iana.org/go/rfc6857)
    #[serde(rename = "Downgraded-Message-Id")]
    DowngradedMessageId, // https://www.iana.org/go/rfc6857
    /// # Downgraded-Original-Recipient
    /// 
    /// The Downgraded-Original-Recipient header field can be used to specify the original recipient of the message.
    /// Reference: [https://www.iana.org/go/rfc6857](https://www.iana.org/go/rfc6857)
    #[serde(rename = "Downgraded-Original-Recipient")]
    DowngradedOriginalRecipient, // https://www.iana.org/go/rfc6857
    /// # Downgraded-References
    /// 
    /// The Downgraded-References header field can be used to specify the references for the message.
    /// Reference: [https://www.iana.org/go/rfc6857](https://www.iana.org/go/rfc6857)
    #[serde(rename = "Downgraded-References")]
    DowngradedReferences, // https://www.iana.org/go/rfc6857

    /// # Encoding
    /// 
    /// The Encoding header field can be used to specify the encoding of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Encoding")]
    Encoding, // https://www.iana.org/go/rfc4021
    /// # Encrypted
    /// 
    /// The Encrypted header field can be used to specify whether the message is encrypted.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Encrypted")]
    Encrypted, // https://www.iana.org/go/rfc4021
    /// # Expires
    /// 
    /// The Expires header field can be used to specify the expiration date of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Expires")]
    Expires, // https://www.iana.org/go/rfc4021
    /// # Expiry-Date
    /// 
    /// The Expiry-Date header field can be used to specify the expiry date of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Expiry-Date")]
    ExpiryDate, // https://www.iana.org/go/rfc4021

    /// # From
    /// 
    /// The From header field can be used to specify the author of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "From")]
    From, // https://www.iana.org/go/rfc5322 & https://www.iana.org/go/rfc6854

    /// # Generate-Delivery-Report
    /// 
    /// The Generate-Delivery-Report header field can be used to specify whether a delivery report should be generated.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Generate-Delivery-Report")]
    GenerateDeliveryReport, // https://www.iana.org/go/rfc4021

    /// # Importance
    /// 
    /// The Importance header field can be used to specify the importance of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Importance")]
    Importance, // https://www.iana.org/go/rfc4021
    /// # In-Reply-To
    /// 
    /// The In-Reply-To header field can be used to specify the message that this message is a reply to.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "In-Reply-To")]
    /// # Incomplete-Copy
    /// 
    /// The Incomplete-Copy header field can be used to specify that the message is an incomplete copy.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    InReplyTo, // https://www.iana.org/go/rfc5322
    /// # Keywords
    /// 
    /// The Keywords header field can be used to specify the keywords for the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Incomplete-Copy")]
    IncompleteCopy, // https://www.iana.org/go/rfc4021

    /// # Keywords
    /// 
    /// The Keywords header field can be used to specify the keywords for the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Keywords")]
    Keywords, // https://www.iana.org/go/rfc5322

    /// # Language
    /// 
    /// The Language header field can be used to specify the language of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Language")]
    Language, // https://www.iana.org/go/rfc4021
    /// # Latest-Delivery-Time
    /// 
    /// The Latest-Delivery-Time header field can be used to specify the latest time at which the message should be delivered.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Latest-Delivery-Time")]
    LatestDeliveryTime, // https://www.iana.org/go/rfc4021
    /// # List-Archive
    /// 
    /// The List-Archive header field can be used to specify the URI for the list archive.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "List-Archive")]
    ListArchive, // https://www.iana.org/go/rfc4021
    /// # List-Help
    /// 
    /// The List-Help header field can be used to specify the URI for the list help.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "List-Help")]
    ListHelp, // https://www.iana.org/go/rfc4021
    /// # List-Id
    /// 
    /// The List-Id header field can be used to specify the identifier for the list.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "List-Id")]
    ListId, // https://www.iana.org/go/rfc4021
    /// # List-Owner
    /// 
    /// The List-Owner header field can be used to specify the owner of the list.
    #[serde(rename = "List-Owner")]
    ListOwner, // https://www.iana.org/go/rfc4021
    /// # List-Post
    /// 
    /// The List-Post header field can be used to specify the URI for posting to the list.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "List-Post")]
    ListPost, // https://www.iana.org/go/rfc4021
    /// # List-Subscribe
    /// 
    /// The List-Subscribe header field can be used to specify the URI for subscribing to the list.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "List-Subscribe")]
    ListSubscribe, // https://www.iana.org/go/rfc4021
    /// # List-Unsubscribe
    /// 
    /// The List-Unsubscribe header field can be used to specify the URI for unsubscribing from the list.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "List-Unsubscribe")]
    ListUnsubscribe, // https://www.iana.org/go/rfc4021
    /// # List-Unsubscribe-Post
    /// 
    /// The List-Unsubscribe-Post header field can be used to specify the URI for unsubscribing from the list.
    /// Reference: [https://www.iana.org/go/rfc8058](https://www.iana.org/go/rfc8058)
    #[serde(rename = "List-Unsubscribe-Post")]
    ListUnsubscribePost, // https://www.iana.org/go/rfc8058

    /// # Message-Context
    /// 
    /// The Message-Context header field can be used to specify the context of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Message-Context")]
    MessageContext, // https://www.iana.org/go/rfc4021
    /// # Message-Id
    /// 
    /// The Message-Id header field can be used to specify the identifier for the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Message-Id")]
    MessageId, // https://www.iana.org/go/rfc5322
    /// # Message-Type
    /// 
    /// The Message-Type header field can be used to specify the type of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Message-Type")]
    MessageType, // https://www.iana.org/go/rfc4021
    /// # MIME-Type
    /// 
    /// The MIME-Type header field can be used to specify the MIME type of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "MIME-Type")]
    MIMEType, // https://www.iana.org/go/rfc4021
    /// # MIME-Version
    /// 
    /// The MIME-Version header field can be used to specify the MIME version of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "MIME-Version")]
    MIMEVersion, // https://www.iana.org/go/rfc4021
    /// # MT-Priority
    /// 
    /// The MT-Priority header field can be used to specify the priority of the message.
    /// Reference: [https://www.iana.org/go/rfc6758](https://www.iana.org/go/rfc6758)
    #[serde(rename = "MT-Priority")]
    MTPriority, // https://www.iana.org/go/rfc6758

    /// # Obsoletes
    /// 
    /// The Obsoletes header field can be used to specify the messages that this message obsoletes.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Obsoletes")]
    Obsoletes, // https://www.iana.org/go/rfc4021
    /// # Organization
    /// 
    /// The Organization header field can be used to specify the organization of the message.
    /// Reference: [https://www.iana.org/go/rfc7681](https://www.iana.org/go/rfc7681)
    #[serde(rename = "Organization")]
    Organization, // https://www.iana.org/go/rfc7681
    /// # Original-Encoded-Information-Types
    /// 
    /// The Original-Encoded-Information-Types header field can be used to specify the encoded information types for the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Original-Encoded-Information-Types")]
    OriginalEncodedInformationTypes, // https://www.iana.org/go/rfc4021
    /// # Original-From
    /// 
    /// The Original-From header field can be used to specify the original author of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Original-From")]
    OriginalFrom, // https://www.iana.org/go/rfc4021
    /// # Original-Message-Id
    /// 
    /// The Original-Message-Id header field can be used to specify the original message identifier of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Original-Message-Id")]
    OriginalMessageId, // https://www.iana.org/go/rfc4021
    /// # Original-Recipient
    /// 
    /// The Original-Recipient header field can be used to specify the original recipient of the message.
    /// Reference: [https://www.iana.org/go/rfc3798](https://www.iana.org/go/rfc3798)
    #[serde(rename = "Original-Recipient")]
    OriginalRecipient, // https://www.iana.org/go/rfc3798 & https://www.iana.org/go/rfc3798
    /// # Originator-Return-Address
    /// 
    /// The Originator-Return-Address header field can be used to specify the return address for non-delivery reports.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Originator-Return-Address")]
    OriginatorReturnAddress, // https://www.iana.org/go/rfc4021
    /// # Original-Subject
    /// 
    /// The Original-Subject header field can be used to specify the original subject of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Original-Subject")]
    OriginalSubject, // https://www.iana.org/go/rfc5703

    /// # PICS-Label
    /// 
    /// The PICS-Label header field can be used to specify the PICS label for the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "PICS-Label")]
    PICSLabel, // https://www.iana.org/go/rfc4021
    /// # Prevent-NonDelivery-Report
    /// 
    /// The Prevent-NonDelivery-Report header field can be used to specify that non-delivery reports should not be generated.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Prevent-NonDelivery-Report")]
    PreventNonDeliveryReport, // https://www.iana.org/go/rfc4021
    /// # Priority
    /// 
    /// The Priority header field can be used to specify the priority of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Priority")]
    Priority, // https://www.iana.org/go/rfc4021

    /// # Received
    /// 
    /// The Received header field can be used to specify the date and time at which the message was received.
    /// Reference: [https://www.iana.org/go/rfc5321](https://www.iana.org/go/rfc5321)
    #[serde(rename = "Received")]
    Received, // https://www.iana.org/go/rfc5321
    /// # Received-SPF
    /// 
    /// The Received-SPF header field can be used to specify the results of the SPF check.
    /// Reference: [https://www.iana.org/go/rfc7208](https://www.iana.org/go/rfc7208)
    #[serde(rename = "Received-SPF")]
    ReceivedSPF, // https://www.iana.org/go/rfc7208
    /// # References
    /// 
    /// The References header field can be used to specify the references for the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "References")]
    References, // https://www.iana.org/go/rfc5322
    /// # Reply-By
    /// 
    /// The Reply-By header field can be used to specify the date and time by which a reply is requested.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Reply-By")]
    ReplyBy, // https://www.iana.org/go/rfc4021
    /// # Reply-To
    /// 
    /// The Reply-To header field can be used to specify the address to which replies should be sent.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Reply-To")]
    ReplyTo, // https://www.iana.org/go/rfc5322
    /// # Require-Recipient-Valid-Since
    /// 
    /// The Require-Recipient-Valid-Since header field can be used to specify the date and time at which the recipient was validated.
    /// Reference: [https://www.iana.org/go/rfc7293](https://www.iana.org/go/rfc7293)
    #[serde(rename = "Require-Recipient-Valid-Since")]
    RequireRecipientValidSince, // https://www.iana.org/go/rfc7293
    /// # Resent-Bcc
    /// 
    /// The Resent-Bcc header field can be used to specify recipients who are not to be included in the delivery of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Resent-Bcc")]
    ResentBcc, // https://www.iana.org/go/rfc5322
    /// # Resent-Cc
    /// 
    /// The Resent-Cc header field can be used to specify recipients who are to receive copies of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Resent-Cc")]
    ResentCc, // https://www.iana.org/go/rfc5322
    /// # Resent-Date
    /// 
    /// The Resent-Date header field can be used to specify the date and time at which the message was resent.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Resent-Date")]
    ResentDate, // https://www.iana.org/go/rfc5322
    /// # Resent-From
    /// 
    /// The Resent-From header field can be used to specify the sender of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Resent-From")]
    ResentFrom, // https://www.iana.org/go/rfc5322
    /// # Resent-Message-Id
    /// 
    /// The Resent-Message-Id header field can be used to specify the message identifier of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Resent-Message-Id")]
    ResentMessageId, // https://www.iana.org/go/rfc5322
    /// # Resent-Reply-To
    /// 
    /// The Resent-Reply-To header field can be used to specify the reply-to address for the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Resent-Reply-To")]
    ResentReplyTo, // https://www.iana.org/go/rfc5322
    /// # Resent-Sender
    /// 
    /// The Resent-Sender header field can be used to specify the sender of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Resent-Sender")]
    ResentSender, // https://www.iana.org/go/rfc5322
    /// # Resent-To
    /// 
    /// The Resent-To header field can be used to specify the recipients of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Resent-To")]
    ResentTo, // https://www.iana.org/go/rfc5322
    /// # Return-Path
    /// 
    /// The Return-Path header field can be used to specify the return path for the message.
    /// Reference: [https://www.iana.org/go/rfc5321](https://www.iana.org/go/rfc5321)
    #[serde(rename = "Return-Path")]
    ReturnPath, // https://www.iana.org/go/rfc5321

    /// # Sender
    /// 
    /// The Sender header field can be used to specify the sender of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Sender")]
    Sender, // https://www.iana.org/go/rfc5322
    /// # Sensitivity
    /// 
    /// The Sensitivity header field can be used to specify the sensitivity of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Sensitivity")]
    Sensitivity, // https://www.iana.org/go/rfc4021
    /// # Solicitation
    /// 
    /// The Solicitation header field can be used to specify the solicitation for the message.
    /// Reference: [https://www.iana.org/go/rfc3865](https://www.iana.org/go/rfc3865)
    #[serde(rename = "Solicitation")]
    Solicitation, // https://www.iana.org/go/rfc3865
    /// # Subject
    /// 
    /// The Subject header field can be used to specify the subject of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "Subject")]
    Subject, // https://www.iana.org/go/rfc5322
    /// # Supersedes
    /// 
    /// The Supersedes header field can be used to specify the message that this message supersedes.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "Supersedes")]
    Supersedes, // https://www.iana.org/go/rfc4021

    /// # TLS-Report-Domain
    /// 
    /// The TLS-Report-Domain header field can be used to specify the domain for the TLS report.
    /// Reference: [https://www.iana.org/go/rfc8460](https://www.iana.org/go/rfc8460)
    #[serde(rename = "TLS-Report-Domain")]
    TLSReportDomain, // https://www.iana.org/go/rfc8460
    /// # TLS-Report-Domain
    /// 
    /// The TLS-Report-Domain header field can be used to specify the domain for the TLS report.
    /// Reference: [https://www.iana.org/go/rfc8460](https://www.iana.org/go/rfc8460)
    #[serde(rename = "TLS-Report-Submitter")]
    TLSReportSubmitter, // https://www.iana.org/go/rfc8460
    /// # TLS-Required
    /// 
    /// The TLS-Required header field can be used to specify that TLS is required for the message.
    /// Reference: [https://www.iana.org/go/rfc8689](https://www.iana.org/go/rfc8689)
    #[serde(rename = "TlS-Required")]
    TLSRequired, // https://www.iana.org/go/rfc8689
    /// # To
    /// 
    /// The To header field can be used to specify the recipients of the message.
    /// Reference: [https://www.iana.org/go/rfc5322](https://www.iana.org/go/rfc5322)
    #[serde(rename = "To")]
    To, // https://www.iana.org/go/rfc5322

    /// # VBR-Info
    /// 
    /// The VBR-Info header field can be used to specify the VBR information for the message.
    /// Reference: [https://www.iana.org/go/rfc5518](https://www.iana.org/go/rfc5518)
    #[serde(rename = "VBR-Info")]
    VBRInfo, // https://www.iana.org/go/rfc5518

    /// # X400-Content-Identifier
    /// 
    /// The X400-Content-Identifier header field can be used to specify the content identifier for the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "X400-Content-Identifier")]
    X400ContentIdentifier, // https://www.iana.org/go/rfc4021
    /// # X400-Content-Return
    /// 
    /// The X400-Content-Return header field can be used to specify the return address for non-delivery reports.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "X400-Content-Return")]
    X400ContentReturn, // https://www.iana.org/go/rfc4021
    /// # X400-Content-Type
    /// 
    /// The X400-Content-Type header field can be used to specify the content type of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "X400-Content-Type")]
    X400ContentType, // https://www.iana.org/go/rfc4021
    /// # X400-MTS-Identifier
    /// 
    /// The X400-MTS-Identifier header field can be used to specify the MTS identifier for the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "X400-MTS-Identifier")]
    X400MTSIdentifier, // https://www.iana.org/go/rfc4021
    /// # X400-Originator
    /// 
    /// The X400-Originator header field can be used to specify the originator of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "X400-Originator")]
    X400Originator, // https://www.iana.org/go/rfc4021
    /// # X400-Received
    /// 
    /// The X400-Received header field can be used to specify the received information for the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "X400-Received")]
    X400Received, // https://www.iana.org/go/rfc4021
    /// # X400-Recipients
    /// 
    /// The X400-Recipients header field can be used to specify the recipients of the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "X400-Recipients")]
    X400Recipients, // https://www.iana.org/go/rfc4021
    /// # X400-Trace
    /// 
    /// The X400-Trace header field can be used to specify the trace information for the message.
    /// Reference: [https://www.iana.org/go/rfc4021](https://www.iana.org/go/rfc4021)
    #[serde(rename = "X400-Trace")]
    X400Trace, // https://www.iana.org/go/rfc4021

    /// # Unknown
    /// 
    /// The Unknown header field can be used to specify an unknown header.
    Unknown(String),
}

/// # Email Headers Implementation
/// 
/// This implementation is for the EmailHeaders enum.
impl EmailHeaders {
    /// # From Bytes
    /// 
    /// This function creates a new EmailHeaders from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let s = from_utf8(bytes).map_err(|_| "Invalid header")?;
        Ok(EmailHeaders::from_str(s).unwrap_or(EmailHeaders::Unknown(s.to_string())))
    }

    /// # To String
    /// 
    /// This function converts the email header to a string.
    pub fn to_string(&self) -> &str {
        match self {
            EmailHeaders::AcceptLanguage => "Accept-Language",
            EmailHeaders::AlternateRecipient => "Alternate-Recipient",
            #[cfg(feature = "smtp-experimental-headers")]
            EmailHeaders::ARCAuthenticationResults => "ARC-Authentication-Results",
            #[cfg(feature = "smtp-experimental-headers")]
            EmailHeaders::ARCMessageSignature => "ARC-Message-Signature",
            #[cfg(feature = "smtp-experimental-headers")]
            EmailHeaders::ARCSeal => "ARC-Seal",
            EmailHeaders::ArchivedAt => "Archived-At",
            EmailHeaders::AuthenticationResults => "Authentication-Results",
            EmailHeaders::AutoSubmitted => "Auto-Submitted",
            EmailHeaders::AutoForwarded => "Autoforwarded",
            EmailHeaders::Autosubmitted => "Autosubmitted",
            EmailHeaders::Bcc => "Bcc",
            EmailHeaders::Cc => "Cc",
            EmailHeaders::Comments => "Comments",
            EmailHeaders::ContentIdentifier => "Content-Identifier",
            EmailHeaders::ContentReturn => "Content-Return",
            EmailHeaders::ContentType => "Content-Type",
            EmailHeaders::ContentTransferEncoding => "Content-Transfer-Encoding",
            EmailHeaders::Conversion => "Conversion",
            EmailHeaders::ConversionWithLoss => "Conversion-With-Loss",
            EmailHeaders::DLExpansionHistory => "DL-Expansion-History",
            EmailHeaders::Date => "Date",
            EmailHeaders::DeferredDelivery => "Deferred-Delivery",
            EmailHeaders::DeliveryDate => "Delivery-Date",
            EmailHeaders::DiscardedX400IPMSExtensions => "Discarded-X400-IPMS-Extensions",
            EmailHeaders::DiscardedX400MTSExtensions => "Discarded-X400-MTS-Extensions",
            EmailHeaders::DiscloseRecipients => "Disclose-Recipients",
            EmailHeaders::DispositionNotificationOptions => "Disposition-Notification-Options",
            EmailHeaders::DispositionNotificationTo => "Disposition-Notification-To",
            EmailHeaders::DKIMSignature => "DKIM-Signature",
            EmailHeaders::DowngradedFinalRecipient => "Downgraded-Final-Recipient",
            EmailHeaders::DowngradedInReplyTo => "Downgraded-In-Reply-To",
            EmailHeaders::DowngradedMessageId => "Downgraded-Message-Id",
            EmailHeaders::DowngradedOriginalRecipient => "Downgraded-Original-Recipient",
            EmailHeaders::DowngradedReferences => "Downgraded-References",
            EmailHeaders::Encoding => "Encoding",
            EmailHeaders::Encrypted => "Encrypted",
            EmailHeaders::Expires => "Expires",
            EmailHeaders::ExpiryDate => "Expiry-Date",
            EmailHeaders::From => "From",
            EmailHeaders::GenerateDeliveryReport => "Generate-Delivery-Report",
            EmailHeaders::Importance => "Importance",
            EmailHeaders::InReplyTo => "In-Reply-To",
            EmailHeaders::IncompleteCopy => "Incomplete-Copy",
            EmailHeaders::Keywords => "Keywords",
            EmailHeaders::Language => "Language",
            EmailHeaders::LatestDeliveryTime => "Latest-Delivery-Time",
            EmailHeaders::ListArchive => "List-Archive",
            EmailHeaders::ListHelp => "List-Help",
            EmailHeaders::ListId => "List-Id",
            EmailHeaders::ListOwner => "List-Owner",
            EmailHeaders::ListPost => "List-Post",
            EmailHeaders::ListSubscribe => "List-Subscribe",
            EmailHeaders::ListUnsubscribe => "List-Unsubscribe",
            EmailHeaders::ListUnsubscribePost => "List-Unsubscribe-Post",
            EmailHeaders::MessageContext => "Message-Context",
            EmailHeaders::MessageId => "Message-Id",
            EmailHeaders::MessageType => "Message-Type",
            EmailHeaders::MIMEType => "MIME-Type",
            EmailHeaders::MIMEVersion => "MIME-Version",
            EmailHeaders::MTPriority => "MT-Priority",
            EmailHeaders::Obsoletes => "Obsoletes",
            EmailHeaders::Organization => "Organization",
            EmailHeaders::OriginalEncodedInformationTypes => "Original-Encoded-Information-Types",
            EmailHeaders::OriginalFrom => "Original-From",
            EmailHeaders::OriginalMessageId => "Original-Message-Id",
            EmailHeaders::OriginalRecipient => "Original-Recipient",
            EmailHeaders::OriginatorReturnAddress => "Originator-Return-Address",
            EmailHeaders::OriginalSubject => "Original-Subject",
            EmailHeaders::PICSLabel => "PICS-Label",
            EmailHeaders::PreventNonDeliveryReport => "Prevent-NonDelivery-Report",
            EmailHeaders::Priority => "Priority",
            EmailHeaders::Received => "Received",
            EmailHeaders::ReceivedSPF => "Received-SPF",
            EmailHeaders::References => "References",
            EmailHeaders::ReplyBy => "Reply-By",
            EmailHeaders::ReplyTo => "Reply-To",
            EmailHeaders::RequireRecipientValidSince => "Require-Recipient-Valid-Since",
            EmailHeaders::ResentBcc => "Resent-Bcc",
            EmailHeaders::ResentCc => "Resent-Cc",
            EmailHeaders::ResentDate => "Resent-Date",
            EmailHeaders::ResentFrom => "Resent-From",
            EmailHeaders::ResentMessageId => "Resent-Message-Id",
            EmailHeaders::ResentReplyTo => "Resent-Reply-To",
            EmailHeaders::ResentSender => "Resent-Sender",
            EmailHeaders::ResentTo => "Resent-To",
            EmailHeaders::ReturnPath => "Return-Path",
            EmailHeaders::Sender => "Sender",
            EmailHeaders::Sensitivity => "Sensitivity",
            EmailHeaders::Solicitation => "Solicitation",
            EmailHeaders::Subject => "Subject",
            EmailHeaders::Supersedes => "Supersedes",
            EmailHeaders::TLSReportDomain => "TLS-Report-Domain",
            EmailHeaders::TLSReportSubmitter => "TLS-Report-Submitter",
            EmailHeaders::TLSRequired => "TlS-Required",
            EmailHeaders::To => "To",
            EmailHeaders::VBRInfo => "VBR-Info",
            EmailHeaders::X400ContentIdentifier => "X400-Content-Identifier",
            EmailHeaders::X400ContentReturn => "X400-Content-Return",
            EmailHeaders::X400ContentType => "X400-Content-Type",
            EmailHeaders::X400MTSIdentifier => "X400-MTS-Identifier",
            EmailHeaders::X400Originator => "X400-Originator",
            EmailHeaders::X400Received => "X400-Received",
            EmailHeaders::X400Recipients => "X400-Recipients",
            EmailHeaders::X400Trace => "X400-Trace",
            EmailHeaders::Unknown(ref s) => s,
        }
    }

    /// # From String
    /// 
    /// This function creates a new EmailHeaders from a string.
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "accept-language" => EmailHeaders::AcceptLanguage,
            "alternate-recipient" => EmailHeaders::AlternateRecipient,
            #[cfg(feature = "smtp-experimental-headers")]
            "arc-authentication-results" => EmailHeaders::ARCAuthenticationResults,
            #[cfg(feature = "smtp-experimental-headers")]
            "arc-message-signature" => EmailHeaders::ARCMessageSignature,
            #[cfg(feature = "smtp-experimental-headers")]
            "arc-seal" => EmailHeaders::ARCSeal,
            "archived-at" => EmailHeaders::ArchivedAt,
            "authentication-results" => EmailHeaders::AuthenticationResults,
            "auto-submitted" => EmailHeaders::AutoSubmitted,
            "autoforwarded" => EmailHeaders::AutoForwarded,
            "autosubmitted" => EmailHeaders::Autosubmitted,
            "bcc" => EmailHeaders::Bcc,
            "cc" => EmailHeaders::Cc,
            "comments" => EmailHeaders::Comments,
            "content-identifier" => EmailHeaders::ContentIdentifier,
            "content-return" => EmailHeaders::ContentReturn,
            "content-type" => EmailHeaders::ContentType,
            "content-transfer-encoding" => EmailHeaders::ContentTransferEncoding,
            "conversion" => EmailHeaders::Conversion,
            "conversion-with-loss" => EmailHeaders::ConversionWithLoss,
            "dl-expansion-history" => EmailHeaders::DLExpansionHistory,
            "date" => EmailHeaders::Date,
            "deferred-delivery" => EmailHeaders::DeferredDelivery,
            "delivery-date" => EmailHeaders::DeliveryDate,
            "discarded-x400-ipms-extensions" => EmailHeaders::DiscardedX400IPMSExtensions,
            "discarded-x400-mts-extensions" => EmailHeaders::DiscardedX400MTSExtensions,
            "disclose-recipients" => EmailHeaders::DiscloseRecipients,
            "disposition-notification-options" => EmailHeaders::DispositionNotificationOptions,
            "disposition-notification-to" => EmailHeaders::DispositionNotificationTo,
            "dkim-signature" => EmailHeaders::DKIMSignature,
            "downgraded-final-recipient" => EmailHeaders::DowngradedFinalRecipient,
            "downgraded-in-reply-to" => EmailHeaders::DowngradedInReplyTo,
            "downgraded-message-id" => EmailHeaders::DowngradedMessageId,
            "downgraded-original-recipient" => EmailHeaders::DowngradedOriginalRecipient,
            "downgraded-references" => EmailHeaders::DowngradedReferences,
            "encoding" => EmailHeaders::Encoding,
            "encrypted" => EmailHeaders::Encrypted,
            "expires" => EmailHeaders::Expires,
            "expiry-date" => EmailHeaders::ExpiryDate,
            "from" => EmailHeaders::From,
            "generate-delivery-report" => EmailHeaders::GenerateDeliveryReport,
            "importance" => EmailHeaders::Importance,
            "in-reply-to" => EmailHeaders::InReplyTo,
            "incomplete-copy" => EmailHeaders::IncompleteCopy,
            "keywords" => EmailHeaders::Keywords,
            "language" => EmailHeaders::Language,
            "latest-delivery-time" => EmailHeaders::LatestDeliveryTime,
            "list-archive" => EmailHeaders::ListArchive,
            "list-help" => EmailHeaders::ListHelp,
            "list-id" => EmailHeaders::ListId,
            "list-owner" => EmailHeaders::ListOwner,
            "list-post" => EmailHeaders::ListPost,
            "list-subscribe" => EmailHeaders::ListSubscribe,
            "list-unsubscribe" => EmailHeaders::ListUnsubscribe,
            "list-unsubscribe-post" => EmailHeaders::ListUnsubscribePost,
            "message-context" => EmailHeaders::MessageContext,
            "message-id" => EmailHeaders::MessageId,
            "message-type" => EmailHeaders::MessageType,
            "mime-type" => EmailHeaders::MIMEType,
            "mime-version" => EmailHeaders::MIMEVersion,
            "mt-priority" => EmailHeaders::MTPriority,
            "obsoletes" => EmailHeaders::Obsoletes,
            "organization" => EmailHeaders::Organization,
            "original-encoded-information-types" => EmailHeaders::OriginalEncodedInformationTypes,
            "original-from" => EmailHeaders::OriginalFrom,
            "original-message-id" => EmailHeaders::OriginalMessageId,
            "original-recipient" => EmailHeaders::OriginalRecipient,
            "originator-return-address" => EmailHeaders::OriginatorReturnAddress,
            "original-subject" => EmailHeaders::OriginalSubject,
            "pics-label" => EmailHeaders::PICSLabel,
            "prevent-nondelivery-report" => EmailHeaders::PreventNonDeliveryReport,
            "priority" => EmailHeaders::Priority,
            "received" => EmailHeaders::Received,
            "received-spf" => EmailHeaders::ReceivedSPF,
            "references" => EmailHeaders::References,
            "reply-by" => EmailHeaders::ReplyBy,
            "reply-to" => EmailHeaders::ReplyTo,
            "require-recipient-valid-since" => EmailHeaders::RequireRecipientValidSince,
            "resent-bcc" => EmailHeaders::ResentBcc,
            "resent-cc" => EmailHeaders::ResentCc,
            "resent-date" => EmailHeaders::ResentDate,
            "resent-from" => EmailHeaders::ResentFrom,
            "resent-message-id" => EmailHeaders::ResentMessageId,
            "resent-reply-to" => EmailHeaders::ResentReplyTo,
            "resent-sender" => EmailHeaders::ResentSender,
            "resent-to" => EmailHeaders::ResentTo,
            "return-path" => EmailHeaders::ReturnPath,
            "sender" => EmailHeaders::Sender,
            "sensitivity" => EmailHeaders::Sensitivity,
            "solicitation" => EmailHeaders::Solicitation,
            "subject" => EmailHeaders::Subject,
            "supersedes" => EmailHeaders::Supersedes,
            "tls-report-domain" => EmailHeaders::TLSReportDomain,
            "tls-report-submitter" => EmailHeaders::TLSReportSubmitter,
            "tls-required" => EmailHeaders::TLSRequired,
            "to" => EmailHeaders::To,
            "vbr-info" => EmailHeaders::VBRInfo,
            "x400-content-identifier" => EmailHeaders::X400ContentIdentifier,
            "x400-content-return" => EmailHeaders::X400ContentReturn,
            "x400-content-type" => EmailHeaders::X400ContentType,
            "x400-mts-identifier" => EmailHeaders::X400MTSIdentifier,
            "x400-originator" => EmailHeaders::X400Originator,
            "x400-received" => EmailHeaders::X400Received,
            "x400-recipients" => EmailHeaders::X400Recipients,
            "x400-trace" => EmailHeaders::X400Trace,
            _ => EmailHeaders::Unknown(s.to_string()),
        }
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
