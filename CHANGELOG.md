# CHANGELOG
All  Octocrab releases are supported by the community and through
[GitHub Sponsors][sp].

## 0.4.0

### New APIs

- [`actions`] Control and automate GitHub Actions.
- [`current`] Metadata about the currently authenticated user.
- [`gitignore`]  Get and generate gitignore templates.
- [`licenses`] Metadata about licenses.
- [`markdown`] Render markdown with GitHub.
- [`orgs`] Organisations
- [`pulls`] Pull Requests
- [`repos`] Repositories
- [`search`] Search using GitHub's query syntax.

### Added Methods

- [`Octocrab::graphql`][`graphql`] Send a GraphQL request.

[`actions`]: https://docs.rs/octocrab/0.4.0/octocrab/actions/struct.ActionsHandler.html
[`current`]: https://docs.rs/octocrab/0.4.0/octocrab/current/struct.CurrentAuthHandler.html
[`gitignore`]: https://docs.rs/octocrab/0.4.0/octocrab/gitignore/struct.GitignoreHandler.html
[`graphql`]: https://docs.rs/octocrab/0.4.0/octocrab/struct.Octocrab.html#graphql-api
[`markdown`]: https://docs.rs/octocrab/0.4.0/octocrab/gitignore/struct.MarkdownHandler.html
[`issues`]: https://docs.rs/octocrab/0.4.0/octocrab/issues/struct.IssueHandler.html
[`licenses`]: https://docs.rs/octocrab/0.4.0/octocrab/licenses/struct.LicenseHandler.html
[`pulls`]: https://docs.rs/octocrab/0.4.0/octocrab/pulls/struct.PullRequestHandler.html
[`orgs`]: https://docs.rs/octocrab/0.4.0/octocrab/orgs/struct.OrgHandler.html
[`repos`]: https://docs.rs/octocrab/0.4.0/octocrab/repos/struct.RepoHandler.html
[`search`]: https://docs.rs/octocrab/0.4.0/octocrab/search/struct.SearchHandler.html
[`teams`]: https://docs.rs/octocrab/0.4.0/octocrab/teams/struct.TeamHandler.html
[sp]: https://github.com/sponsors/XAMPPRocky
