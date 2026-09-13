use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{MarketplaceAccountId, PlanId};

/// A GitHub Marketplace plan.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#list-plans)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketplacePlan {
    pub url: Url,
    pub accounts_url: Url,
    pub id: PlanId,
    pub number: u64,
    pub name: String,
    pub description: String,
    pub monthly_price_in_cents: i64,
    pub yearly_price_in_cents: i64,
    pub price_model: String,
    pub has_free_trial: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
    pub state: String,
    pub bullets: Vec<String>,
}

/// A pending change for a GitHub Marketplace plan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketplacePendingChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<MarketplacePlan>,
}

/// A GitHub Marketplace purchase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketplacePurchase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_free_trial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_trial_ends_on: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    pub plan: MarketplacePlan,
}

/// A GitHub Marketplace account associated with a plan.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#get-a-subscription-plan-for-an-account)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketplaceAccount {
    pub url: Url,
    pub r#type: String,
    pub id: MarketplaceAccountId,
    pub login: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_billing_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_pending_change: Option<MarketplacePendingChange>,
    pub marketplace_purchase: MarketplacePurchase,
}

/// Account information in a user's marketplace purchase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketplaceUserAccount {
    pub url: Url,
    pub r#type: String,
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub login: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_billing_email: Option<String>,
}

/// A user's GitHub Marketplace subscription.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#list-subscriptions-for-the-authenticated-user)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserMarketplacePurchase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_free_trial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_trial_ends_on: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    pub account: MarketplaceUserAccount,
    pub plan: MarketplacePlan,
}
