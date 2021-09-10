# Octocrab: A modern, extensible GitHub API client.

[![Rust](https://github.com/XAMPPRocky/octocrab/workflows/Rust/badge.svg)](https://github.com/XAMPPRocky/octocrab/actions?query=workflow%3ARust)
[![crates.io](https://img.shields.io/crates/d/octocrab.svg)](https://crates.io/crates/octocrab)
[![Help Wanted](https://img.shields.io/github/issues/XAMPPRocky/octocrab/help%20wanted?color=green)](https://github.com/XAMPPRocky/octocrab/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![Lines Of Code](https://tokei.rs/b1/github/XAMPPRocky/octocrab?category=code)](https://github.com/XAMPPRocky/octocrab)
[![Documentation](https://docs.rs/octocrab/badge.svg)](https://docs.rs/octocrab/)

Octocrab is a third party GitHub API client, allowing you to easily build
your own GitHub integrations or bots in Rust. `Octocrab` comes with two primary
sets of APIs for communicating with GitHub, a high level strongly typed
semantic API, and a lower level HTTP API for extending behaviour.

#### Cargo.toml
```toml
octocrab = "0.9"
```

## Semantic API
The semantic API provides strong typing around GitHub's API, a set of
[`models`] that maps to GitHub's types, and [`auth`] functions that are useful
for GitHub apps.
Currently, the following modules are available.

- [`actions`] GitHub Actions.
- [`current`] Information about the current user.
- [`gitignore`] Gitignore templates.
- [`graphql`] GraphQL.
- [`issues`] Issues and related items, e.g. comments, labels, etc.
- [`licenses`] License Metadata.
- [`markdown`] Rendering Markdown with GitHub.
- [`orgs`] GitHub Organisations.
- [`pulls`] Pull Requests.
- [`releases`] Releases.
- [`repos`] Repositories.
- [`search`] GitHub's search API.
- [`teams`] Teams.

[`models`]: https://docs.rs/octocrab/latest/octocrab/models/index.html
[`models`]: https://docs.rs/octocrab/latest/octocrab/auth/index.html

[`actions`]: https://docs.rs/octocrab/latest/octocrab/actions/struct.ActionsHandler.html
[`current`]: https://docs.rs/octocrab/latest/octocrab/current/struct.CurrentAuthHandler.html
[`gitignore`]: https://docs.rs/octocrab/latest/octocrab/gitignore/struct.GitignoreHandler.html
[`graphql`]: https://docs.rs/octocrab/latest/octocrab/struct.Octocrab.html#graphql-api
[`markdown`]: https://docs.rs/octocrab/latest/octocrab/gitignore/struct.MarkdownHandler.html
[`issues`]: https://docs.rs/octocrab/latest/octocrab/issues/struct.IssueHandler.html
[`licenses`]: https://docs.rs/octocrab/latest/octocrab/licenses/struct.LicenseHandler.html
[`pulls`]: https://docs.rs/octocrab/latest/octocrab/pulls/struct.PullRequestHandler.html
[`orgs`]: https://docs.rs/octocrab/latest/octocrab/orgs/struct.OrgHandler.html
[`repos`]: https://docs.rs/octocrab/latest/octocrab/repos/struct.RepoHandler.html
[`releases`]: https://docs.rs/octocrab/0.8.1/octocrab/repos/struct.ReleasesHandler.html
[`search`]: https://docs.rs/octocrab/latest/octocrab/search/struct.SearchHandler.html
[`teams`]: https://docs.rs/octocrab/latest/octocrab/teams/struct.TeamHandler.html

#### Getting a Pull Request
```rust
// Get pull request #404 from `octocrab/repo`.
let issue = octocrab::instance().pulls("octocrab", "repo").get(404).await?;
```

All methods with multiple optional parameters are built as `Builder`
structs, allowing you to easily specify parameters.

#### Listing issues
```rust
use octocrab::{models, params};

let octocrab = octocrab::instance();
// Returns the first page of all issues.
let page = octocrab.issues("octocrab", "repo")
    .list()
    // Optional Parameters
    .creator("octocrab")
    .state(params::State::All)
    .per_page(50)
    .send()
    .await?;

// Go through every page of issues. Warning: There's no rate limiting so
// be careful.
let mut next_page = page.next;
while let Some(page) = octocrab.get_page::<models::Issue>(&next_page).await? {
    next_page = page.next;
    for issue in page {
        println!("{}", issue.title);
    }
}
```

## HTTP API
The typed API currently doesn't cover all of GitHub's API at this time, and
even if it did GitHub is in active development and this library will
likely always be somewhat behind GitHub at some points in time. However that
shouldn't mean that in order to use those features, you have to fork
or replace `octocrab` with your own solution.

Instead `octocrab` exposes a suite of HTTP methods allowing you to easily
extend `Octocrab`'s existing behaviour. Using these HTTP methods allows you
to keep using the same authentication and configuration, while having
control over the request and response. There is a method for each HTTP
method, `get`, `post`, `patch`, `put`, `delete`, all of which accept a
relative route and a optional body.

```rust
let user: octocrab::models::User = octocrab::instance()
    .get("/user", None::<&()>)
    .await?;
```

Each of the HTTP methods expects a body, formats the URL with the base
URL, and errors if GitHub doesn't return a successful status, but this isn't
always desired when working with GitHub's API, sometimes you need to check
the response status or headers. As such there are companion methods `_get`,
`_post`, etc. that perform no additional pre or post-processing to
the request.

```rust
let octocrab = octocrab::instance();
let response =  octocrab
    ._get("https://api.github.com/organizations", None::<&()>)
    .await?;

// You can also use `Octocrab::absolute_url` if you want to still to go to
// the same base.
let response =  octocrab
    ._get(octocrab.absolute_url("/organizations")?, None::<&()>)
    .await?;
```

You can use the those HTTP methods to easily create your own extensions to
`Octocrab`'s typed API. (Requires `async_trait`).

```rust
use octocrab::{Octocrab, Page, Result, models};

#[async_trait::async_trait]
trait OrganisationExt {
  async fn list_every_organisation(&self) -> Result<Page<models::Organization>>;
}

#[async_trait::async_trait]
impl OrganisationExt for Octocrab {
  async fn list_every_organisation(&self) -> Result<Page<models::Organization>> {
    self.get("/organizations", None::<&()>).await
  }
}
```

You can also easily access new properties that aren't available in the
current models using `serde`.

```rust
#[derive(Deserialize)]
struct RepositoryWithVisibility {
    #[serde(flatten)]
    inner: octocrab::models::Repository,
    visibility: String,
}

let my_repo = octocrab::instance()
    .get::<RepositoryWithVisibility>("https://api.github.com/repos/XAMPPRocky/octocrab", None::<&()>)
    .await?;
```

## Static API
`Octocrab` also provides a statically reference counted version of its API,
allowing you to easily plug it into existing systems without worrying
about having to integrate and pass around the client.

```rust
// Initialises the static instance with your configuration and returns an
// instance of the client.
octocrab::initialise(octocrab::Octocrab::builder());
// Gets a instance of `Octocrab` from the static API. If you call this
// without first calling `octocrab::initialise` a default client will be
// initialised and returned instead.
let octocrab = octocrab::instance();
```


