# Changelog
All  Octocrab releases are supported by the community and through
[GitHub Sponsors][sp].

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.20.0-alpha.2](https://github.com/XAMPPRocky/octocrab/compare/v0.20.0-alpha.1...v0.20.0-alpha.2) - 2023-04-10

### Other
- Extend `GistsHandler` through `star(...)`, `unstar(...)`, `is_starred(...)` (#330)
- added poll org events (#325)
- Add `CurrentAuthHandler::list_gists_for_authenticated_user` (#328)
- Fix typo in POST /gists endpoint (#327)
- Update hyper-rustls requirement from 0.23.2 to 0.24.0 (#324)
- Percent encode label name in remove_label to avoid InvalidUri(InvalidUriChar) error (#323)
- Update CHANGELOG.md
- Update CHANGELOG.md

## [0.20.0-alpha.1](https://github.com/XAMPPRocky/octocrab/compare/v0.20.0-alpha.0...v0.20.0-alpha.1) - 2023-03-31

### Other
- Fix GitHubError / InvalidUri(InvalidFormat)  (#320)
- Fix the spelling of `committer` in `RepoCommitPage` (#316) (#317)
- Add update state reason (#290)
- Add target URL to Status model (#308)
- *(ci)* add release-plz (#309)
- Add remove_requested_reviewer function (#312)
- Make command compatible with copy paste (#318)
- Update tower-http requirement from 0.3.2 to 0.4.0 (#315)

### Added Methods

- [`UpdateIssueBuilder::state_reason`] Updates the state reason.

## 0.4.1
- Relaxed the `body` argument on `Octocrab::graphql` from `impl AsRef<str>` to
  `&impl serde::Serialize + ?Sized` to allow accepting any valid JSON value.
  This is mainly useful for being able to use types from other libraries like
  [`graphql_client`][gql] directly.

[gql]: https://docs.rs/graphql_client

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
- [`IssueHandler::lock`] Lock a GitHub issue with an optional reason.
- [`IssueHandler::unlock`] Unlock a GitHub issue.
- [`IssueHandler::replace_all_labels`] Replace all labels on an issue.
- [`IssueHandler::delete_label`] Remove labels from an issue.
- [`IssueHandler::list_labels_for_issue`] List all labels on an issue.
- [`IssueHandler::list_labels_for_repo`] List all labels in a repository.
- [`PullRequestHandler::media_type`] Set the media type for a single request.
- [`PullRequestHandler::get_diff`] Get a pull request's diff file.
- [`PullRequestHandler::get_patch`] Get a pull request's patch file.
- [`Page::number_of_pages`] Get the number of pages in a paginated query
  if possible.

### Changes
- [`Page<T>`] now has new fields for being used with GitHub's search APi such as
  `incomplete_results` and  `total_count`.

[`actions`]: https://docs.rs/octocrab/0.4.1/octocrab/actions/struct.ActionsHandler.html
[`current`]: https://docs.rs/octocrab/0.4.1/octocrab/current/struct.CurrentAuthHandler.html
[`gitignore`]: https://docs.rs/octocrab/0.4.1/octocrab/gitignore/struct.GitignoreHandler.html
[`graphql`]: https://docs.rs/octocrab/0.4.1/octocrab/struct.Octocrab.html#graphql-api
[`markdown`]: https://docs.rs/octocrab/0.4.1/octocrab/gitignore/struct.MarkdownHandler.html
[`issues`]: https://docs.rs/octocrab/0.4.1/octocrab/issues/struct.IssueHandler.html
[`licenses`]: https://docs.rs/octocrab/0.4.1/octocrab/licenses/struct.LicenseHandler.html
[`pulls`]: https://docs.rs/octocrab/0.4.1/octocrab/pulls/struct.PullRequestHandler.html
[`orgs`]: https://docs.rs/octocrab/0.4.1/octocrab/orgs/struct.OrgHandler.html
[`repos`]: https://docs.rs/octocrab/0.4.1/octocrab/repos/struct.RepoHandler.html
[`search`]: https://docs.rs/octocrab/0.4.1/octocrab/search/struct.SearchHandler.html
[`teams`]: https://docs.rs/octocrab/0.4.1/octocrab/teams/struct.TeamHandler.html
[sp]: https://github.com/sponsors/XAMPPRocky
[`IssueHandler::lock`]: https://docs.rs/octocrab/0.4.1/octocrab/issues/struct.IssueHandler.html#method.lock
[`IssueHandler::unlock`]: https://docs.rs/octocrab/0.4.1/octocrab/issues/struct.IssueHandler.html#method.unlock
[`IssueHandler::replace_all_labels`]: https://docs.rs/octocrab/0.4.1/octocrab/issues/struct.IssueHandler.html#method.replace_all_labels
[`IssueHandler::delete_label`]: https://docs.rs/octocrab/0.4.1/octocrab/issues/struct.IssueHandler.html#method.delete_label
[`IssueHandler::list_labels_for_issue`]: https://docs.rs/octocrab/0.4.1/octocrab/issues/struct.IssueHandler.html#method.list_labels_for_issue
[`IssueHandler::list_labels_for_repo`]: https://docs.rs/octocrab/0.4.1/octocrab/issues/struct.IssueHandler.html#method.list_labels_for_repo
[`PullRequestHandler::media_type`]: https://docs.rs/octocrab/0.4.1/octocrab/pulls/struct.PullRequestHandler.html#method.media_type
[`PullRequestHandler::get_diff`]: https://docs.rs/octocrab/0.4.1/octocrab/pulls/struct.PullRequestHandler.html#method.get_diff
[`PullRequestHandler::get_patch`]: https://docs.rs/octocrab/0.4.1/octocrab/pulls/struct.PullRequestHandler.html#method.get_patch
[`Page<T>`]: https://docs.rs/octocrab/0.4.1/octocrab/struct.Page.html
[`Page::number_of_pages`]: https://docs.rs/octocrab/0.4.1/octocrab/struct.Page.html#method.number_of_pages
