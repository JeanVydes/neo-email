use std::str::from_utf8;

use crate::errors::Error;

use super::headers::EmailHeaders;
use hashbrown::HashMap;

/// # Mail
///
/// This struct represents an email message.
///
/// ## Fields
///
/// * `headers` - A HashMap of EmailHeaders and its values.
/// * `body` - The body of the email.
/// 
/// ## Example
/// 
/// ```rust
/// use neo_email::mail::Mail;
/// 
/// let raw_email = b"From: Jean<jean@nervio.com>\nSubject: Hello\n\nHello, World!";
/// let mail = Mail::<Vec<u8>>::from_bytes(raw_email.to_vec()).unwrap();
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Mail<T> {
    /// # Headers
    ///
    /// A HashMap of EmailHeaders and its values.
    ///
    /// ## Example
    ///
    /// `From -> "jean@nervio.us"`
    pub headers: HashMap<EmailHeaders, String>,
    /// # Body
    ///
    /// The body of the email.
    pub body: T,
}

impl<T> Mail<T> {
    /// # From Bytes
    /// 
    /// This function creates a new Mail from bytes.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use neo_email::mail::Mail;
    /// 
    /// let raw_email = b"From: Jean<jean@nervio.com>\nSubject: Hello\n\nHello, World!";
    /// let mail = Mail::<Vec<u8>>::from_bytes(raw_email.to_vec()).unwrap();
    /// ```
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Mail<T>, String>
    where
        T: From<Vec<u8>>,
    {
        let mut headers = HashMap::new();
        let mut body = Vec::new();
        let mut lines = bytes.split(|&b| b == b'\n').peekable();
        let mut header_complete = false;

        while let Some(line) = lines.next() {
            if line.is_empty() || line == b"\r" {
                header_complete = true;
                break;
            }

            if let Some(&b' ') | Some(&b'\t') = line.first() {
                if let Some(last_header) = headers.keys().last().cloned() {
                    let value: &mut String = headers.get_mut(&last_header).unwrap();
                    value.push_str(from_utf8(line).map_err(|_| "Invalid header value")?);
                    continue;
                }
            }

            let mut parts = line.splitn(2, |&b| b == b':');
            let key = parts.next().ok_or("Invalid header")?;
            let value = parts.next().ok_or("Invalid header value not exist")?;
            let value = from_utf8(value).map_err(|_| "Invalid header value")?.trim();
            let value = value.split_whitespace().collect::<Vec<&str>>().join(" ");

            headers.insert(EmailHeaders::from_bytes(key)?, value.to_owned());
        }

        if header_complete {
            for line in lines {
                body.extend_from_slice(line);
                body.push(b'\n');
            }
        } else {
            return Err("Invalid mail format".to_string());
        }

        Ok(Mail {
            headers,
            body: body.into(),
        })
    }
}

/// # Mail Trait
/// 
/// This trait is implemented by Mail and is used to downcast the Mail struct.
pub trait MailTrait: Send + Sync + 'static {
    /// # As Any
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: Send + Sync + 'static> MailTrait for Mail<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl<T: Clone + Send + Sync + 'static> Clone for Mail<T> {
    fn clone(&self) -> Self {
        Mail {
            headers: self.headers.clone(),
            body: self.body.clone(),
        }
    }
}

/// # Email Address
/// 
/// This struct represents an email address.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress {
    /// # Username
    /// 
    /// The username of the email address.
    /// 
    /// ## Example
    /// 
    /// `jean`
    pub username: String,
    /// # Domain
    /// 
    /// The domain of the email address.
    /// 
    /// ## Example
    pub domain: String,
}

impl EmailAddress {
    /// # From String
    /// 
    /// This function creates a new EmailAddress from a string.
    pub fn from_string(data: &str) -> Result<Self, Error> {
        let mut parts = data.split('@');
        let username = parts
            .next()
            .ok_or(Error::ParseError("Invalid email address".to_string()))?
            .to_owned();

        if username.is_empty() {
            return Err(Error::ParseError("Invalid email address".to_string()));
        }

        if username.len() > 64 {
            return Err(Error::ParseError("Invalid email address".to_string()));
        }

        let domain = parts
            .next()
            .ok_or(Error::ParseError("Invalid email address".to_string()))?
            .to_owned();

        if domain.is_empty() {
            return Err(Error::ParseError("Invalid email address".to_string()));
        }

        if domain.len() > 253 {
            return Err(Error::ParseError("Invalid email address".to_string()));
        }

        Ok(EmailAddress { username, domain })
    }

    /// # To String
    /// 
    /// This function converts EmailAdress to a String.
    pub fn to_string(&self) -> String {
        format!("{}@{}", self.username, self.domain)
    }
}
