use serde::{Deserialize, Serialize};

/// Security products that can be enabled or disabled for an organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityProduct {
    DependencyGraph,
    DependabotAlerts,
    DependabotSecurityUpdates,
    AdvancedSecurity,
    SecretScanning,
    SecretScanningPushProtection,
    SecretScanningValidityChecks,
    SecretScanningNonProviderPatterns,
}

impl std::fmt::Display for SecurityProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DependencyGraph => write!(f, "dependency_graph"),
            Self::DependabotAlerts => write!(f, "dependabot_alerts"),
            Self::DependabotSecurityUpdates => write!(f, "dependabot_security_updates"),
            Self::AdvancedSecurity => write!(f, "advanced_security"),
            Self::SecretScanning => write!(f, "secret_scanning"),
            Self::SecretScanningPushProtection => write!(f, "secret_scanning_push_protection"),
            Self::SecretScanningValidityChecks => write!(f, "secret_scanning_validity_checks"),
            Self::SecretScanningNonProviderPatterns => {
                write!(f, "secret_scanning_non_provider_patterns")
            }
        }
    }
}

/// Enablement state for a security product.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEnablement {
    EnableAll,
    DisableAll,
}

impl std::fmt::Display for SecurityEnablement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnableAll => write!(f, "enable_all"),
            Self::DisableAll => write!(f, "disable_all"),
        }
    }
}
