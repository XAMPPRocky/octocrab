//! The gist API

use serde::Serialize;
use std::collections::BTreeMap;

use crate::{models::gists::Gist, Octocrab, Result};

/// Handler for GitHub's gist API.
///
/// Created with [`Octocrab::gists`].
pub struct GistsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> GistsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Create a new gist.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let gitignore = octocrab::instance()
    ///     .gists()
    ///     .create()
    ///     .file("hello_world.rs", "fn main() {\n println!(\"Hello World!\");\n}")
    ///     // Optional Parameters
    ///     .description("Hello World in Rust")
    ///     .public(false)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self) -> CreateGistBuilder<'octo> {
        CreateGistBuilder::new(self.crab)
    }

    /// Get a single gist.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let gist = octocrab::instance().gists().get("00000000000000000000000000000000").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Gist> {
        let id = id.as_ref();
        self.crab.get(format!("/gists/{}", id), None::<&()>).await
    }
}

#[derive(Debug)]
pub struct CreateGistBuilder<'octo> {
    crab: &'octo Octocrab,
    data: CreateGist,
}

impl<'octo> CreateGistBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            data: Default::default(),
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.data.description = Some(description.into());
        self
    }

    pub fn public(mut self, public: bool) -> Self {
        self.data.public = Some(public);
        self
    }

    pub fn file(mut self, filename: impl Into<String>, content: impl Into<String>) -> Self {
        let file = CreateGistFile {
            filename: Default::default(),
            content: content.into(),
        };
        self.data.files.insert(filename.into(), file);
        self
    }

    pub async fn send(self) -> Result<Gist> {
        self.crab.post("gists", Some(&self.data)).await
    }
}

#[derive(Debug, Default, Serialize)]
struct CreateGist {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public: Option<bool>,
    files: BTreeMap<String, CreateGistFile>,
}

#[derive(Debug, Serialize)]
struct CreateGistFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    content: String,
}
