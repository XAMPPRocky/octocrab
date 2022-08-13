//! The gist API
mod list_commits;

use serde::Serialize;
use std::collections::BTreeMap;

use crate::{models::gists::{Gist, GistRevision}, Octocrab, Result};
pub use self::list_commits::ListCommitsBuilder;

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
    ///
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

    /// Update an existing gist.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let gitignore = octocrab::instance()
    ///   .gists()
    ///   .update("aa5a315d61ae9438b18d")
    ///   // Optional Parameters
    ///   .description("Updated!")
    ///   .file("hello_world.rs")
    ///   .rename_to("fibonacci.rs")
    ///   .with_content("fn main() {\n println!(\"I should be a Fibonacci!\");\n}")
    ///   .file("delete_me.rs")
    ///   .delete()
    ///   .send()
    ///   .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, id: impl AsRef<str>) -> UpdateGistBuilder<'octo> {
        UpdateGistBuilder::new(self.crab, format!("gists/{id}", id=id.as_ref()))
    }

    /// Get a single gist.
    ///
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
    
    /// Get a single gist revision.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let revision = octocrab::instance()
    ///     .gists()
    ///     .get_revision("00000000000000000000000000000000", "1111111111111111111111111111111111111111")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_revision(&self, id: impl AsRef<str>, sha1: impl AsRef<str>) -> Result<GistRevision> {
        let id = id.as_ref();
        let sha1 = sha1.as_ref();
        self.crab.get(format!("/gists/{}/{}", id, sha1), None::<&()>).await
    }

    /// List commits for the specified gist.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// // Get the least active repos belonging to `owner`.
    /// let page = octocrab::instance()
    ///     .gists()
    ///     .list_commits("00000000000000000000000000000000")
    ///     // Optional Parameters
    ///     .per_page(25)
    ///     .page(5u32)
    ///     // Send the request.
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_commits(&self, gist_id: impl Into<String>) -> list_commits::ListCommitsBuilder {
        list_commits::ListCommitsBuilder::new(self, gist_id.into())
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

    /// Set a description for the gist to be created.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.data.description = Some(description.into());
        self
    }

    /// Set the `public` flag of the gist to be created.
    pub fn public(mut self, public: bool) -> Self {
        self.data.public = Some(public);
        self
    }

    /// Add a file to the gist with `filename` and `content`.
    pub fn file(mut self, filename: impl Into<String>, content: impl Into<String>) -> Self {
        let file = CreateGistFile {
            filename: Default::default(),
            content: content.into(),
        };
        self.data.files.insert(filename.into(), file);
        self
    }

    /// Send the `CreateGist` request to Github for execution.
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

#[derive(Debug)]
pub struct UpdateGistBuilder<'octo> {
    crab: &'octo Octocrab,
    gist_path: String,
    data: UpdateGist
}

impl<'octo> UpdateGistBuilder<'octo> {
    fn new(crab: &'octo Octocrab, gist_path: String) -> Self {
        Self {
            crab,
            gist_path,
            data: Default::default()
        }
    }

    /// Update the description of the the gist with the content provided by `description`.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.data.description = Some(description.into());
        self
    }

    /// Update the file with the `filename`. 
    ///
    /// The update operation is chosen in further calls to the returned builder.
    pub fn file(self, filename: impl Into<String>) -> UpdateGistFileBuilder<'octo> {
        UpdateGistFileBuilder::new(self, filename)
    }

    /// Send the `UpdateGist` command to Github for execution.
    pub async fn send(self) -> Result<Gist> {
        self.crab.patch(self.gist_path, Some(&self.data)).await
    }
}

#[derive(Debug, Default, Serialize)]
struct UpdateGist {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<BTreeMap<String, Option<UpdateGistFile>>>
}

#[derive(Debug, Default, Serialize)]
pub struct UpdateGistFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>
}

pub struct UpdateGistFileBuilder<'octo> {
    builder: UpdateGistBuilder<'octo>,
    filename: String,
    file: Option<UpdateGistFile>,
    ready: bool
}

impl<'octo> UpdateGistFileBuilder<'octo> {
    fn new(builder: UpdateGistBuilder<'octo>, filename: impl Into<String>) -> Self {
        Self {
            builder,
            filename: filename.into(),
            file: None,
            ready: false
        }
    }

    fn build(mut self) -> UpdateGistBuilder<'octo> {
        if self.ready {
            self.builder.data.files.get_or_insert_with(BTreeMap::new).insert(self.filename, self.file);
        }
        self.builder
    }

    /// Delete the file from the gist.
    pub fn delete(mut self) -> UpdateGistBuilder<'octo> {
        self.ready = true;
        self.file = None;
        self.build()
    }

    /// Rename the file to `filename`.
    pub fn rename_to(mut self, filename: impl Into<String>) -> Self {
        self.ready = true;
        self.file.get_or_insert_with(Default::default).filename = Some(filename.into());
        self
    }

    /// Update the content of the file and overwrite it with `content`.
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.ready = true;
        self.file.get_or_insert_with(Default::default).content = Some(content.into());
        self
    }

    /// Overwrite the Description of the gist with `description`.
    ///
    /// This will finalize the update operation and will continue to operate on the gist itself.
    pub fn description(self, description: impl Into<String>) -> UpdateGistBuilder<'octo> {
        self.build().description(description)
    }

    /// Update the next file identified by `filename`.
    ///
    /// This will finalize the update operation and will continue to operate on the gist itself.
    pub fn file(self, filename: impl Into<String>) -> UpdateGistFileBuilder<'octo> {
        self.build().file(filename)
    }

    /// Send the `UpdateGist` command to Github for execution.
    /// 
    /// This will finalize the update operation before sending.
    pub async fn send(self) -> Result<Gist> {
        self.build().send().await
    }
}
