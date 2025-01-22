use super::super::*;

// implements https://docs.github.com/en/rest/copilot/copilot-user-management
// as of API Version 2022-11-28
//
// We have chosen to not map out the enums as the copilot API is still fresh,
// this means GitHub may add additional enums in the future and they would
// require more maintenance than just providing a String.
//
// For a list of available enums, refer to the "response schema" in the link above.
//
// missing:
// - billing/seats misses the assigning_team field
//
// This requires a API Key with the `copilot:billing` or `enterprise:billing` scope and authorized
// to access an enterprise. As of writing, the `copilot` scope is only
// available to GitHub Enterprise customers and limited to Enterprise Administrators.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopilotBilling {
    pub seat_breakdown: CopilotSeatBreakdown,
    pub seat_management_setting: String,
    pub ide_chat: String,
    pub platform_chat: String,
    pub cli: String,
    pub public_code_suggestions: String,
    pub plan_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopilotSeatBreakdown {
    pub total: u32,
    pub added_this_cycle: u32,
    pub pending_invitation: u32,
    pub pending_cancellation: u32,
    pub active_this_cycle: u32,
    pub inactive_this_cycle: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopilotBillingSeats {
    pub total_seats: u32,
    pub seats: Vec<CopilotSeat>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopilotSeat {
    pub created_at: DateTime<Utc>,
    pub pending_cancellation_date: Option<String>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub last_activity_editor: Option<String>,
    pub plan_type: Option<String>,
    pub assignee: SimpleUser,
}
