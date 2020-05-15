# Octocrab: A modern, extensible GitHub API client.

[![Rust](https://github.com/XAMPPRocky/octocrab/workflows/Rust/badge.svg)](https://github.com/XAMPPRocky/octocrab/actions?query=workflow%3ARust)
[![crates.io](https://img.shields.io/crates/d/octocrab.svg)](https://crates.io/crates/octocrab)
[![Help Wanted](https://img.shields.io/github/issues/XAMPPRocky/octocrab/help%20wanted?color=green)](https://github.com/XAMPPRocky/octocrab/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![Lines Of Code](https://tokei.rs/b1/github/XAMPPRocky/octocrab?category=code)](https://github.com/XAMPPRocky/octocrab)
[![Documentation](https://docs.rs/octocrab/badge.svg)](https://docs.rs/octocrab/)

Octocrab is an third party GitHub API client, allowing you to easily build
your own GitHub integrations or bots in Rust. `Octocrab` comes with two primary
set of APIs for communicating with GitHub, a high level strongly typed
semantic API, and a lower level HTTP API for extending behaviour.

#### Cargo.toml
```toml
octocrab = "0.2"
```

## Semantic API
The semantic API provides strong typing around GitHub's API, as well as a
set of [`models`] that maps to GitHub's types. Currently the following 
modules are available.

- [`issues`] Issues and related items, e.g. comments, labels, etc.
- [`pulls`] Pull Requests
- [`orgs`] GitHub Organisations

[`models`]: https://docs.rs/octocrab/0.2.2/octocrab/models/index.html
[`issues`]: https://docs.rs/octocrab/0.2.2/octocrab/issues/struct.IssueHandler.html
[`pulls`]: https://docs.rs/octocrab/0.2.2/octocrab/pulls/struct.PullRequestHandler.html
[`orgs`]: https://docs.rs/octocrab/0.2.2/octocrab/orgs/struct.OrgHandler.html

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
while let Some(page) = octocrab.get_page::<models::Issue>(&page.next).await? {
    for issue in page {
        println!("{}", issue.title);
    }
}
```

## HTTP API
The typed API currently doesn't cover all of GitHub's API at this time, and
even if it did GitHub is in active development and this library will
likely always be somewhat behind GitHub at some points in time. However that
shouldn't mean that in order to use those features that you have to now fork
or replace `octocrab` with your own solution.

Instead `octocrab` exposes a suite of HTTP methods allowing you to easily
extend `Octocrab`'s existing behaviour. Using these HTTP methods allows you
to keep using the same authentication and configuration, while having
control over the request and response. There is a method for each HTTP
method `get`, `post`, `patch`, `put`, `delete`, all of which accept a
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

## Static API
`Octocrab` also provides a statically reference count version of its API,
allowing you to easily plug it into existing systems without worrying
about having to integrate and pass around the client.

```rust
// Initialises the static instance with your configuration and returns an
// instance of the client.
octocrab::initialise(octocrab::Octocrab::builder());
// Gets a instance of `Octocrab` from the static API. If you call this
// without first calling `octocrab::initialise` a default client will be
// initialised and returned instead.
octocrab::instance();
```


