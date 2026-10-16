use serde::Serialize;

use crate::models::projects_v2::{
    ProjectV2, ProjectV2Field, ProjectV2Item, ProjectV2ItemResponse, ProjectV2View,
    ProjectV2ViewResponse,
};
use crate::{Octocrab, Page, Result};

/// A client to GitHub's Projects (v2) API.
///
/// Created with [`Octocrab::projects_v2`].
pub struct ProjectsV2Handler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ProjectsV2Handler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Access project endpoints scoped to an organization.
    pub fn org<O: Into<String>>(&self, org: O) -> OrgProjectsV2Handler<'octo> {
        OrgProjectsV2Handler {
            crab: self.crab,
            org: org.into(),
        }
    }

    /// Access project endpoints scoped to a user.
    pub fn user<U: Into<String>>(&self, user: U) -> UserProjectsV2Handler<'octo> {
        UserProjectsV2Handler {
            crab: self.crab,
            user: user.into(),
        }
    }
}

/// Handler for organization-scoped Projects (v2).
pub struct OrgProjectsV2Handler<'octo> {
    crab: &'octo Octocrab,
    org: String,
}

impl<'octo> OrgProjectsV2Handler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: impl Into<String>) -> Self {
        Self {
            crab,
            org: org.into(),
        }
    }

    /// List projects for the organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/projects?apiVersion=2022-11-28#list-projects-for-organization)
    pub fn list(&self) -> ListProjectsV2Builder<'octo, '_> {
        ListProjectsV2Builder::new(self.crab, format!("/orgs/{}/projectsV2", self.org))
    }

    /// Get a project for the organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/projects?apiVersion=2022-11-28#get-project-for-organization)
    pub async fn get(&self, project_number: u64) -> Result<ProjectV2> {
        let route = format!("/orgs/{}/projectsV2/{}", self.org, project_number);
        self.crab.get(route, None::<&()>).await
    }

    /// Create a draft item in an organization-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/drafts?apiVersion=2022-11-28#create-draft-item-for-organization-owned-project)
    pub fn create_draft(
        &self,
        project_number: u64,
        title: impl Into<String>,
    ) -> CreateDraftItemBuilder<'octo, '_> {
        CreateDraftItemBuilder::new(
            self.crab,
            format!("/orgs/{}/projectsV2/{}/drafts", self.org, project_number),
            title.into(),
        )
    }

    /// List project fields for an organization-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/fields?apiVersion=2022-11-28#list-project-fields-for-organization)
    pub fn list_fields(&self, project_number: u64) -> ListProjectV2FieldsBuilder<'octo, '_> {
        ListProjectV2FieldsBuilder::new(
            self.crab,
            format!("/orgs/{}/projectsV2/{}/fields", self.org, project_number),
        )
    }

    /// Add a field to an organization-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/fields?apiVersion=2022-11-28#add-a-field-to-an-organization-owned-project)
    pub fn add_field(
        &self,
        project_number: u64,
        name: impl Into<String>,
        data_type: impl Into<String>,
    ) -> CreateProjectFieldBuilder<'octo, '_> {
        CreateProjectFieldBuilder::new(
            self.crab,
            format!("/orgs/{}/projectsV2/{}/fields", self.org, project_number),
            name.into(),
            data_type.into(),
        )
    }

    /// Get a project field for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/fields?apiVersion=2022-11-28#get-project-field-for-organization)
    pub async fn get_field(
        &self,
        project_number: u64,
        field_id: impl AsRef<str>,
    ) -> Result<ProjectV2Field> {
        let route = format!(
            "/orgs/{}/projectsV2/{}/fields/{}",
            self.org,
            project_number,
            field_id.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// List items for an organization-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#list-items-for-an-organization-owned-project)
    pub fn list_items(&self, project_number: u64) -> ListProjectV2ItemsBuilder<'octo, '_> {
        ListProjectV2ItemsBuilder::new(
            self.crab,
            format!("/orgs/{}/projectsV2/{}/items", self.org, project_number),
        )
    }

    /// Add an item to an organization-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#add-item-to-organization-owned-project)
    pub async fn add_item(
        &self,
        project_number: u64,
        item: &impl Serialize,
    ) -> Result<ProjectV2Item> {
        let route = format!("/orgs/{}/projectsV2/{}/items", self.org, project_number);
        let res: ProjectV2ItemResponse = self.crab.post(route, Some(item)).await?;
        Ok(res.into())
    }

    /// Get an item for an organization-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#get-an-item-for-an-organization-owned-project)
    pub async fn get_item(
        &self,
        project_number: u64,
        item_id: u64,
        fields: Option<impl AsRef<str>>,
    ) -> Result<ProjectV2Item> {
        let mut route = format!(
            "/orgs/{}/projectsV2/{}/items/{}",
            self.org, project_number, item_id
        );
        if let Some(f) = fields {
            route.push_str("?fields=");
            route.push_str(f.as_ref());
        }
        let res: ProjectV2ItemResponse = self.crab.get(route, None::<&()>).await?;
        Ok(res.into())
    }

    /// Update a project item for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#update-project-item-for-organization)
    pub async fn update_item(
        &self,
        project_number: u64,
        item_id: u64,
        body: &impl Serialize,
    ) -> Result<ProjectV2Item> {
        let route = format!(
            "/orgs/{}/projectsV2/{}/items/{}",
            self.org, project_number, item_id
        );
        let res: ProjectV2ItemResponse = self.crab.patch(route, Some(body)).await?;
        Ok(res.into())
    }

    /// Delete a project item for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#delete-project-item-for-organization)
    pub async fn delete_item(&self, project_number: u64, item_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{}/projectsV2/{}/items/{}",
            self.org, project_number, item_id
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// List items for an organization project view.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#list-items-for-an-organization-project-view)
    pub fn list_view_items(
        &self,
        project_number: u64,
        view_number: u64,
    ) -> ListProjectV2ItemsBuilder<'octo, '_> {
        ListProjectV2ItemsBuilder::new(
            self.crab,
            format!(
                "/orgs/{}/projectsV2/{}/views/{}/items",
                self.org, project_number, view_number
            ),
        )
    }

    /// Create a view for an organization-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/views?apiVersion=2022-11-28#create-a-view-for-an-organization-owned-project)
    pub fn create_view(
        &self,
        project_number: u64,
        name: impl Into<String>,
        layout: impl Into<String>,
    ) -> CreateProjectViewBuilder<'octo, '_> {
        CreateProjectViewBuilder::new(
            self.crab,
            format!("/orgs/{}/projectsV2/{}/views", self.org, project_number),
            name.into(),
            layout.into(),
        )
    }
}

/// Handler for user-scoped Projects (v2).
pub struct UserProjectsV2Handler<'octo> {
    crab: &'octo Octocrab,
    user: String,
}

impl<'octo> UserProjectsV2Handler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, user: impl Into<String>) -> Self {
        Self {
            crab,
            user: user.into(),
        }
    }

    /// List projects for a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/projects?apiVersion=2022-11-28#list-projects-for-user)
    pub fn list(&self) -> ListProjectsV2Builder<'octo, '_> {
        ListProjectsV2Builder::new(self.crab, format!("/users/{}/projectsV2", self.user))
    }

    /// Get a project for a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/projects?apiVersion=2022-11-28#get-project-for-user)
    pub async fn get(&self, project_number: u64) -> Result<ProjectV2> {
        let route = format!("/users/{}/projectsV2/{}", self.user, project_number);
        self.crab.get(route, None::<&()>).await
    }

    /// Create a draft item for a user-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/drafts?apiVersion=2022-11-28#create-draft-item-for-user-owned-project)
    pub fn create_draft(
        &self,
        project_number: u64,
        title: impl Into<String>,
    ) -> CreateDraftItemBuilder<'octo, '_> {
        CreateDraftItemBuilder::new(
            self.crab,
            format!("/user/{}/projectsV2/{}/drafts", self.user, project_number),
            title.into(),
        )
    }

    /// List project fields for a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/fields?apiVersion=2022-11-28#list-project-fields-for-user)
    pub fn list_fields(&self, project_number: u64) -> ListProjectV2FieldsBuilder<'octo, '_> {
        ListProjectV2FieldsBuilder::new(
            self.crab,
            format!("/users/{}/projectsV2/{}/fields", self.user, project_number),
        )
    }

    /// Add a field to a user-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/fields?apiVersion=2022-11-28#add-field-to-user-owned-project)
    pub fn add_field(
        &self,
        project_number: u64,
        name: impl Into<String>,
        data_type: impl Into<String>,
    ) -> CreateProjectFieldBuilder<'octo, '_> {
        CreateProjectFieldBuilder::new(
            self.crab,
            format!("/users/{}/projectsV2/{}/fields", self.user, project_number),
            name.into(),
            data_type.into(),
        )
    }

    /// Get a project field for a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/fields?apiVersion=2022-11-28#get-project-field-for-user)
    pub async fn get_field(
        &self,
        project_number: u64,
        field_id: impl AsRef<str>,
    ) -> Result<ProjectV2Field> {
        let route = format!(
            "/users/{}/projectsV2/{}/fields/{}",
            self.user,
            project_number,
            field_id.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// List items for a user-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#list-items-for-a-user-owned-project)
    pub fn list_items(&self, project_number: u64) -> ListProjectV2ItemsBuilder<'octo, '_> {
        ListProjectV2ItemsBuilder::new(
            self.crab,
            format!("/users/{}/projectsV2/{}/items", self.user, project_number),
        )
    }

    /// Add an item to a user-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#add-item-to-user-owned-project)
    pub async fn add_item(
        &self,
        project_number: u64,
        item: &impl Serialize,
    ) -> Result<ProjectV2Item> {
        let route = format!("/users/{}/projectsV2/{}/items", self.user, project_number);
        let res: ProjectV2ItemResponse = self.crab.post(route, Some(item)).await?;
        Ok(res.into())
    }

    /// Get an item for a user-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#get-an-item-for-a-user-owned-project)
    pub async fn get_item(
        &self,
        project_number: u64,
        item_id: u64,
        fields: Option<impl AsRef<str>>,
    ) -> Result<ProjectV2Item> {
        let mut route = format!(
            "/users/{}/projectsV2/{}/items/{}",
            self.user, project_number, item_id
        );
        if let Some(f) = fields {
            route.push_str("?fields=");
            route.push_str(f.as_ref());
        }
        let res: ProjectV2ItemResponse = self.crab.get(route, None::<&()>).await?;
        Ok(res.into())
    }

    /// Update a project item for a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#update-project-item-for-user)
    pub async fn update_item(
        &self,
        project_number: u64,
        item_id: u64,
        body: &impl Serialize,
    ) -> Result<ProjectV2Item> {
        let route = format!(
            "/users/{}/projectsV2/{}/items/{}",
            self.user, project_number, item_id
        );
        let res: ProjectV2ItemResponse = self.crab.patch(route, Some(body)).await?;
        Ok(res.into())
    }

    /// Delete a project item for a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#delete-project-item-for-user)
    pub async fn delete_item(&self, project_number: u64, item_id: u64) -> Result<()> {
        let route = format!(
            "/users/{}/projectsV2/{}/items/{}",
            self.user, project_number, item_id
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// List items for a user project view.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/items?apiVersion=2022-11-28#list-items-for-a-user-project-view)
    pub fn list_view_items(
        &self,
        project_number: u64,
        view_number: u64,
    ) -> ListProjectV2ItemsBuilder<'octo, '_> {
        ListProjectV2ItemsBuilder::new(
            self.crab,
            format!(
                "/users/{}/projectsV2/{}/views/{}/items",
                self.user, project_number, view_number
            ),
        )
    }

    /// Create a view for a user-owned project.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/projects/views?apiVersion=2022-11-28#create-a-view-for-a-user-owned-project)
    pub fn create_view(
        &self,
        project_number: u64,
        name: impl Into<String>,
        layout: impl Into<String>,
    ) -> CreateProjectViewBuilder<'octo, '_> {
        CreateProjectViewBuilder::new(
            self.crab,
            format!("/users/{}/projectsV2/{}/views", self.user, project_number),
            name.into(),
            layout.into(),
        )
    }
}

// -------------------------------- Builders --------------------------------

/// Builder for listing Projects (v2).
#[derive(Serialize)]
pub struct ListProjectsV2Builder<'octo, 'b> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip)]
    _marker: std::marker::PhantomData<&'b ()>,
}

impl<'octo, 'b> ListProjectsV2Builder<'octo, 'b> {
    pub(crate) fn new(crab: &'octo Octocrab, route: String) -> Self {
        Self {
            crab,
            route,
            q: None,
            before: None,
            after: None,
            per_page: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn q(mut self, q: impl Into<String>) -> Self {
        self.q = Some(q.into());
        self
    }

    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn per_page(mut self, per_page: u8) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub async fn send(self) -> Result<Page<ProjectV2>> {
        self.crab.get(&self.route, Some(&self)).await
    }
}

/// Builder for creating draft project items.
#[derive(Serialize)]
pub struct CreateDraftItemBuilder<'octo, 'b> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip)]
    _marker: std::marker::PhantomData<&'b ()>,
}

impl<'octo, 'b> CreateDraftItemBuilder<'octo, 'b> {
    pub(crate) fn new(crab: &'octo Octocrab, route: String, title: String) -> Self {
        Self {
            crab,
            route,
            title,
            body: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub async fn send(self) -> Result<ProjectV2Item> {
        let res: ProjectV2ItemResponse = self.crab.post(&self.route, Some(&self)).await?;
        Ok(res.into())
    }
}

/// Builder for listing project fields.
#[derive(Serialize)]
pub struct ListProjectV2FieldsBuilder<'octo, 'b> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip)]
    _marker: std::marker::PhantomData<&'b ()>,
}

impl<'octo, 'b> ListProjectV2FieldsBuilder<'octo, 'b> {
    pub(crate) fn new(crab: &'octo Octocrab, route: String) -> Self {
        Self {
            crab,
            route,
            before: None,
            after: None,
            per_page: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn per_page(mut self, per_page: u8) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub async fn send(self) -> Result<Vec<ProjectV2Field>> {
        self.crab.get(&self.route, Some(&self)).await
    }
}

/// Builder for creating project fields.
#[derive(Serialize)]
pub struct CreateProjectFieldBuilder<'octo, 'b> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: String,
    name: String,
    data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_field_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_select_options: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iteration_configuration: Option<serde_json::Value>,
    #[serde(skip)]
    _marker: std::marker::PhantomData<&'b ()>,
}

impl<'octo, 'b> CreateProjectFieldBuilder<'octo, 'b> {
    pub(crate) fn new(
        crab: &'octo Octocrab,
        route: String,
        name: String,
        data_type: String,
    ) -> Self {
        Self {
            crab,
            route,
            name,
            data_type,
            issue_field_id: None,
            single_select_options: None,
            iteration_configuration: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn issue_field_id(mut self, issue_field_id: u64) -> Self {
        self.issue_field_id = Some(issue_field_id);
        self
    }

    pub fn single_select_options(mut self, options: Vec<serde_json::Value>) -> Self {
        self.single_select_options = Some(options);
        self
    }

    pub fn iteration_configuration(mut self, config: serde_json::Value) -> Self {
        self.iteration_configuration = Some(config);
        self
    }

    pub async fn send(self) -> Result<ProjectV2Field> {
        self.crab.post(&self.route, Some(&self)).await
    }
}

/// Builder for listing project items.
#[derive(Serialize)]
pub struct ListProjectV2ItemsBuilder<'octo, 'b> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip)]
    _marker: std::marker::PhantomData<&'b ()>,
}

impl<'octo, 'b> ListProjectV2ItemsBuilder<'octo, 'b> {
    pub(crate) fn new(crab: &'octo Octocrab, route: String) -> Self {
        Self {
            crab,
            route,
            q: None,
            fields: None,
            before: None,
            after: None,
            per_page: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn q(mut self, q: impl Into<String>) -> Self {
        self.q = Some(q.into());
        self
    }

    pub fn fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn per_page(mut self, per_page: u8) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub async fn send(self) -> Result<Page<ProjectV2Item>> {
        self.crab.get(&self.route, Some(&self)).await
    }
}

/// Builder for creating project views.
#[derive(Serialize)]
pub struct CreateProjectViewBuilder<'octo, 'b> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: String,
    name: String,
    layout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible_fields: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_by: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_group_by: Option<Vec<u64>>,
    #[serde(skip)]
    _marker: std::marker::PhantomData<&'b ()>,
}

impl<'octo, 'b> CreateProjectViewBuilder<'octo, 'b> {
    pub(crate) fn new(crab: &'octo Octocrab, route: String, name: String, layout: String) -> Self {
        Self {
            crab,
            route,
            name,
            layout,
            filter: None,
            visible_fields: None,
            sort_by: None,
            group_by: None,
            vertical_group_by: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn visible_fields(mut self, fields: Vec<u64>) -> Self {
        self.visible_fields = Some(fields);
        self
    }

    pub fn sort_by(mut self, sort_by: serde_json::Value) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    pub fn group_by(mut self, group_by: Vec<u64>) -> Self {
        self.group_by = Some(group_by);
        self
    }

    pub fn vertical_group_by(mut self, vertical_group_by: Vec<u64>) -> Self {
        self.vertical_group_by = Some(vertical_group_by);
        self
    }

    pub async fn send(self) -> Result<ProjectV2View> {
        let res: ProjectV2ViewResponse = self.crab.post(&self.route, Some(&self)).await?;
        Ok(res.into())
    }
}
