# Changelog
All  Octocrab releases are supported by the community and through
[GitHub Sponsors][sp].

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.38.0](https://github.com/XAMPPRocky/octocrab/compare/v0.37.0...v0.38.0) - 2024-04-09

### Added
- Add `generate_release_notes`: ([#588](https://github.com/XAMPPRocky/octocrab/pull/588))

### Other
- Unify diff entry models ([#624](https://github.com/XAMPPRocky/octocrab/pull/624))
- Added tests for .delete_thread_subscriptions() ([#620](https://github.com/XAMPPRocky/octocrab/pull/620))
- Adding tests, and fixing missing error code. ([#619](https://github.com/XAMPPRocky/octocrab/pull/619))

## [0.37.0](https://github.com/XAMPPRocky/octocrab/compare/v0.36.0...v0.37.0) - 2024-03-27

### Fixed
- [**breaking**] Change `Rate::reset` to `u64` ([#615](https://github.com/XAMPPRocky/octocrab/pull/615))

### Other
- Update graphql_client requirement from 0.13.0 to 0.14.0 ([#617](https://github.com/XAMPPRocky/octocrab/pull/617))
- Added basic support for getting user profiles ([#616](https://github.com/XAMPPRocky/octocrab/pull/616))
- Update manual github app authentication example ([#610](https://github.com/XAMPPRocky/octocrab/pull/610))

## [0.36.0](https://github.com/XAMPPRocky/octocrab/compare/v0.35.0...v0.36.0) - 2024-03-19

### Added
- WASM Support ([#591](https://github.com/XAMPPRocky/octocrab/pull/591))

### Other
- fix some comments ([#611](https://github.com/XAMPPRocky/octocrab/pull/611))
- Add status code to GithubError ([#607](https://github.com/XAMPPRocky/octocrab/pull/607))
- Add `Pending` variant to `octocrab::models::workflows::Status` ([#604](https://github.com/XAMPPRocky/octocrab/pull/604))
- Fix personal token auth for pagination ([#602](https://github.com/XAMPPRocky/octocrab/pull/602))
- Fixed some `.gists()` functions, and wrote tests. ([#606](https://github.com/XAMPPRocky/octocrab/pull/606))
- Allow directly passing SecretString ([#601](https://github.com/XAMPPRocky/octocrab/pull/601))

## [0.35.0](https://github.com/XAMPPRocky/octocrab/compare/v0.34.3...v0.35.0) - 2024-03-10

### Added
- add repository list language support ([#595](https://github.com/XAMPPRocky/octocrab/pull/595))

### Other
- Added test for `actions().create_workflow_dispatch()` ([#603](https://github.com/XAMPPRocky/octocrab/pull/603))
- added "Create a new Git commit object" ([#600](https://github.com/XAMPPRocky/octocrab/pull/600))
- Added `check-suites` endpoints ([#596](https://github.com/XAMPPRocky/octocrab/pull/596))
- Adding org secrets tests ([#599](https://github.com/XAMPPRocky/octocrab/pull/599))

## [0.34.3](https://github.com/XAMPPRocky/octocrab/compare/v0.34.2...v0.34.3) - 2024-03-05

### Other
- Add delete_ref in RepoHandler ([#573](https://github.com/XAMPPRocky/octocrab/pull/573))
- Adding tests to `orgs().secrets().create_or_update_secret()` ([#590](https://github.com/XAMPPRocky/octocrab/pull/590))

## [0.34.2](https://github.com/XAMPPRocky/octocrab/compare/v0.34.1...v0.34.2) - 2024-03-04

### Other
- Update base64 requirement from 0.21.0 to 0.22.0 ([#592](https://github.com/XAMPPRocky/octocrab/pull/592))
- Add Public and Release events ([#589](https://github.com/XAMPPRocky/octocrab/pull/589))
- Adding tests for `actions().remove_selected_repo_from_org_secret()` ([#586](https://github.com/XAMPPRocky/octocrab/pull/586))
- Fixed the `projects().delete_project().send() function ([#587](https://github.com/XAMPPRocky/octocrab/pull/587))
- Added more tests, for `repos().delete()` ([#585](https://github.com/XAMPPRocky/octocrab/pull/585))
- Adding test for actions().actions_delete_workflow_run_logs_test() ([#584](https://github.com/XAMPPRocky/octocrab/pull/584))
- Fixing `snafu` version update ([#580](https://github.com/XAMPPRocky/octocrab/pull/580))
- Fixing the test that `wiremock@v0.6.0` breaks. ([#579](https://github.com/XAMPPRocky/octocrab/pull/579))
- Added some more tests ([#578](https://github.com/XAMPPRocky/octocrab/pull/578))
- Added a test ([#577](https://github.com/XAMPPRocky/octocrab/pull/577))

## [0.34.1](https://github.com/XAMPPRocky/octocrab/compare/v0.34.0...v0.34.1) - 2024-02-09

### Other
- Added tests to prove `repos().secrets().delete_secret()` ([#570](https://github.com/XAMPPRocky/octocrab/pull/570))
- Adding test for `issues().delete_comment()` ([#569](https://github.com/XAMPPRocky/octocrab/pull/569))
- Fixed `issues().delete_comment_reaction()` ([#566](https://github.com/XAMPPRocky/octocrab/pull/566))

## [0.34.0](https://github.com/XAMPPRocky/octocrab/compare/v0.33.4...v0.34.0) - 2024-02-07

### Other
- Upgrade http/hyper 1.0 ([#564](https://github.com/XAMPPRocky/octocrab/pull/564))

## [0.33.4](https://github.com/XAMPPRocky/octocrab/compare/v0.33.3...v0.33.4) - 2024-02-06

### Other
- Don't send Authorization headers to third-party services. ([#562](https://github.com/XAMPPRocky/octocrab/pull/562))
- Added tests for `repos().merges()` ([#560](https://github.com/XAMPPRocky/octocrab/pull/560))

## [0.33.3](https://github.com/XAMPPRocky/octocrab/compare/v0.33.2...v0.33.3) - 2024-01-26

### Other
- Handle redirects in ReleasesHandler::stream_asset(). ([#556](https://github.com/XAMPPRocky/octocrab/pull/556))
- Tests for locking and unlocking issue comments ([#557](https://github.com/XAMPPRocky/octocrab/pull/557))

## [0.33.2](https://github.com/XAMPPRocky/octocrab/compare/v0.33.1...v0.33.2) - 2024-01-24

### Other
- Resolving issue with deleting reactions from issues. ([#555](https://github.com/XAMPPRocky/octocrab/pull/555))
- Added a test for `repos().is_collaborator()`. The 204 case is good! ([#553](https://github.com/XAMPPRocky/octocrab/pull/553))

### Other
- Added a test case for `repos().is_collaborator()` to cover both 204 and 404 cases.

## [0.33.1](https://github.com/XAMPPRocky/octocrab/compare/v0.33.0...v0.33.1) - 2024-01-15

### Fixed
- fixed timeout methods not being feature gated ([#519](https://github.com/XAMPPRocky/octocrab/pull/519))

## [0.33.0](https://github.com/XAMPPRocky/octocrab/compare/v0.32.0...v0.33.0) - 2024-01-15

### Added
- add methods to builder for changing timeouts ([#498](https://github.com/XAMPPRocky/octocrab/pull/498))

### Other
- Always set http request header `CONTENT-LENGTH` ([#464](https://github.com/XAMPPRocky/octocrab/pull/464))
- Add link to `Content::decoded_content` ([#514](https://github.com/XAMPPRocky/octocrab/pull/514))
- Repairing two 204-related serde errors, and adding some simple tests. ([#503](https://github.com/XAMPPRocky/octocrab/pull/503))
- Add CheckRunOutput ([#515](https://github.com/XAMPPRocky/octocrab/pull/515))
- cargo clippy --tests --examples --fix ([#496](https://github.com/XAMPPRocky/octocrab/pull/496))
- make dismissal_message optional ([#501](https://github.com/XAMPPRocky/octocrab/pull/501))
- Add conclusion, completed_at and output fields to create check api ([#467](https://github.com/XAMPPRocky/octocrab/pull/467))
- Fix typo in docs ([#508](https://github.com/XAMPPRocky/octocrab/pull/508))
- [docs] Improve the update_file docs ([#513](https://github.com/XAMPPRocky/octocrab/pull/513))
- added repos/list_contributors ([#500](https://github.com/XAMPPRocky/octocrab/pull/500))
- Make user information optional
- Use graphql_client to get stronger typing
- add fields for refresh token and expiry timestamps ([#477](https://github.com/XAMPPRocky/octocrab/pull/477))

### Fixed

- Fixed two functions which return 204s and have no body, which throws serde errors
  - `octocrab.issues().delete_label()`
  - `octocrab.teams().repos().remove()`

### Other

- Added tests for the above functions
- Also added tests for `octocrab.teams().repos().add_or_update()`
- Also added tests for `octocrab.issues().remove_label()`

## [0.32.0](https://github.com/XAMPPRocky/octocrab/compare/v0.31.2...v0.31.3) - 2023-11-01

### Other
- support eq and hash ([#480](https://github.com/XAMPPRocky/octocrab/pull/480))
- Demonstrate variables and paging ([#481](https://github.com/XAMPPRocky/octocrab/pull/481))
- Fix a bunch of clippy issues ([#482](https://github.com/XAMPPRocky/octocrab/pull/482))
- add events for auto_merge_[disabled|enabled] ([#479](https://github.com/XAMPPRocky/octocrab/pull/479))
- Add examples/graphql_issues.rs ([#478](https://github.com/XAMPPRocky/octocrab/pull/478))
- Document the PullRequest number field ([#476](https://github.com/XAMPPRocky/octocrab/pull/476))
- Update jsonwebtoken requirement from 8 to 9 ([#471](https://github.com/XAMPPRocky/octocrab/pull/471))

## [0.31.2](https://github.com/XAMPPRocky/octocrab/compare/v0.31.1...v0.31.2) - 2023-10-15

### Other
- Add `follow-redirect` feature ([#469](https://github.com/XAMPPRocky/octocrab/pull/469))

## [0.31.1](https://github.com/XAMPPRocky/octocrab/compare/v0.31.0...v0.31.1) - 2023-10-08

### Other
- add get_readme for RepoHandler ([#465](https://github.com/XAMPPRocky/octocrab/pull/465))

## [0.31.0](https://github.com/XAMPPRocky/octocrab/compare/v0.30.1...v0.31.0) - 2023-10-02

### Other
- Add WatchEvent ([#462](https://github.com/XAMPPRocky/octocrab/pull/462))
- Checks API ([#461](https://github.com/XAMPPRocky/octocrab/pull/461))
- Fix `github_app_authentication.rs` ([#458](https://github.com/XAMPPRocky/octocrab/pull/458))

## [0.30.1](https://github.com/XAMPPRocky/octocrab/compare/v0.30.0...v0.30.1) - 2023-09-04

### Other
- add tracing debug only when tracing feature is enabled ([#454](https://github.com/XAMPPRocky/octocrab/pull/454))

## [0.30.0](https://github.com/XAMPPRocky/octocrab/compare/v0.29.3...v0.30.0) - 2023-09-01

### Other
- Fix commit_comment webhook event parsing ([#453](https://github.com/XAMPPRocky/octocrab/pull/453))
- Add Octocrab::users() and UsersHandler::repos ([#451](https://github.com/XAMPPRocky/octocrab/pull/451))
- Add CommitHandler::associated_check_runs ([#450](https://github.com/XAMPPRocky/octocrab/pull/450))
- Fix installation token cache issue ([#442](https://github.com/XAMPPRocky/octocrab/pull/442))
- Add projects Api ([#447](https://github.com/XAMPPRocky/octocrab/pull/447))
- Enhance installation webhook events
- Add test for push webhook event
- Implement custom deserialization for hybrid Github API timestamps
- Make webhook_events deserialization tests more robust

## [0.29.3](https://github.com/XAMPPRocky/octocrab/compare/v0.29.2...v0.29.3) - 2023-08-15

### Other
- add `WebhookEventType::Schedule` variant

## [0.29.2](https://github.com/XAMPPRocky/octocrab/compare/v0.29.1...v0.29.2) - 2023-08-14

### Fixed
- fix get_asset url

### Other
- Add optional email field to Author
- Add get_org_installation to AppsRequestHandler
- Update CHANGELOG.md

## [0.29.1](https://github.com/XAMPPRocky/octocrab/compare/v0.29.0...v0.29.1) - 2023-07-31

### Other
- Make models::webhook_events::payload public

## [0.29.0](https://github.com/XAMPPRocky/octocrab/compare/v0.28.0...v0.29.0) - 2023-07-30

### Other
- Add webhook event deserialization ([#427](https://github.com/XAMPPRocky/octocrab/pull/427))
- Update changelog for v0.28.0 ([#428](https://github.com/XAMPPRocky/octocrab/pull/428))
- Add associated pull requests and commit compare functionality  ([#413](https://github.com/XAMPPRocky/octocrab/pull/413))
- Fix clippy 1.71 warnings ([#424](https://github.com/XAMPPRocky/octocrab/pull/424))

## [0.28.0](https://github.com/XAMPPRocky/octocrab/compare/v0.27.0...v0.28.0) - 2023-07-19

### Other

- Handle errors when kicking off github workflows ([#409](https://github.com/XAMPPRocky/octocrab/pull/409))
- Update license field following https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields ([#412](https://github.com/XAMPPRocky/octocrab/pull/416))
- cargo clippy --tests ([#416](https://github.com/XAMPPRocky/octocrab/pull/416))
- Improve workflow job types ([#414](https://github.com/XAMPPRocky/octocrab/pull/416))
- Fix graphql example ([#404](https://github.com/XAMPPRocky/octocrab/pull/404))

## [0.27.0](https://github.com/XAMPPRocky/octocrab/compare/v0.26.0...v0.26.1) - 2023-07-18

### Other
- Handle errors when kicking off github workflows ([#409](https://github.com/XAMPPRocky/octocrab/pull/409))
- Update license field following https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields ([#412](https://github.com/XAMPPRocky/octocrab/pull/412))
- cargo clippy --tests ([#416](https://github.com/XAMPPRocky/octocrab/pull/416))
- Improve workflow job types  ([#414](https://github.com/XAMPPRocky/octocrab/pull/414))
- Fix graphql example ([#404](https://github.com/XAMPPRocky/octocrab/pull/404))

## [0.25.1](https://github.com/XAMPPRocky/octocrab/compare/v0.25.0...v0.25.1) - 2023-06-06

### Other
- Pass through hyper-rustls/webpki-tokio ([#392](https://github.com/XAMPPRocky/octocrab/pull/392))

## [0.25.0](https://github.com/XAMPPRocky/octocrab/compare/v0.24.0...v0.25.0) - 2023-06-03

### Other
- Add User Access Authentication ([#375](https://github.com/XAMPPRocky/octocrab/pull/375))
- Add allow_forking & allow_update_branch in Repository model ([#379](https://github.com/XAMPPRocky/octocrab/pull/379))
- added org secrets api ([#384](https://github.com/XAMPPRocky/octocrab/pull/384))

## [0.24.0](https://github.com/XAMPPRocky/octocrab/compare/v0.23.0...v0.23.1) - 2023-06-02

### Fixed
- the API returns one reviewer not reviewers ([#390](https://github.com/XAMPPRocky/octocrab/pull/390))
- wrap pull_request_review_id in an Option ([#388](https://github.com/XAMPPRocky/octocrab/pull/388))

### Other
- Add Issue Timeline API ([#389](https://github.com/XAMPPRocky/octocrab/pull/389))
- add some of the missing fields to PullRequest ([#386](https://github.com/XAMPPRocky/octocrab/pull/386))
- Builder for list_reviews for pulls ([#387](https://github.com/XAMPPRocky/octocrab/pull/387))
- Link to `gists` documentation in  README ([#383](https://github.com/XAMPPRocky/octocrab/pull/383))

## [0.23.0](https://github.com/XAMPPRocky/octocrab/compare/v0.22.0...v0.22.1) - 2023-05-21

### Other
- Add "updated since" support to ListIssuesBuilder (#373)
- Gists API: Complete support (#371)
- Add more fields (#369)

## [0.22.0](https://github.com/XAMPPRocky/octocrab/compare/v0.21.0...v0.21.1) - 2023-05-16

### Other
- Add leading / to NotificationsHandler.lists() (#364)
- Alter graphql method to pass arbitrarily complex payloads (variables and graphql-client support) (#332)
- Fix authentication endpoints (#354)
- Handle redirects for download_tarball (#359)
- Make building without the `retry` feature work. (#358)
- Add list_org_memberships_for_authenticated_user (#357)
- add Uploader struct for Asset uploader field (#355)

## [0.21.0](https://github.com/XAMPPRocky/octocrab/compare/v0.20.0...v0.21.0) - 2023-04-29

### Other
- Add an example showing gist creation (#329)
- Use CommitAuthor for Commit.author (#353)
- Create release-plz.toml
- Sort deps in cargo.toml (#352)
- Enable rustls(and use as default client) (#351)
- *(ci)* update release-plz version (#350)
- Add missing pub to struct ListCheckRuns 😅 (#347)
- Add Checks API skeleton (#345)
- cargo fmt (#343)
- Remove reqwest (#342)

## [0.20.0-alpha.3](https://github.com/XAMPPRocky/octocrab/compare/v0.20.0-alpha.2...v0.20.0-alpha.3) - 2023-04-12

### Other
- Handle `DELETE /gists/{gist_id}` (#333)

## [0.20.0-alpha.2](https://github.com/XAMPPRocky/octocrab/compare/v0.20.0-alpha.1...v0.20.0-alpha.2) - 2023-04-10

### Other
- Extend `GistsHandler` through `star(...)`, `unstar(...)`, `is_starred(...)` (#330)
- added poll org events (#325)
- Add `CurrentAuthHandler::list_gists_for_authenticated_user` (#328)
- Fix typo in POST /gists endpoint (#327)
- Update hyper-rustls requirement from 0.23.2 to 0.24.0 (#324)
- Percent encode label name in remove_label to avoid InvalidUri(InvalidUriChar) error (#323)

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
