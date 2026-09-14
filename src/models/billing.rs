use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// =========================================================================
// Actions, Packages & Shared Storage Billing Models
// =========================================================================

/// GitHub Actions billing usage for an organization or user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActionsBillingUsage {
    /// Sum of free and paid GitHub Actions minutes used.
    pub total_minutes_used: u64,
    /// Total paid GitHub Actions minutes used.
    pub total_paid_minutes_used: u64,
    /// Free GitHub Actions minutes available.
    pub included_minutes: u64,
    /// Breakdown of minutes used on different runner machines.
    #[serde(default)]
    pub minutes_used_breakdown: MinutesUsedBreakdown,
}

/// Breakdown of minutes used across runner types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct MinutesUsedBreakdown {
    #[serde(rename = "UBUNTU", skip_serializing_if = "Option::is_none")]
    pub ubuntu: Option<u64>,
    #[serde(rename = "MACOS", skip_serializing_if = "Option::is_none")]
    pub macos: Option<u64>,
    #[serde(rename = "WINDOWS", skip_serializing_if = "Option::is_none")]
    pub windows: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubuntu_4_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubuntu_8_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubuntu_16_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubuntu_32_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubuntu_64_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_4_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_8_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_16_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_32_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_64_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macos_12_core: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, u64>,
}

/// GitHub Packages billing usage for an organization or user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PackagesBillingUsage {
    /// Sum of free and paid storage space (GB) for GitHub Packages.
    pub total_gigabytes_bandwidth_used: u64,
    /// Total paid storage space (GB) for GitHub Packages.
    pub total_paid_gigabytes_bandwidth_used: u64,
    /// Free storage space (GB) for GitHub Packages.
    pub included_gigabytes_bandwidth: u64,
}

/// Shared storage billing usage for an organization or user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CombinedBillingUsage {
    /// Days left in billing cycle.
    pub days_left_in_billing_cycle: u64,
    /// Estimated storage space (GB) used in billing cycle.
    pub estimated_paid_storage_for_month: u64,
    /// Estimated sum of free and paid storage space (GB) used in billing cycle.
    pub estimated_storage_for_month: u64,
}

/// Type alias for shared storage billing usage.
pub type SharedStorageBillingUsage = CombinedBillingUsage;

// =========================================================================
// Usage Reports & Summaries Models
// =========================================================================

/// Billing usage report containing line items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BillingUsageReport {
    #[serde(rename = "usageItems")]
    pub usage_items: Vec<BillingUsageReportItem>,
}

/// Individual item in a billing usage report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BillingUsageReportItem {
    pub date: String,
    pub product: String,
    pub sku: String,
    pub quantity: u64,
    #[serde(rename = "unitType")]
    pub unit_type: String,
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: f64,
    #[serde(rename = "grossAmount")]
    pub gross_amount: f64,
    #[serde(rename = "discountAmount")]
    pub discount_amount: f64,
    #[serde(rename = "netAmount")]
    pub net_amount: f64,
    #[serde(rename = "organizationName", skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(rename = "repositoryName", skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// Time period specification in billing summary reports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingTimePeriod {
    pub year: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<u8>,
}

/// Billing usage summary report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BillingUsageSummaryReport {
    #[serde(rename = "timePeriod")]
    pub time_period: BillingTimePeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "usageItems")]
    pub usage_items: Vec<BillingSummaryUsageItem>,
}

/// Item in a billing usage summary report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BillingSummaryUsageItem {
    pub product: String,
    pub sku: String,
    #[serde(rename = "unitType")]
    pub unit_type: String,
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: f64,
    #[serde(rename = "grossQuantity")]
    pub gross_quantity: f64,
    #[serde(rename = "grossAmount")]
    pub gross_amount: f64,
    #[serde(rename = "discountQuantity")]
    pub discount_quantity: f64,
    #[serde(rename = "discountAmount")]
    pub discount_amount: f64,
    #[serde(rename = "netQuantity")]
    pub net_quantity: f64,
    #[serde(rename = "netAmount")]
    pub net_amount: f64,
}

/// AI credit usage report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BillingAiCreditUsageReport {
    #[serde(rename = "timePeriod")]
    pub time_period: BillingTimePeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "usageItems")]
    pub usage_items: Vec<BillingModelUsageItem>,
}

/// Premium request usage report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BillingPremiumRequestUsageReport {
    #[serde(rename = "timePeriod")]
    pub time_period: BillingTimePeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "usageItems")]
    pub usage_items: Vec<BillingModelUsageItem>,
}

/// Item in an AI credit or premium request usage report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BillingModelUsageItem {
    pub product: String,
    pub sku: String,
    pub model: String,
    #[serde(rename = "unitType")]
    pub unit_type: String,
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: f64,
    #[serde(rename = "grossQuantity")]
    pub gross_quantity: f64,
    #[serde(rename = "grossAmount")]
    pub gross_amount: f64,
    #[serde(rename = "discountQuantity")]
    pub discount_quantity: f64,
    #[serde(rename = "discountAmount")]
    pub discount_amount: f64,
    #[serde(rename = "netQuantity")]
    pub net_quantity: f64,
    #[serde(rename = "netAmount")]
    pub net_amount: f64,
}

// =========================================================================
// Organization Budgets Models
// =========================================================================

/// Budget scope enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetScope {
    Enterprise,
    Organization,
    Repository,
    CostCenter,
    MultiUserCustomer,
    MultiUserCostCenter,
    User,
}

/// Budget pricing type enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetType {
    SkuPricing,
    ProductPricing,
}

/// Alerting configuration for a budget.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetAlerting {
    pub will_alert: bool,
    pub alert_recipients: Vec<String>,
}

/// Budget entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Budget {
    pub id: String,
    pub budget_type: BudgetType,
    pub budget_amount: u64,
    pub prevent_further_usage: bool,
    pub budget_scope: BudgetScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_entity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_amount: Option<f64>,
    pub budget_product_sku: String,
    pub budget_alerting: BudgetAlerting,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Response returned when listing budgets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetAllBudgets {
    pub budgets: Vec<Budget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_budget: Option<EffectiveBudget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
}

/// Effective budget details for a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectiveBudget {
    pub id: String,
    pub budget_amount: u64,
    pub consumed_amount: f64,
}

/// Request payload to create a budget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBudget {
    pub budget_amount: u64,
    pub prevent_further_usage: bool,
    pub budget_alerting: BudgetAlerting,
    pub budget_scope: BudgetScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_entity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<BudgetType>,
    pub budget_product_sku: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Response returned on budget creation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBudgetResponse {
    pub message: String,
    pub budget: Budget,
}

/// Request payload to update a budget.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateBudget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_further_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_alerting: Option<BudgetAlerting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_scope: Option<BudgetScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_entity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<BudgetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_product_sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Response returned on budget update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateBudgetResponse {
    pub message: String,
    pub budget: Budget,
}

/// Response returned on budget deletion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteBudgetResponse {
    pub message: String,
    pub id: String,
}
