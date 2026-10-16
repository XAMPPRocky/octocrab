use crate::api::orgs::OrgHandler;
use crate::Result;

/// Handler for organization insights API.
pub struct OrgInsightsHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgInsightsHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Get route stats by actor.
    pub async fn route_stats(
        &self,
        actor_type: impl AsRef<str>,
        actor_id: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/insights/api/route-stats/{}/{}",
            self.handler.owner,
            actor_type.as_ref(),
            actor_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get subject stats.
    pub async fn subject_stats(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/insights/api/subject-stats", self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get summary stats.
    pub async fn summary_stats(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/insights/api/summary-stats", self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get summary stats by user.
    pub async fn summary_stats_by_user(
        &self,
        user_id: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/insights/api/summary-stats/users/{}",
            self.handler.owner,
            user_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get summary stats by actor.
    pub async fn summary_stats_by_actor(
        &self,
        actor_type: impl AsRef<str>,
        actor_id: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/insights/api/summary-stats/{}/{}",
            self.handler.owner,
            actor_type.as_ref(),
            actor_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get time stats.
    pub async fn time_stats(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/insights/api/time-stats", self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get time stats by user.
    pub async fn time_stats_by_user(&self, user_id: impl AsRef<str>) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/insights/api/time-stats/users/{}",
            self.handler.owner,
            user_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get time stats by actor.
    pub async fn time_stats_by_actor(
        &self,
        actor_type: impl AsRef<str>,
        actor_id: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/insights/api/time-stats/{}/{}",
            self.handler.owner,
            actor_type.as_ref(),
            actor_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get user stats.
    pub async fn user_stats(&self, user_id: impl AsRef<str>) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/insights/api/user-stats/{}",
            self.handler.owner,
            user_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }
}
