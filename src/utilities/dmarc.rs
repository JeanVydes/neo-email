use crate::{connection::SMTPConnection, errors::SMTPError};
use base64::prelude::*;
use openssl::{pkey::PKey, rsa::Rsa, sign::Verifier};
use sha1::Digest;
use std::sync::Arc;
use tokio::sync::Mutex;
use trust_dns_resolver::TokioAsyncResolver;

/// # DMARC Policy
///
/// Represents the policy to apply in the DMARC record
#[derive(Debug, Clone)]
pub enum DMARCPolicy {
    None,       // No policy
    Quarantine, // Quarantine policy
    Reject,     // Reject policy
}

pub enum DMARCDKIMAlignment {
    Relaxed,
    Strict,
}

pub enum DMARKCPFAlignment {
    Relaxed,
    Strict,
}

pub enum DMARCForensicReport {
    None,
    DKIM,
    SPF,
    Both,
}

/// # DMARCRecord
///
/// Represents a DMARC record
/// Example `v=dmarc1; p=none; rua=mailto:
#[derive(Debug, Clone)]
pub struct DMARCRecord {
    pub version: String,                // Always should be v=dmarc1
    pub policy: DMARCPolicy,            // The policy to apply

    pub aggregate_report_email: Option<String>, // The email to send the aggregate reports
    pub forensic_report_email: Option<String>,  // The email to send the forensic reports

    pub dkim_alignment: Option<DMARCDKIMAlignment>, // The DKIM alignment
    pub spf_alignment: Option<DMARCSPFAlignment>,   // The SPF alignment

    pub report_format: Option<String>, // The report format
    pub percentage: Option<u8>,         // The percentage of emails to apply the policy

    pub report_interval: Option<u32>, // The report interval
}

/// # DKIMRecord
///
/// DKIMRecord implementation
impl DMARCRecord {
    /// # new
    ///
    /// Creates a new DMARCRecord
    pub fn new(
        version: String,
        policy: DMARCPolicy,
        aggregate_report_email: String,
        forensic_report_email: String,
    ) -> Self {
        DMARCRecord {
            version,
            policy,
            aggregate_report_email,
            forensic_report_email,
        }
    }

    /// # from_string
    ///
    /// Parse a DNS DMARC record to a DMARCRecord struct
    pub fn from_string(record: &str) -> Result<Self, SMTPError> {
        // Split the record by spaces
        let record = record.split(";").collect::<Vec<&str>>();
        // Remove trailing spaces
        let record = record.iter().map(|s| s.trim()).collect::<Vec<&str>>();
        // Check if the record has at least 2 elements
        if record.len() < 2 {
            return Err(SMTPError::DKIMError("Invalid DMARC record".to_string()));
        }

        // Check if the version is v=dkim1
        if record[0] != "v=dmarc1" && record[0] != "v=DMARC1" {
            return Err(SMTPError::DKIMError("Invalid DKIM version".to_string()));
        }

        let mut version = String::new();
        let mut policy = DMARCPolicy::None;
        let mut aggregate_report_email = None;
        let mut forensic_report_email = None;
        let mut dkim_alignment = None;
        let mut spf_alignment = None;
        let mut report_format = None;
        let mut percentage = None;

        for i in 0..record.len() {
            let record = record[i];
            if record.starts_with("v=") {
                version = record[2..].to_string().to_lowercase();
            } else if record.starts_with("p=") {
                policy = match record[2..].to_lowercase().as_str() {
                    "none" => DMARCPolicy::None,
                    "quarantine" => DMARCPolicy::Quarantine,
                    "reject" => DMARCPolicy::Reject,
                    _ => return Err(SMTPError::DKIMError("Invalid DMARC policy".to_string())),
                };
            } else if record.starts_with("rua=") {
                let mailto = record[4..260].to_string();
                let email = mailto.split(":").collect::<Vec<&str>>()[1];
                aggregate_report_email = Some(email.to_string());
            } else if record.starts_with("ruf=") {
                let mailto = record[4..260].to_string();
                let email = mailto.split(":").collect::<Vec<&str>>()[1];
                forensic_report_email = Some(email.to_string());
            } else if record.starts_with("adkim=") {
                dkim_alignment = match record[6..7].to_lowercase().as_str() {
                    "r" => Some(DMARCDKIMAlignment::Relaxed),
                    "s" => Some(DMARCDKIMAlignment::Strict),
                    _ => return Err(SMTPError::DKIMError("Invalid DMARC DKIM alignment".to_string())),
                };
            } else if record.starts_with("aspf=") {
                spf_alignment = match record[5..6].to_lowercase().as_str() {
                    "r" => Some(DMARCSPFAlignment::Relaxed),
                    "s" => Some(DMARCSPFAlignment::Strict),
                    _ => return Err(SMTPError::DKIMError("Invalid DMARC SPF alignment".to_string())),
                };
            } else if record.starts_with("rf=") {
                report_format = Some(record[3..128].to_string());
            } else if record.starts_with("pct=") {
                percentage = Some(record[4..].parse().map_err(|_| SMTPError::DMARCError("Invalid DMARC percentage".to_string()))?);
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
