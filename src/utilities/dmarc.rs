use crate::{connection::SMTPConnection, errors::Error, mail::EmailAddress};
use std::sync::Arc;
use tokio::sync::Mutex;
use trust_dns_resolver::TokioAsyncResolver;

/// # DMARC Policy
///
/// Represents the policy to apply in the DMARC record
#[derive(Debug, Clone)]
pub enum DMARCPolicy {
    /// # None
    /// 
    /// No policy
    None,       // No policy
    /// # Quarantine
    /// 
    /// Quarantine policy, the email will be sent to the spam folder
    Quarantine, // Quarantine policy
    /// # Reject
    /// 
    /// Reject policy, the email will be rejected
    Reject,     // Reject policy
}

/// # DMARCDKIMAlignment
/// 
/// Represents the DKIM alignment
#[derive(Debug, Clone)]
pub enum DMARCDKIMAlignment {
    /// # Relaxed
    /// 
    /// Relaxed alignment
    Relaxed,
    /// # Strict
    /// 
    /// Strict alignment
    Strict,
}

/// # DMARCSPFAlignment
/// 
/// Represents the SPF alignment
#[derive(Debug, Clone)]
pub enum DMARCSPFAlignment {
    /// # Relaxed
    /// 
    /// Relaxed alignment
    Relaxed,
    /// # Strict
    /// 
    /// Strict alignment
    Strict,
}

/// # DMARCForensicReport
/// 
/// Represents the forensic report to send
#[derive(Debug, Clone)]
pub enum DMARCForensicReport {
    /// # None
    /// 
    /// No forensic report
    None,
    /// # DKIM
    /// 
    /// forensic report for invalid DKIM
    DKIM,
    /// # SPF
    /// 
    /// forensic report for invalid SPF
    SPF,
    /// # Both
    /// 
    /// forensic report for both DKIM and SPF
    Both,
}

/// # DMARCRecord
///
/// Represents a DMARC record
/// Example `v=dmarc1; p=none; rua=mailto:
#[derive(Debug, Clone)]
pub struct DMARCRecord {
    /// # version
    /// 
    /// Always should be v=dmarc1
    pub version: String,
    /// # policy
    /// 
    /// The policy to apply
    pub policy: DMARCPolicy,

    /// # aggregate_report_email
    /// 
    /// The email to send the aggregate reports
    pub aggregate_report_email: Option<EmailAddress>, // The email to send the aggregate reports
    /// # forensic_report_email
    /// 
    /// The email to send the forensic reports
    pub forensic_report_email: Option<EmailAddress>,  // The email to send the forensic reports

    /// # dkim_alignment
    /// 
    /// The DKIM alignment
    pub dkim_alignment: Option<DMARCDKIMAlignment>, // The DKIM alignment
    /// # spf_alignment
    /// 
    /// The SPF alignment
    pub spf_alignment: Option<DMARCSPFAlignment>,   // The SPF alignment

    /// # report_format
    /// 
    /// The report format
    pub report_format: Option<String>, // The report format
    /// # percentage
    /// 
    /// The percentage of emails to apply the policy
    pub percentage: Option<u8>,        // The percentage of emails to apply the policy
    /// # report_interval
    /// 
    /// The report interval
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
        aggregate_report_email: Option<EmailAddress>,
        forensic_report_email: Option<EmailAddress>,
        dkim_alignment: Option<DMARCDKIMAlignment>,
        spf_alignment: Option<DMARCSPFAlignment>,
        report_format: Option<String>,
        percentage: Option<u8>,
        report_interval: Option<u32>,
    ) -> Self {
        DMARCRecord {
            version,
            policy,
            aggregate_report_email,
            forensic_report_email,
            dkim_alignment,
            spf_alignment,
            report_format,
            percentage,
            report_interval,
        }
    }

    /// # from_string
    ///
    /// Parse a DNS DMARC record to a DMARCRecord struct
    pub fn from_string(record: &str) -> Result<Self, Error> {
        // Split the record by spaces
        let record = record.split(";").collect::<Vec<&str>>();
        // Remove trailing spaces
        let record = record.iter().map(|s| s.trim()).collect::<Vec<&str>>();
        // Check if the record has at least 2 elements
        if record.len() < 2 {
            return Err(Error::DKIMError("Invalid DMARC record".to_string()));
        }

        // Check if the version is v=dkim1
        if record[0] != "v=dmarc1" && record[0] != "v=DMARC1" {
            return Err(Error::DKIMError("Invalid DKIM version".to_string()));
        }

        let mut version = String::new();
        let mut policy = DMARCPolicy::None;
        let mut aggregate_report_email = None;
        let mut forensic_report_email = None;
        let mut dkim_alignment = None;
        let mut spf_alignment = None;
        let mut report_format = None;
        let mut percentage = None;
        let mut report_interval = None;

        for i in 0..record.len() {
            let record = record[i];
            if record.starts_with("v=") {
                version = record.replace("v=", "");
            } else if record.starts_with("p=") {
                policy = match record.replace("p=", "").to_lowercase().as_str() {
                    "none" => DMARCPolicy::None,
                    "quarantine" => DMARCPolicy::Quarantine,
                    "reject" => DMARCPolicy::Reject,
                    _ => return Err(Error::DKIMError("Invalid DMARC policy".to_string())),
                };
            } else if record.starts_with("rua=") {
                // Get the mailto:email part
                let mailto = record.replace("rua=", "");
                // Check if the email starts with mailto:
                if !mailto.starts_with("mailto:") {
                    return Err(Error::DKIMError(
                        "Invalid DMARC aggregate report email".to_string(),
                    ));
                }

                // Get the email
                let email = mailto.split(":").collect::<Vec<&str>>()[1];
                // Check if the email is valid
                let email = EmailAddress::from_string(email).map_err(|_| {
                    Error::DKIMError("Invalid DMARC aggregate report email".to_string())
                })?;

                // Set the email
                aggregate_report_email = Some(email);
            } else if record.starts_with("ruf=") {
                // Get the mailto:email part
                let mailto = record.replace("ruf=", "");
                // Check if the email starts with mailto:
                if !mailto.starts_with("mailto:") {
                    return Err(Error::DKIMError(
                        "Invalid DMARC forensic report email".to_string(),
                    ));
                }
                // Get the email
                let email = mailto.split(":").collect::<Vec<&str>>()[1];
                // Check if the email is valid
                let email = EmailAddress::from_string(email).map_err(|_| {
                    Error::DKIMError("Invalid DMARC aggregate report email".to_string())
                })?;
                // Set the email
                forensic_report_email = Some(email);
            } else if record.starts_with("adkim=") {
                // Get the DKIM alignment
                dkim_alignment = match record.replace("adkim=", "").to_lowercase().as_str() {
                    "r" => Some(DMARCDKIMAlignment::Relaxed),
                    "s" => Some(DMARCDKIMAlignment::Strict),
                    _ => {
                        return Err(Error::DKIMError(
                            "Invalid DMARC DKIM alignment".to_string(),
                        ))
                    }
                };
            } else if record.starts_with("aspf=") {
                // Get the SPF alignment
                spf_alignment = match record.replace("aspf=", "").to_lowercase().as_str() {
                    "r" => Some(DMARCSPFAlignment::Relaxed),
                    "s" => Some(DMARCSPFAlignment::Strict),
                    _ => {
                        return Err(Error::DKIMError(
                            "Invalid DMARC SPF alignment".to_string(),
                        ))
                    }
                };
            } else if record.starts_with("rf=") {
                report_format = Some(record.replace("rf=", "").to_string());
            } else if record.starts_with("pct=") {
                percentage = Some(
                    record
                        .replace("pct=", "")
                        .parse::<u8>()
                        .map_err(|_| Error::DKIMError("Invalid DMARC percentage".to_string()))?,
                );
            } else if record.starts_with("ri=") {
                report_interval = Some(
                    record
                        .replace("ri=", "")
                        .parse::<u32>()
                        .map_err(|_| Error::DKIMError("Invalid DMARC report interval".to_string()))?,
                );
            }
        }

        // Return the DKIM record
        Ok(DMARCRecord::new(
            version,
            policy,
            aggregate_report_email,
            forensic_report_email,
            dkim_alignment,
            spf_alignment,
            report_format,
            percentage,
            report_interval,
        ))
    }

    /// # get_dns_dmarc_record
    ///
    /// Get the DMARC record from the DNS
    pub async fn get_dns_dmarc_record(
        dns_resolver: Arc<Mutex<TokioAsyncResolver>>,
        for_domain: &str,
    ) -> Result<Self, Error> {
        // Lock the DNS resolver
        let dns_resolver_guarded = dns_resolver.lock().await;
        // Get the DMARC record from the DNS
        let txt_records = dns_resolver_guarded
            .txt_lookup(format!("{}.", for_domain).as_str())
            .await
            .map_err(|_| Error::DNSError("Failed to get DMARC record".to_string()))?;

        // Find the DMARC record for DMARC policy
        let dmarc_record = txt_records.iter().find(|record| {
            record.to_string().starts_with("v=dmarc1") || record.to_string().starts_with("v=DMARC1")
        });

        if dmarc_record.is_none() {
            return Err(Error::DKIMError("DMARC record not found".to_string()));
        }

        let dmarc_record = dmarc_record.unwrap().to_string();

        // Parse the DMARC record
        let parsed_dmarc_record = match Self::from_string(dmarc_record.as_str()) {
            Ok(record) => record,
            Err(e) => return Err(e),
        };

        // Return the DMARC
        Ok(parsed_dmarc_record)
    }
}

/// # dmarc
///
/// Get DMARC Information
/// 
/// ## Arguments
/// 
/// * `conn` - The SMTP Connection
/// * `for_domain` - The domain to get the DMARC record
pub async fn get_dmarc<B>(
    conn: Arc<Mutex<SMTPConnection<B>>>,
    for_domain: &str,
) -> Result<DMARCRecord, Error> {
    let conn = conn.lock().await;
    let record = DMARCRecord::get_dns_dmarc_record(conn.dns_resolver.clone(), for_domain).await?;
    Ok(record)
}