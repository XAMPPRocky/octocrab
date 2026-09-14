use std::marker::PhantomData;

use super::*;

pub struct CopilotSeatHandler<'octo, 'r> {
    crab: &'octo Octocrab,
    owner: String,
    _phantom: PhantomData<&'r ()>,
}

#[derive(serde::Serialize)]
struct SelectedTeams {
    selected_teams: Vec<String>,
}

#[derive(serde::Serialize)]
struct SelectedUsernames {
    selected_usernames: Vec<String>,
}

impl<'octo, 'r> CopilotSeatHandler<'octo, 'r> {
    pub fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            crab: handler.crab,
            owner: handler.owner.clone(),
            _phantom: PhantomData,
        }
    }

    pub(crate) fn new_with_owner(crab: &'octo Octocrab, owner: String) -> Self {
        Self {
            crab,
            owner,
            _phantom: PhantomData,
        }
    }

    /// Adds the specified teams from copilot seats.
    /// Note that this adds new seats immediately to your billing cycle.
    pub async fn add_teams(
        self,
        teams: Vec<String>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::SeatsCreated> {
        let route = format!(
            "/orgs/{org}/copilot/billing/selected_teams",
            org = self.owner,
        );
        let teams = SelectedTeams {
            selected_teams: teams,
        };

        self.crab.post(route, Some(&teams)).await
    }

    /// Removes the specified teams from copilot seats.
    /// Note that the seat removal takes effect the next billing cycle.
    pub async fn remove_teams(
        self,
        teams: Vec<String>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::SeatsCancelled> {
        let route = format!(
            "/orgs/{org}/copilot/billing/selected_teams",
            org = self.owner,
        );
        let teams = SelectedTeams {
            selected_teams: teams,
        };

        self.crab.delete(route, Some(&teams)).await
    }

    /// Adds the specified usernames from copilot seats.
    /// Note that this adds new seats immediately to your billing cycle.
    pub async fn add_usernames(
        self,
        usernames: Vec<String>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::SeatsCreated> {
        let route = format!(
            "/orgs/{org}/copilot/billing/selected_users",
            org = self.owner,
        );
        let usernames = SelectedUsernames {
            selected_usernames: usernames,
        };

        self.crab.post(route, Some(&usernames)).await
    }

    /// Removes the specified users from copilot seats.
    /// Note that the seat removal takes effect the next billing cycle.
    pub async fn remove_usernames(
        self,
        usernames: Vec<String>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::SeatsCancelled> {
        let route = format!(
            "/orgs/{org}/copilot/billing/selected_users",
            org = self.owner,
        );
        let usernames = SelectedUsernames {
            selected_usernames: usernames,
        };

        self.crab.delete(route, Some(&usernames)).await
    }

    /// Gets the GitHub Copilot seat assignment details for a member of an organization who currently has access to GitHub Copilot.
    ///
    /// Note: You can also retrieve user seat details directly via [`CopilotHandler::seat_assignment`][crate::api::orgs::copilot::CopilotHandler::seat_assignment].
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/copilot/copilot-user-management?apiVersion=2022-11-28#get-copilot-seat-assignment-details-for-a-user)
    pub async fn get_user(
        &self,
        username: impl AsRef<str>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::CopilotSeat> {
        let route = format!(
            "/orgs/{org}/members/{username}/copilot",
            org = self.owner,
            username = username.as_ref(),
        );

        self.crab.get(route, None::<&()>).await
    }

    /// Shortcut for [`Self::get_user`].
    ///
    /// Note: You can also retrieve user seat details directly via [`CopilotHandler::seat_assignment`][crate::api::orgs::copilot::CopilotHandler::seat_assignment].
    pub async fn seat_assignment(
        &self,
        username: impl AsRef<str>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::CopilotSeat> {
        self.get_user(username).await
    }
}
