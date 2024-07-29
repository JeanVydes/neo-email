use crate::{connection::SMTPConnection, errors::SMTPError};
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
pub struct SPFRecord {
    pub version: String,               // Always should be v=spf1
    pub ipv4: Vec<String>,             // List of allowed IPs
    pub all: SPFRecordAll,             // Policy to apply
    pub include: Vec<String>,          // List of to include SPF records
    pub included: Box<Vec<SPFRecord>>, // Included SPF records
    pub redirect: Option<String>,      // Redirect to another domain
}

/// # SPFRecord
///
/// SPFRecord implementation
impl SPFRecord {
    /// # new
    ///
    /// Creates a new SPFRecord
    pub fn new(
        version: String,
        ipv4: Vec<String>,
        all: SPFRecordAll,
        include: Vec<String>,
        included: Box<Vec<SPFRecord>>,
        redirect: Option<String>,
    ) -> Self {
        SPFRecord {
            version,
            ipv4,
            all,
            include,
            included,
            redirect,
        }
    }

    /// # from_string
    ///
    /// Parse a DNS SPF record to a SPFRecord struct
    pub fn from_string(spf_record: &str) -> Result<Self, SMTPError> {
        // Remove trailing spaces
        let spf_record = spf_record.trim();
        // Split the record by spaces
        let spf_record = spf_record.split_whitespace().collect::<Vec<&str>>();
        // Check if the record is valid (have enough information)
        if spf_record.len() < 2 {
            return Err(SMTPError::SPFError("Invalid SPF record".to_string()));
        }

        // Extract the version (should be v=spf1)
        let version = spf_record[0].to_string().split("=").collect::<Vec<&str>>()[1].to_string();
        if version != "spf1" {
            return Err(SMTPError::SPFError("Invalid SPF version".to_string()));
        }

        // Initialize the lists
        let mut ip4 = Vec::new();
        // Initialize the policy
        let mut all = SPFRecordAll::Passive;
        // Initialize the included records
        let mut include = Vec::new();
        // Initialize the redirect
        let mut redirect = None;

        // Iterate over the record
        for i in 1..spf_record.len() {
            // Get the record part
            let record = spf_record[i];
            // Convert the record to lowercase
            let record = record.to_lowercase();

            // Check the record
            // If the record starts with ip4: then add it to the ip4 list
            if record.starts_with("ip4:") {
                // Add the IP to the list of allowed IPs
                ip4.push(record.replace("ip4:", ""));
            // If the record starts with -all, ~all or +all then set the policy
            } else if record.starts_with("-all") {
                all = SPFRecordAll::Aggresive;
            } else if record.starts_with("~all") {
                all = SPFRecordAll::Passive;
            } else if record.starts_with("+all") {
                all = SPFRecordAll::Permissive;
            // If the record starts with include: then add it to the include list
            } else if record.starts_with("include:") {
                include.push(record.replace("include:", ""));
            // If the record starts with redirect= then set the redirect
            } else if record.starts_with("redirect=") {
                redirect = Some(record.replace("redirect=", ""));
            }
        }

        // Return the SPFRecord
        Ok(SPFRecord::new(
            version,
            ip4,
            all,
            include,
            Box::new(vec![]),
            redirect,
        ))
    }

    /// # get_dns_spf_record
    ///
    /// Get the SPF record from the DNS
    /// `remaining_redirects` is the number of redirects that the DNS resolver will follow
    /// `dns_resolver` is the DNS resolver
    /// `domain` is the domain to get the SPF record
    pub async fn get_dns_spf_record(
        remaining_redirects: u8,
        dns_resolver: Arc<Mutex<TokioAsyncResolver>>,
        domain: &str,
    ) -> Result<Self, SMTPError> {
        // Check if the number of remaining redirects is 0, and return an error
        if remaining_redirects == 0 {
            return Err(SMTPError::DNSError("Max redirects reached".to_string()));
        }

        // Lock the DNS resolver
        let dns_resolver_guarded = dns_resolver.lock().await;
        // Get the SPF record from the DNS
        let spf_record = dns_resolver_guarded
            .txt_lookup(format!("{}.", domain).as_str())
            .await
            .map_err(|_| SMTPError::DNSError("Failed to get SPF record".to_string()))?;

        // Find the SPF record for SPF policy
        let spf_record = spf_record
            .iter()
            .find(|record| record.to_string().starts_with("v=spf1"));

        // Check if the SPF record was found
        let spf_record = match spf_record {
            Some(record) => record.to_string(),
            None => return Err(SMTPError::SPFError("SPF record not found".to_string())),
        };

        // Parse the SPF record
        let parsed_spf_record = match Self::from_string(spf_record.as_str()) {
            Ok(record) => record,
            Err(e) => return Err(e),
        };

        // Some SMTP can delegate its SPF to another domain, for example gmail.com delegated to _spf.google.com
        if let Some(redirect) = parsed_spf_record.redirect {
            // Drop the DNS resolver for the next iteration
            drop(dns_resolver_guarded);
            // Box the future
            return Box::pin(Self::get_dns_spf_record(
                remaining_redirects - 1,
                dns_resolver.clone(),
                redirect.as_str(),
            ))
            .await;
        }

        // Return the SPF record
        Ok(parsed_spf_record)
    }
}

/// # sender_policy_framework
///
/// Check if the sender is allowed to send emails on behalf of the domain
/// `conn` is the SMTP connection
/// `domain` is the domain to check the SPF record
/// `policy` is the policy to apply
/// `max_depth_redirect` is the maximum depth of redirects that the SPF record can have
/// `max_include` is the maximum number of included SPF records
pub async fn sender_policy_framework<B>(
    conn: Arc<Mutex<SMTPConnection<B>>>,
    domain: &str,
    policy: SPFRecordAll,
    max_depth_redirect: u8,
    max_include: u8,
) -> Result<(bool, SPFRecord), SMTPError> {
    // Lock the connection
    let conn = conn.lock().await;
    // Get the IP address of the sender
    let ip = match conn.get_peer_addr().await {
        Ok(ip) => ip,
        Err(_) => return Err(SMTPError::SPFError("Failed to get IP address".to_string())),
    };

    // Get the SPF record from the DNS with a max depth of 3
    let mut record =
        match SPFRecord::get_dns_spf_record(max_depth_redirect, conn.dns_resolver.clone(), domain)
            .await
        {
            Ok(record) => record,
            Err(_) => return Err(SMTPError::SPFError("Failed to get SPF record".to_string())),
        };

    // Check if record require including other SPF records, and include it
    // For now this included_records cant include other, but allow redirect
    if record.include.len() > 0 {
        // Include only `max_include` records
        let mut i = max_include;
        // Include the SPF records
        for include in record.clone().include {
            // If the max_include is 0, then break the loop
            if i == 0 {
                break;
            }
            // For now this included_records cant include other, but allow redirect
            let included_record =
                match SPFRecord::get_dns_spf_record(3, conn.dns_resolver.clone(), include.as_str())
                    .await
                {
                    Ok(record) => record,
                    Err(_) => {
                        return Err(SMTPError::SPFError(
                            "Failed to get included SPF record".to_string(),
                        ))
                    }
                };
            // Add the included record to the SPF record
            record.included.push(included_record);
            // Decrement the counter
            i -= 1;
        }
    }

    // Extend the ipv4 list with the included records
    let mut total_ipv4 = record.ipv4.clone();
    for included_record in record.included.iter() {
        // Extend the ipv4 list with the included records
        total_ipv4.extend(included_record.ipv4.clone());
    }

    // Check if the IP is in the list of allowed IPs
    let listed_ipv4_record = total_ipv4.iter().find(|record| *record == &ip.to_string());

    // Check the policy based on the result
    match (policy, listed_ipv4_record) {
        // If the policy is Aggresive and the IP is on the list then return true
        (SPFRecordAll::Aggresive, Some(_)) => Ok((true, record)),
        // If the policy is Aggresive and the IP is not on the list then return an error
        (SPFRecordAll::Aggresive, None) => Err(SMTPError::SPFError("IP not allowed".to_string())),
        // If the policy is Passive and the IP is on the list then return true
        (SPFRecordAll::Passive, Some(_)) => Ok((true, record)),
        // If the policy is Passive and the IP is not on the list then return false
        (SPFRecordAll::Passive, None) => Ok((false, record)),
        // If the policy is Permissive then return true
        (SPFRecordAll::Permissive, _) => Ok((true, record)),
    }
}
