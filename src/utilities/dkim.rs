/// # DKIM
/// 
/// This module contains the DKIM implementation.
/// DKIM is used to verify the authenticity of the email.
/// It uses a public key to verify the signature of the email.
/// 
/// Note: This module is not implemented yet.

/*
use crate::{connection::SMTPConnection, errors::SMTPError};
use base64::prelude::*;
use openssl::{pkey::PKey, rsa::Rsa, sign::Verifier};
use sha1::Digest;
use std::sync::Arc;
use tokio::sync::Mutex;
use trust_dns_resolver::TokioAsyncResolver;

/// # SPFRecordAll
///
/// Represents the policy to apply in the SPF record
///
/// - Aggresive: -all means that all IPs that are not listed in the SPF record are not allowed to send emails
/// - Passive: ~all means that all IPs that are not listed in the SPF record are allowed to send emails but marked as spam
/// - Permissive: +all means that all IPs that are not listed in the SPF record are allowed to send emails
#[derive(Debug, Clone)]
pub enum SPFRecordAll {
    Aggresive, // -all means that all IPs that are not listed in the SPF record are not allowed to send emails
    Passive, // ~all means that all IPs that are not listed in the SPF record are allowed to send emails but marked as spam
    Permissive, // +all means that all IPs that are not listed in the SPF record are allowed to send emails
}

/// # SPFRecord
///
/// Represents an SPF record
/// Example `v=spf1 ip4:192.0.2.0 ip4:192.0.2.1 include:examplesender.email -all`
#[derive(Debug, Clone)]
pub struct DKIMRecord {
    pub version: String,    // Always should be v=dkim1
    pub public_key: String, // The public key
}

/// # DKIMRecord
///
/// DKIMRecord implementation
impl DKIMRecord {
    /// # new
    ///
    /// Creates a new DKIMRecord
    pub fn new(version: String, public_key: String) -> Self {
        DKIMRecord {
            version,
            public_key,
        }
    }

    /// # from_string
    ///
    /// Parse a DNS DKIM record to a DKIMRecord struct
    pub fn from_string(record: &str) -> Result<Self, SMTPError> {
        // Split the record by spaces
        let record = record.split(";").collect::<Vec<&str>>();
        // Remove trailing spaces
        let record = record.iter().map(|s| s.trim()).collect::<Vec<&str>>();
        // Check if the record has at least 2 elements
        if record.len() < 2 {
            return Err(SMTPError::DKIMError("Invalid DKIM record".to_string()));
        }

        // Check if the version is v=dkim1
        if record[0] != "v=dkim1" && record[0] != "v=DKIM1" {
            return Err(SMTPError::DKIMError("Invalid DKIM version".to_string()));
        }

        let mut version = String::new();
        let mut public_key = String::new();

        for i in 0..record.len() {
            let record = record[i];
            if record.starts_with("v=") {
                version = record[2..].to_string().to_lowercase();
            } else if record.starts_with("p=") {
                public_key = record[2..].to_string();
            }
        }

        // Return the DKIM record
        Ok(DKIMRecord::new(version, public_key))
    }

    /// # get_dns_dkim_record
    ///
    /// Get the DKIM record from the DNS
    /// `remaining_redirects` is the number of redirects that the DNS resolver will follow
    /// `dns_resolver` is the DNS resolver
    /// `domain` is the domain to get the SPF record
    pub async fn get_dns_dkim_record(
        dns_resolver: Arc<Mutex<TokioAsyncResolver>>,
        dkim_header: DKIMHeader,
    ) -> Result<Self, SMTPError> {
        // Lock the DNS resolver
        let dns_resolver_guarded = dns_resolver.lock().await;
        // Get the DKIM record from the DNS
        let txt_records = dns_resolver_guarded
            .txt_lookup(format!("{}.", dkim_header.domain).as_str())
            .await
            .map_err(|_| SMTPError::DNSError("Failed to get DKIM record".to_string()))?;

        // Find the DKIM record for DKIM policy
        let dkim_record = txt_records.iter().find(|record| {
            record.to_string().starts_with("v=dkim1") || record.to_string().starts_with("v=DKIM1")
        });

        // Check if the DKIM record was found
        /*let dkim_record = match dkim_record {
            Some(record) => record.to_string(),
            None => return Err(SMTPError::SPFError("DKIM record not found".to_string())),
        };*/

        // test dkim record
        let dkim_record = "v=DKIM1;t=s;p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDiZDfLB7SBvT+I7uAiikct0qiJGXaFq5rL3qn8cX383VpNq0V7pCKlW3rpdPcHzG9LvV68kIvpdxZZDR+9z41JIFg79hA2FrHpZhCpyRKrpdJKR8nI0VXBHPWKWcVibvH45faDwNtQNwA7BvIkeMd48TzbXg3aOe1m1wuQOQ2UawIDAQAB".to_string();

        // Parse the DKIM record
        let parsed_dkim_record = match Self::from_string(dkim_record.as_str()) {
            Ok(record) => record,
            Err(e) => return Err(e),
        };

        // Return the DKIM record
        Ok(parsed_dkim_record)
    }
}

/// # dkim
///
/// Check if the email is valid with the DKIM record
pub async fn dkim<B>(
    conn: Arc<Mutex<SMTPConnection<B>>>,
    dkim_header: String,
    body: Vec<u8>,
) -> Result<DKIMRecord, SMTPError> {
    let conn = conn.lock().await;
    let dkim_header = DKIMHeader::from_string(dkim_header.as_str())?;
    // Get the DKIM record from the DNS
    let record =
        DKIMRecord::get_dns_dkim_record(conn.dns_resolver.clone(), dkim_header.clone()).await?;
    let pem_key = format_public_key(record.public_key.as_str());
    let rsa = Rsa::public_key_from_pem(pem_key.as_bytes())
        .map_err(|err| SMTPError::DKIMError(err.to_string()))?;
    let pkey = PKey::from_rsa(rsa).map_err(|err| SMTPError::DKIMError(err.to_string()))?;

    let alg = match dkim_header.algorithm.as_str() {
        "rsa-sha1" => openssl::hash::MessageDigest::sha1(),
        "rsa-sha256" => openssl::hash::MessageDigest::sha256(),
        _ => return Err(SMTPError::DKIMError("Invalid DKIM algorithm".to_string())),
    };

    let mut verifier =
        Verifier::new(alg, &pkey).map_err(|e| SMTPError::DKIMError(e.to_string()))?;
    verifier
        .set_rsa_padding(openssl::rsa::Padding::PKCS1)
        .map_err(|e| SMTPError::DKIMError(e.to_string()))?;

    let clean_signature = dkim_header
        .signature
        .replace('\r', "")
        .replace('\n', "")
        .replace(' ', "");

    // Decode the Base64 encoded signature
    let mut signature_bytes = match BASE64_STANDARD.decode(clean_signature.as_bytes()) {
        Ok(signature_bytes) => signature_bytes,
        Err(e) => return Err(SMTPError::DKIMError(e.to_string())),
    };

    // Verify the signature
    verifier
        .verify(&signature_bytes)
        .map_err(|e| SMTPError::DKIMError(e.to_string()))?;

    Ok(record)
}

fn format_public_key(base64_key: &str) -> String {
    let key = base64_key.replace("\n", "").replace("\r", "");
    format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
        key.chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    )
}

#[derive(Debug, Clone)]
pub struct DKIMHeader {
    pub version: String,
    pub algorithm: String,
    pub domain: String,
    pub selector: String,
    pub headers: Vec<String>,
    pub body_hash: String,
    pub signature: String,
}

impl DKIMHeader {
    pub fn from_string(header: &str) -> Result<Self, SMTPError> {
        // Split the record by spaces
        let header = header.split(";").collect::<Vec<&str>>();
        // Remove trailing spaces
        let header = header.iter().map(|s| s.trim()).collect::<Vec<&str>>();
        let mut version = String::new();
        let mut algorithm = String::new();
        let mut domain = String::new();
        let mut selector = String::new();
        let mut headers = Vec::new();
        let mut body_hash = String::new();
        let mut signature = String::new();

        for i in 0..header.len() {
            let record = header[i];
            if record.starts_with("v=") {
                version = record[2..].to_string();
            } else if record.starts_with("a=") {
                algorithm = record[2..].to_string();
            } else if record.starts_with("d=") {
                domain = record[2..].to_string();
            } else if record.starts_with("s=") {
                selector = record[2..].to_string();
            } else if record.starts_with("h=") {
                headers = record[2..].split(':').map(|s| s.to_string()).collect();
            } else if record.starts_with("bh=") {
                body_hash = record[3..].to_string();
            } else if record.starts_with("b=") {
                signature = record[2..].to_string();
            }
        }

        Ok(DKIMHeader {
            version,
            algorithm,
            domain,
            selector,
            headers,
            body_hash,
            signature,
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "v={}; a={}; d={}; s={}; h={}; bh={}; b={}",
            self.version,
            self.algorithm,
            self.domain,
            self.selector,
            self.headers.join(":"),
            self.body_hash,
            self.signature
        )
    }
}
*/