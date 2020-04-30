pub mod issues;
pub mod pulls;

/// The status of a pull request.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StateParameter {
    All,
    Open,
    Closed,
}
