use super::super::*;

/// The weekly aggregate of additions and deletions of code to a repository.
///
/// On the wire, this is represented as a 3-element array: `[timestamp, additions, deletions]`.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-the-weekly-commit-activity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeFrequency(pub i64, pub i64, pub i64);

impl CodeFrequency {
    /// The Unix timestamp for the start of the week.
    pub fn week(&self) -> i64 {
        self.0
    }

    /// Number of additions for the week.
    pub fn additions(&self) -> i64 {
        self.1
    }

    /// Number of deletions for the week (negative number).
    pub fn deletions(&self) -> i64 {
        self.2
    }
}

/// A weekly commit activity breakdown.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-the-last-year-of-commit-activity)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitActivity {
    /// Commit counts for each day of the week, starting with Sunday (index 0) to Saturday (index 6).
    pub days: Vec<i64>,
    /// Total number of commits during the week.
    pub total: i64,
    /// The Unix timestamp for the start of the week.
    pub week: i64,
}

/// Contributor commit activity.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-all-contributor-commit-activity)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContributorActivity {
    /// The contributor details.
    #[serde(default)]
    pub author: Option<Author>,
    /// The total number of commits made by the contributor.
    pub total: i64,
    /// Weekly breakdown of the contributor's activity.
    pub weeks: Vec<ContributorWeeklyActivity>,
}

/// Contributor commit activity for a specific week.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-all-contributor-commit-activity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContributorWeeklyActivity {
    /// Start of the week, given as a Unix timestamp.
    #[serde(rename = "w")]
    pub week: i64,
    /// Number of additions.
    #[serde(rename = "a")]
    pub additions: i64,
    /// Number of deletions.
    #[serde(rename = "d")]
    pub deletions: i64,
    /// Number of commits.
    #[serde(rename = "c")]
    pub commits: i64,
}

/// The weekly commit counts for a repository over the last 52 weeks.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-the-weekly-commit-count)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ParticipationStats {
    /// Commit count for each of the last 52 weeks across all committers.
    pub all: Vec<i64>,
    /// Commit count for each of the last 52 weeks for the repository owner.
    pub owner: Vec<i64>,
}

/// Hourly commit counts for a day of the week.
///
/// On the wire, this is represented as a 3-element array: `[day, hour, commits]`.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-the-hourly-commit-count-for-each-day)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PunchCard(pub u8, pub u8, pub u64);

impl PunchCard {
    /// Day of the week (0 = Sunday, 1 = Monday, ..., 6 = Saturday).
    pub fn day(&self) -> u8 {
        self.0
    }

    /// Hour of the day (0 = midnight, ..., 23 = 11pm).
    pub fn hour(&self) -> u8 {
        self.1
    }

    /// Number of commits during this hour.
    pub fn commits(&self) -> u64 {
        self.2
    }
}
