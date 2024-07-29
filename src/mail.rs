use std::str::from_utf8;

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

        Ok(Mail { headers, body: body.into() })
    }
}

pub trait MailTrait: Send + Sync + 'static {
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
