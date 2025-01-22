use chrono::NaiveDate;

use super::super::*;

// implements https://docs.github.com/en/rest/copilot/copilot-usage
// as of API Version 2022-11-28
//
// OAuth app tokens and personal access tokens (classic) need either the manage_billing:copilot, read:org, or read:enterprise scopes to use this endpoint.
// Some of these permissions, as of writing, are only available to GitHub Enterprise customers and further limited to Enterprise Administrators.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopilotUsage {
    pub day: NaiveDate,
    pub total_suggestions_count: u32,
    pub total_acceptances_count: u32,
    pub total_lines_suggested: u32,
    pub total_lines_accepted: u32,
    pub total_active_users: u32,
    pub total_chat_acceptances: u32,
    pub total_chat_turns: u32,
    pub total_active_chat_users: u32,
    pub breakdown: Vec<CopilotBreakdown>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopilotBreakdown {
    pub language: String,
    pub editor: String,
    pub suggestions_count: u32,
    pub acceptances_count: u32,
    pub lines_suggested: u32,
    pub lines_accepted: u32,
    pub active_users: u32,
}
