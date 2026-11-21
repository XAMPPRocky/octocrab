# Migrating `octocrab` from `chrono` to `jiff` (Issue #1004)

`RUSTSEC-2026-0543` marks `chrono` as unmaintained and recommends migrating to [`jiff`](https://crates.io/crates/jiff). Starting in `0.55.0`, `octocrab` replaces `chrono` with `jiff = "1.0"`, removing `chrono` completely from the dependency graph.

## Design Decisions

- **Minimal `jiff` feature set**: `jiff = { version = "1.0", default-features = false, features = ["std", "serde", "js"] }`.
  - Matches `octocrab`'s MSRV (`1.85.0` — `jiff 1.0.1` requires Rust `1.85`).
  - `js` is target-gated inside `jiff` to `wasm32`/`wasm64` + `target_os = "unknown"`, keeping `jiff::Timestamp::now()` working in browser WASM builds (matching `chrono`'s `wasmbind` feature) with zero extra dependencies on native targets.
  - Disabling default features avoids pulling timezone database crates (`jiff-tzdb-platform`, `windows-link`) because `octocrab` only needs UTC instants (`jiff::Timestamp`), calendar dates (`jiff::civil::Date`), and signed physical durations (`jiff::SignedDuration`). Downstream crates needing IANA time zones can enable `jiff`'s default features via Cargo feature unification.
- **`Octocrab::installation_token_with_buffer` takes `jiff::SignedDuration`**: Direct 1:1 semantic replacement for `chrono::Duration` (Option A).
- **No `pub use jiff;` re-export**: Matches prior `chrono` convention; downstream callers declare `jiff = "1"` in their own `Cargo.toml`.
- **No `chrono` compatibility feature flag**: Cargo features must be additive, so mutually exclusive `chrono`/`jiff` features break workspaces where two crates enable different ones; keeping `chrono` on by default fails to clear `RUSTSEC-2026-0543`; and supporting both simultaneously would duplicate ~70+ files of public models and builders.
- **`web-time` kept untouched**: `web-time` is actively maintained with no advisory. Its `web_time::Duration` is a re-export of `std::time::Duration`, which is required directly by `tokio::time::sleep`, `hyper-timeout`, and `tower-http` (`src/lib.rs` timeouts/tracing callbacks and `src/auth.rs` device-flow polling). `jiff::SignedDuration` is a separate type and cannot replace `std::time::Duration` in those APIs. Only `src/auth.rs::create_jwt` uses `web_time::SystemTime`, so migrating it would not remove `web-time`.
- **Release `0.55.0` via `release-plz`**: `.github/workflows/release-plz.yml` + `release-plz.toml` (`semver_check = false`) automatically bumps `0.54.9` -> `0.55.0` when merged with a breaking conventional commit (`feat!: replace chrono with jiff (#1004)` or a `BREAKING CHANGE:` footer).

## Type & API Mapping

| `chrono` (<= 0.54.x) | `jiff` (>= 0.55.0) | Notes |
| --- | --- | --- |
| `chrono::DateTime<chrono::Utc>` | `jiff::Timestamp` | Same `Copy`, `Default`, `Ord`, `Hash`, `FromStr`, and RFC 3339 serde behavior. |
| `chrono::NaiveDate` | `jiff::civil::Date` | Used in `models::orgs_copilot::metrics::{CopilotMetrics, Model}` (`YYYY-MM-DD`). |
| `chrono::Duration` | `jiff::SignedDuration` | Used in `Octocrab::installation_token_with_buffer`. |
| `chrono::Utc::now()` | `jiff::Timestamp::now()` | Works on native (`std`) and browser WASM (`js`). |
| `chrono::DateTime::from_timestamp(ts, 0)` | `jiff::Timestamp::from_second(ts).ok()` | Used by `GitHubError::rate_limit_reset_at()`. |
| `chrono::DateTime::parse_from_rfc3339(s)?.into()` | `s.parse::<jiff::Timestamp>()?` | Parses RFC 3339 strings with `Z` or `±HH:MM` offsets directly. |
| `dt.to_rfc3339()` | `ts.to_string()` | Formats UTC as `...Z` (instead of `chrono`'s `...+00:00`). |
| `Utc::now() - chrono::Duration::days(1)` | `jiff::Timestamp::now() - jiff::SignedDuration::from_hours(24)` | See pitfall below. |

> **Pitfall — Do not subtract `jiff::Span` with days from `Timestamp`**: `Timestamp` represents an instant without a time zone, so adding/subtracting calendar units larger than hours (such as `1.day()`) panics at runtime. Use `jiff::SignedDuration::from_hours(24)` (or `jiff::Zoned` if calendar-day DST math is desired).

## Migration Examples

### `Cargo.toml`

```diff
-chrono = "0.4"
+jiff = "1.0"
```

### Filtering with `.since(...)`

```rust
// Before
let yesterday = chrono::Utc::now() - chrono::Duration::days(1);
let issues = octocrab.issues("owner", "repo").list().since(yesterday).send().await?;

// After
let yesterday = jiff::Timestamp::now() - jiff::SignedDuration::from_hours(24);
let issues = octocrab.issues("owner", "repo").list().since(yesterday).send().await?;
```

### `Octocrab::installation_token_with_buffer`

```rust
// Before
let token = octocrab
    .installation_token_with_buffer(chrono::Duration::seconds(60))
    .await?;

// After
let token = octocrab
    .installation_token_with_buffer(jiff::SignedDuration::from_secs(60))
    .await?;
```

### `GitHubError::rate_limit_reset_at`

```rust
// Returns Option<jiff::Timestamp>
if let Some(reset_at) = github_err.rate_limit_reset_at() {
    println!("Rate limit resets at {reset_at}");
}
```

### `CopilotMetrics::date`

```rust
// Before: chrono::NaiveDate
// After: jiff::civil::Date
println!("Metrics for {}", metrics.date);
```
