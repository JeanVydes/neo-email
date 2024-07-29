use std::sync::Arc;

use hashbrown::HashMap;
use tokio::sync::Mutex;
use trust_dns_resolver::Resolver;

use crate::{errors::SMTPError, headers::EmailHeaders, mail::Mail};

#[derive(Debug, Clone, PartialEq, Eq)]
struct DkimSignatureHeader {
    version: String,                      // DKIM version
    algorithm: String,                    // Algorithm used for the signature
    canonicalization: String,             // Canonicalization methods for header and body
    domain: String,                       // Domain used in the DKIM signature
    selector: String,                     // Selector for the DKIM key
    timestamp: Option<u64>,               // Unix timestamp for when the signature was generated
    body_hash: Option<String>,            // Hash of the email body
    header_list: Option<Vec<String>>,     // List of headers included in the signature
    signature: String,                    // Actual digital signature in base64 encoding
}

impl DkimSignatureHeader {
    fn new(
        version: String,
        algorithm: String,
        canonicalization: String,
        domain: String,
        selector: String,
        signature: String,
        timestamp: Option<u64>,
        body_hash: Option<String>,
        header_list: Option<Vec<String>>,
    ) -> Self {
        DkimSignatureHeader {
            version,
            algorithm,
            canonicalization,
            domain,
            selector,
            timestamp,
            body_hash,
            header_list,
            signature,
        }
    }

    fn to_string(&self) -> String {
        let mut header = format!(
            "v={}; a={}; c={}; d={}; s={}; b={}",
            self.version,
            self.algorithm,
            self.canonicalization,
            self.domain,
            self.selector,
            self.signature,
        );

        if let Some(timestamp) = self.timestamp {
            header.push_str(&format!("; t={}", timestamp));
        }
        if let Some(ref body_hash) = self.body_hash {
            header.push_str(&format!("; bh={}", body_hash));
        }
        if let Some(ref header_list) = self.header_list {
            header.push_str(&format!("; h={}", header_list.join(":")));
        }

        header
    }

    fn from_string(header: &str) -> Result<Self, SMTPError> {
        let mut fields: HashMap<&str, &str> = HashMap::new();

        for part in header.split(';') {
            let mut kv = part.splitn(2, '=');
            let key = kv.next().ok_or(SMTPError::DKIMError("Invalid DKIM-Signature header".to_owned()))?.trim();
            let value = kv.next().ok_or(SMTPError::DKIMError("Invalid DKIM-Signature header".to_owned()))?.trim();
            fields.insert(key, value);
        }

        let version = fields.get("v").ok_or(SMTPError::DKIMError("Missing version field".to_owned()))?.to_string();
        let algorithm = fields.get("a").ok_or(SMTPError::DKIMError("Missing algorithm field".to_owned()))?.to_string();
        let canonicalization = fields.get("c").ok_or(SMTPError::DKIMError("Missing canonicalization field".to_owned()))?.to_string();
        let domain = fields.get("d").ok_or(SMTPError::DKIMError("Missing domain field".to_owned()))?.to_string();
        let selector = fields.get("s").ok_or(SMTPError::DKIMError("Missing selector field".to_owned()))?.to_string();
        let signature = fields.get("b").ok_or(SMTPError::DKIMError("Missing signature field".to_owned()))?.to_string();

        let timestamp = fields.get("t").map(|t| t.parse::<u64>().ok()).flatten();
        let body_hash = fields.get("bh").map(|bh| bh.to_string());
        let header_list = fields.get("h").map(|h| h.split(':').map(|s| s.to_string()).collect());

        Ok(DkimSignatureHeader {
            version,
            algorithm,
            canonicalization,
            domain,
            selector,
            timestamp,
            body_hash,
            header_list,
            signature,
        })
    }
}

pub async fn verify_dkim_from_email<'a, T>(
    dns_resolver: Arc<Mutex<Resolver>>,
    mail: Mail<T>,
) -> Result<bool, SMTPError> {
    let dkim_signature = mail
        .headers
        .get(&EmailHeaders::DKIMSignature)
        .ok_or(SMTPError::DKIMError("DKIM Signature not found".to_owned()))?;
    let dkim_signature = DkimSignatureHeader::from_string(dkim_signature)?;
    let dkim_record = query_dkim_record(dns_resolver.clone(), &dkim_signature).await?;
    let dkim_record = DkimPublicKey::from_record(&dkim_record)?;
    let alg = dkim_signature.algorithm.to_lowercase();

    let result = match alg.as_str() {
        "rsa-sha1" => {
            // Verify using RSA-SHA1
            // ...
            true
        }
        "rsa-sha256" => {
            // Verify using RSA-SHA256
            // ...
            true
        }
        _ => false,
    };

    Ok(result)
}

pub async fn query_dkim_record(
    dns_resolver: Arc<Mutex<Resolver>>,
    dkim_signature: &DkimSignatureHeader,
) -> Result<String, SMTPError> {
    let selector = &dkim_signature.selector;
    let domain = &dkim_signature.domain;

    let query_name = format!("{}._domainkey.{}.", selector, domain);

    let dns_resolver = dns_resolver.lock().await;

    match dns_resolver.txt_lookup(&query_name) {
        Ok(response) => {
            for txt in response.iter() {
                let txt_data = txt.to_string();
                if txt_data.contains(&format!("v={}", dkim_signature.version)) {
                    return Ok(txt_data);
                }
            }
            Err(SMTPError::DKIMError("DKIM record not found".to_owned()))
        }
        Err(e) => Err(SMTPError::DKIMError(format!("DNS query failed: {}", e))),
    }
}

#[derive(Debug, Clone)]
pub struct DkimPublicKey {
    pub pub_key: String,
    pub flags: Option<String>,
    pub hash_algo: Option<String>,
}

impl DkimPublicKey {
    pub fn from_record(record: &str) -> Result<DkimPublicKey, SMTPError> {
        let mut fields: HashMap<&str, &str> = HashMap::new();

        for part in record.split(';') {
            let mut kv = part.splitn(2, '=');
            let key = kv.next().ok_or(SMTPError::DKIMError("Invalid DKIM record".to_owned()))?.trim();
            let value = kv.next().ok_or(SMTPError::DKIMError("Invalid DKIM record".to_owned()))?.trim();
            fields.insert(key, value);
        }

        let pub_key = fields.get("p").ok_or(SMTPError::DKIMError("Public key not found in DKIM record".to_owned()))?.to_string();
        let flags = fields.get("t").map(|t| t.to_string());
        let hash_algo = fields.get("h").map(|h| h.to_string());

        Ok(DkimPublicKey { pub_key, flags, hash_algo })
    }
}