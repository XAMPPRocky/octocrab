use serde::{Deserialize, Serialize};

/// Status of automated security fixes (Dependabot security updates) for a repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutomatedSecurityFixes {
    pub enabled: bool,
    pub paused: bool,
}

impl AutomatedSecurityFixes {
    pub fn new(enabled: bool, paused: bool) -> Self {
        Self { enabled, paused }
    }
}

/// Status of private vulnerability reporting for a repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PrivateVulnerabilityReporting {
    pub enabled: bool,
}

impl PrivateVulnerabilityReporting {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
