/// # DKIM
/// 
/// This module contains the DomainKeys Identified Mail. (Not implemented yet)
//#[cfg(feature = "dkim-experimental")]
//pub mod dkim;

/// # SPF
/// 
/// This module contains the Sender Policy Framework.
#[cfg(feature = "spf-experimental")]
pub mod spf;

/// # DMARC
/// 
/// This module contains the Domain-based Message Authentication, Reporting and Conformance.
#[cfg(feature = "dmarc-experimental")]
pub mod dmarc;