use crate::{connection::SMTPConnection, errors::SMTPError};
use std::{net::IpAddr, sync::Arc};
use tokio::sync::Mutex;
use trust_dns_resolver::{proto::rr::RecordType, TokioAsyncResolver};

/// # SPFRecordAll
///
/// Represents the policy to apply in the SPF record
///
/// - Aggresive: -all means that all IPs that are not listed in the SPF record are not allowed to send emails
/// - Passive: ~all means that all IPs that are not listed in the SPF record are allowed to send emails but marked as spam
/// - Permissive: +all means that all IPs that are not listed in the SPF record are allowed to send emails
#[derive(Debug, Clone)]
pub enum SPFRecordAll {
    /// -all means that all IPs that are not listed in the SPF record are not allowed to send emails
    Aggresive,
    /// ~all means that all IPs that are not listed in the SPF record are allowed to send emails but marked as spam
    Passive,
    /// +all means that all IPs that are not listed in the SPF record are allowed to send emails
    Permissive,
}

/// # SPFRecord
///
/// Represents an SPF record
/// Example of a raw TXT SPF Record `v=spf1 ip4:192.0.2.0 ip4:192.0.2.1 include:examplesender.email -all`
#[derive(Debug, Clone)]
pub struct SPFRecord {
    /// # Version
    ///
    /// Always should be v=spf1
    pub version: String, // Always should be v=spf1
    /// # IPv4
    ///
    /// List of allowed IPs
    pub ipv4: Vec<String>, // List of allowed IPs
    /// # IPv6
    ///
    /// List of allowed IPs
    ///
    /// List of allowed IPs
    pub ipv6: Vec<String>, // List of allowed IPs
    /// # All
    ///
    /// Policy to apply
    pub all: SPFRecordAll, // Policy to apply
    /// # Root Include
    ///
    /// List of to include SPF records (only contain the IP-Domains where the SPF record is located)
    pub root_include: Vec<String>, // List of to include SPF records
    /// # Included
    ///
    /// Included SPF records from other domains
    pub included: Box<Vec<SPFRecord>>, // Included SPF records
    /// # Redirect
    ///
    /// Set the SPF Policy on behalf of another domain
    pub redirect: Option<String>, // Redirect to another domain
    /// # Exists
    /// 
    /// Check if the SPF record exists
    pub exists: Option<String>,
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
        ipv6: Vec<String>,
        all: SPFRecordAll,
        root_include: Vec<String>,
        included: Box<Vec<SPFRecord>>,
        redirect: Option<String>,
        exists: Option<String>,
    ) -> Self {
        SPFRecord {
            version,
            ipv4,
            ipv6,
            all,
            root_include,
            included,
            redirect,
            exists,
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
        let mut ip6 = Vec::new();
        // Initialize the policy
        let mut all = SPFRecordAll::Passive;
        // Initialize the included records
        let mut include = Vec::new();
        // Initialize the redirect
        let mut redirect = None;

        let mut exists = None;

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
            } else if record.starts_with("ip6:") {
                // Add the IP to the list of allowed IPs
                ip6.push(record.replace("ip6:", ""));
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
            } else if record.starts_with("exists:") {
                exists = Some(record.replace("exists:", ""));
            }
        }

        // Return the SPFRecord
        Ok(SPFRecord::new(
            version,
            ip4,
            ip6,
            all,
            include,
            Box::new(vec![]),
            redirect,
            exists,
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
/// 
/// `conn` is the SMTP connection
/// `domain` is the domain to check the SPF record
/// `policy` is the policy to apply
/// `max_depth_redirect` is the maximum depth of redirects that the SPF record can have
/// `max_include` is the maximum number of included SPF records
///
/// Returns a tuple with the result of the SPF check, the SPF record and the matched allowed IP pattern
pub async fn sender_policy_framework<B>(
    conn: Arc<Mutex<SMTPConnection<B>>>,
    domain: &str,
    policy: SPFRecordAll,
    max_depth_redirect: u8,
    max_include: u8,
) -> Result<(bool, SPFRecord, Option<String>), SMTPError> {
    // Lock the connection
    let conn = conn.lock().await;
    // Get the IP address of the sender
    let origin_ip = match conn.get_peer_addr().await {
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

    // If exists mechanism is present, check if the record exists
    match &record.exists {
        Some(domain_to_query) => {
            // Append the dot to the domain for a better query
            let domain_to_query = format!("{}.", domain_to_query);
            // Lock the DNS resolver
            let dns_resolver_guarded = conn.dns_resolver.lock().await;
            // Check if the domain has a valid record
            let mut record_exists = false;

            // Check if the domain has an A or AAAA record
            // If the domain has an A or AAAA record, then the domain exists
            if origin_ip.is_ipv4() {
                // Get the A record
                let lookup = dns_resolver_guarded
                    .lookup(domain_to_query.as_str(), RecordType::A)
                    .await
                    .map_err(|_| SMTPError::DNSError("Failed to get A record".to_string()))?;
                // Check if the domain has an A record
                let a_record_exists = lookup.records().iter().find(|record| {
                    record.record_type() == RecordType::A
                });
                // If the domain has an A record, then the domain exists
                if a_record_exists.is_some() {
                    record_exists = true;
                }
            } else {
                // Get the AAAA record
                let lookup = dns_resolver_guarded
                    .lookup(domain_to_query.as_str(), RecordType::AAAA)
                    .await
                    .map_err(|_| SMTPError::DNSError("Failed to get AAAA record".to_string()))?;
                // Check if the domain has an AAAA record
                let aaaa_record_exists = lookup.records().iter().find(|record| {
                    record.record_type() == RecordType::AAAA
                });
                // If the domain has an AAAA record, then the domain exists
                if aaaa_record_exists.is_some() {
                    record_exists = true;
                }
            }
            // If the domain does not exist, then return an error
            if !record_exists {
                return Err(SMTPError::SPFError("IP not allowed".to_string()));
            }
        }
        None => {}
    }

    // Check if record require including other SPF records, and include it
    // For now this included_records cant include other, but allow redirects
    if record.root_include.len() > 0 {
        // Include only `max_include` records
        let mut i = max_include;
        // Include the SPF records
        for include in &record.root_include {
            // If the max_include is 0, then break the loop
            if i == 0 {
                break;
            }
            // For now this included_records cant include other, but allow redirect
            let included_record = match SPFRecord::get_dns_spf_record(
                max_depth_redirect,
                conn.dns_resolver.clone(),
                include.as_str(),
            )
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
    let mut total_ipv6 = record.ipv6.clone();
    for included_record in record.included.iter() {
        // Extend the ipv4 list with the included records
        total_ipv4.extend(included_record.ipv4.clone());
        // Extend the ipv6 list with the included records
        total_ipv6.extend(included_record.ipv6.clone());
    }

    // Check if the IP is in the list of allowed IPs
    let mut matched_allowed_ip_pattern: Option<String> = None;

    if origin_ip.is_ipv4() {
        for ipv4 in total_ipv4.iter() {
            // Split the IP/CIDR
            let parts = ipv4.split("/").collect::<Vec<&str>>();

            // Check if the IP is valid
            let (allowed_ip, cdir) = if parts.len() == 2 {
                (parts[0], parts[1])
            } else if parts.len() == 1 {
                (parts[0], "32") // Default prefix length for single IP addresses
            } else {
                // Invalid format, skip this record
                continue;
            };

            // Convert the IP to a number
            let ip_num = allowed_ip
                .split('.')
                .map(|s| s.parse::<u32>().unwrap())
                .fold(0, |acc, part| (acc << 8) + part);

            // Create the mask
            let cdir_num = match cdir.parse::<u32>() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // Create the mask
            let mask = (0xffffffff as u32) << (32 - cdir_num);

            // Apply the mask
            let ip_num = ip_num & mask;
            // Get the IP from the peer IP
            let origin_ip = origin_ip.ip();

            // Example
            // allowed ip: 130.211.0.0/22 from an allowed Gmail google server
            // Range 130.211.0.0 -> 130.211.2.255
            // origin ip: 130.211.0.155 that is in range of allowed IPs
            // so supossing that email is sent from
            // let origin_ip = IpAddr::V4(std::net::Ipv4Addr::new(130, 211, 0, 155));`

            // Extract the IP number from the peer IP
            if let IpAddr::V4(ipv4_addr) = origin_ip {
                // Convert the IP to a number
                let peer_ip_num = u32::from(ipv4_addr);

                // Check if the IP is in the range
                if ip_num == (peer_ip_num & mask) {
                    matched_allowed_ip_pattern = Some(ipv4.to_string());
                    break;
                }
            }
        }
    } else {
        for ipv6 in total_ipv6.iter() {
            // Split the IP/CIDR
            let parts = ipv6.split("/").collect::<Vec<&str>>();

            // Check if the IP is valid
            let (allowed_ip, cdir) = if parts.len() == 2 {
                (parts[0], parts[1])
            } else if parts.len() == 1 {
                (parts[0], "128") // Default prefix length for single IP addresses
            } else {
                // Invalid format, skip this record
                continue;
            };

            // Parse the CIDR value
            let cidr_num: u8 = match cdir.parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // Parse the allowed IP into segments
            let allowed_ip_segments: Vec<u16> = allowed_ip
                .split(':')
                .map(|s| u16::from_str_radix(s, 16).unwrap_or(0))
                .collect();

            // Compute the mask for the given CIDR
            let mask: Vec<u16> = (0..8)
                .map(|i| {
                    if i < (cidr_num / 16) {
                        0xffff
                    } else if i == (cidr_num / 16) {
                        0xffff << (16 - (cidr_num % 16))
                    } else {
                        0
                    }
                })
                .collect();

            // Apply the mask to the allowed IP segments
            let masked_allowed_ip: Vec<u16> = allowed_ip_segments
                .iter()
                .zip(&mask)
                .map(|(segment, m)| segment & m)
                .collect();

            // Apply the mask to the sender's IP segments
            if let IpAddr::V6(ipv6_addr) = origin_ip.ip() {
                let peer_ip_segments: Vec<u16> = ipv6_addr.segments().to_vec();
                let masked_peer_ip: Vec<u16> = peer_ip_segments
                    .iter()
                    .zip(&mask)
                    .map(|(segment, m)| segment & m)
                    .collect();

                // Check if the masked allowed IP and the masked peer IP match
                if masked_allowed_ip == masked_peer_ip {
                    matched_allowed_ip_pattern = Some(ipv6.to_string());
                    break;
                }
            }
        }
    }

    // Check the policy based on the result
    match (policy, matched_allowed_ip_pattern.as_ref()) {
        // If the policy is Aggresive and the IP is on the list then return true
        (SPFRecordAll::Aggresive, Some(_)) => Ok((true, record, matched_allowed_ip_pattern)),
        // If the policy is Aggresive and the IP is not on the list then return an error
        (SPFRecordAll::Aggresive, None) => Err(SMTPError::SPFError("IP not allowed".to_string())),
        // If the policy is Passive and the IP is on the list then return true
        (SPFRecordAll::Passive, Some(_)) => Ok((true, record, matched_allowed_ip_pattern)),
        // If the policy is Passive and the IP is not on the list then return false
        (SPFRecordAll::Passive, None) => Ok((false, record, matched_allowed_ip_pattern)),
        // If the policy is Permissive then return true
        (SPFRecordAll::Permissive, _) => Ok((true, record, matched_allowed_ip_pattern)),
    }
}
