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

    /// Update an existing gist.
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
   
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.data.description = Some(description.into());
        self
    }

    pub fn file(self, filename: impl Into<String>) -> UpdateGistFileBuilder<'octo> {
        UpdateGistFileBuilder::new(self, filename)
    }
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
    filename: Option<String>,
    content: Option<String>
}

pub struct UpdateGistFileBuilder<'octo> {
    builder: UpdateGistBuilder<'octo>,
    filename: String,
    file: Option<UpdateGistFile>
}
impl<'octo> UpdateGistFileBuilder<'octo> {
    fn new(builder: UpdateGistBuilder<'octo>, filename: impl Into<String>) -> Self {
        Self {
            builder,
            filename: filename.into(),
            file: None
        }
    }

    fn build(mut self) -> UpdateGistBuilder<'octo> {
        self.builder.data.files.get_or_insert_with(BTreeMap::new).insert(self.filename, self.file);
        self.builder
    }

    pub fn delete(mut self) -> UpdateGistBuilder<'octo> {
        self.file = None;
        self.build()
    }

    pub fn rename_to(mut self, filename: impl Into<String>) -> Self {
        self.file.get_or_insert_with(Default::default).filename = Some(filename.into());
        self
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.file.get_or_insert_with(Default::default).content = Some(content.into());
        self
    }

    pub fn description(self, description: impl Into<String>) -> UpdateGistBuilder<'octo> {
        self.build().description(description)
    }

    pub fn file(self, filename: impl Into<String>) -> UpdateGistFileBuilder<'octo> {
        self.build().file(filename)
    }

    pub async fn send(self) -> Result<Gist> {
        self.build().send().await
    }
}