use chrono::NaiveDate;

use super::super::*;

// implements https://docs.github.com/en/rest/copilot/copilot-metrics
// as of API Version 2022-11-28
// missing:
// - copilot_dotcom_chat
// - copilot_dotcom_pull_requests
// - copilot_ide_chat
//
// This requires a API Key with the `copilot` scope and authorized
// to access an enterprise. As of writing, the `copilot` scope is only
// available to GitHub Enterprise customers and limited to Enterprise Administrators.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopilotMetrics {
    pub date: NaiveDate,
    pub total_active_users: u32,
    pub total_engaged_users: u32,
    pub copilot_ide_code_completions: CopilotIdeCodeCompletions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopilotIdeCodeCompletions {
    pub total_engaged_users: u32,
    pub languages: Vec<Language>,
    pub editors: Vec<Editor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub total_engaged_users: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Editor {
    pub name: String,
    pub total_engaged_users: u32,
    pub models: Vec<Model>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub is_custom_model: bool,
    pub custom_model_training_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_engaged_users: Option<u32>,
    pub languages: Vec<EditorLanguage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditorLanguage {
    pub name: String,
    pub total_engaged_users: u32,
    pub total_code_suggestions: u32,
    pub total_code_acceptances: u32,
    pub total_code_lines_suggested: u32,
    pub total_code_lines_accepted: u32,
}
