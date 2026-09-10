use http::StatusCode;
use wiremock::{
    matchers::{body_json, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::{models::commits::CommitComparison, Octocrab};

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const COMMIT_SHA: &str = "c5b97d5ae6c19d5c5df71a34c7fbeeda2479ccbc";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_branch_json(name: &str, sha: &str, protected: bool) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "commit": {
            "sha": sha,
            "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/commits/{sha}")
        },
        "protected": protected
    })
}

// ---------------------------------------------------------------------------
// Merge Upstream
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_merge_upstream_via_repo_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let response_json = serde_json::json!({
        "message": "Successfully fetched and fast-forwarded from upstream upstream:main.",
        "merge_type": "fast-forward",
        "base_branch": "upstream:main"
    });

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/merge-upstream")))
        .and(body_json(serde_json::json!({ "branch": "main" })))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("POST on /repos/{OWNER}/{REPO}/merge-upstream was not received"),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .merge_upstream("main")
        .await
        .unwrap();

    assert_eq!(
        result.message.as_deref(),
        Some("Successfully fetched and fast-forwarded from upstream upstream:main.")
    );
    assert_eq!(result.merge_type.as_deref(), Some("fast-forward"));
    assert_eq!(result.base_branch.as_deref(), Some("upstream:main"));
}

#[tokio::test]
async fn should_merge_upstream_via_branches_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let response_json = serde_json::json!({
        "message": "Successfully fetched and fast-forwarded from upstream upstream:main.",
        "merge_type": "fast-forward",
        "base_branch": "upstream:main"
    });

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/merge-upstream")))
        .and(body_json(serde_json::json!({ "branch": "main" })))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("POST on /repos/{OWNER}/{REPO}/merge-upstream was not received"),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .branches()
        .merge_upstream("main")
        .await
        .unwrap();

    assert_eq!(result.merge_type.as_deref(), Some("fast-forward"));
}

#[tokio::test]
async fn should_handle_merge_upstream_conflict() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/merge-upstream")))
        .and(body_json(serde_json::json!({ "branch": "feature" })))
        .respond_with(
            ResponseTemplate::new(StatusCode::CONFLICT).set_body_json(
                serde_json::json!({ "message": "Branch could not be fast-forwarded" }),
            ),
        )
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("POST on /repos/{OWNER}/{REPO}/merge-upstream was not received"),
    )
    .await;

    let result = octocrab.repos(OWNER, REPO).merge_upstream("feature").await;

    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Branches Where Head
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_list_branches_where_head_via_commit_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let branch_json = sample_branch_json("main", COMMIT_SHA, true);

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/commits/{COMMIT_SHA}/branches-where-head"
        )))
        .respond_with(
            ResponseTemplate::new(StatusCode::OK).set_body_json(serde_json::json!([branch_json])),
        )
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!(
            "GET on /repos/{OWNER}/{REPO}/commits/{COMMIT_SHA}/branches-where-head was not received"
        ),
    )
    .await;

    let branches = octocrab
        .commits(OWNER, REPO)
        .branches_where_head(COMMIT_SHA)
        .await
        .unwrap();

    assert_eq!(branches.len(), 1);
    assert_eq!(branches[0].name, "main");
    assert_eq!(branches[0].commit.sha, COMMIT_SHA);
    assert!(branches[0].protected);
}

#[tokio::test]
async fn should_list_branches_where_head_via_repo_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let branch_json = sample_branch_json("feature", COMMIT_SHA, false);

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/commits/{COMMIT_SHA}/branches-where-head"
        )))
        .respond_with(
            ResponseTemplate::new(StatusCode::OK).set_body_json(serde_json::json!([branch_json])),
        )
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!(
            "GET on /repos/{OWNER}/{REPO}/commits/{COMMIT_SHA}/branches-where-head was not received"
        ),
    )
    .await;

    let branches = octocrab
        .repos(OWNER, REPO)
        .branches_where_head(COMMIT_SHA)
        .await
        .unwrap();

    assert_eq!(branches.len(), 1);
    assert_eq!(branches[0].name, "feature");
    assert!(!branches[0].protected);
}

#[tokio::test]
async fn should_list_branches_where_head_via_branches_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let branch_json = sample_branch_json("dev", COMMIT_SHA, false);

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/commits/{COMMIT_SHA}/branches-where-head"
        )))
        .respond_with(
            ResponseTemplate::new(StatusCode::OK).set_body_json(serde_json::json!([branch_json])),
        )
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!(
            "GET on /repos/{OWNER}/{REPO}/commits/{COMMIT_SHA}/branches-where-head was not received"
        ),
    )
    .await;

    let branches = octocrab
        .repos(OWNER, REPO)
        .branches()
        .where_head(COMMIT_SHA)
        .await
        .unwrap();

    assert_eq!(branches.len(), 1);
    assert_eq!(branches[0].name, "dev");
}

// ---------------------------------------------------------------------------
// Compare Commits
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_compare_commits_via_repo_handler() {
    let comparison_fixture: CommitComparison =
        serde_json::from_str(include_str!("resources/repo_compare_commits.json")).unwrap();

    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let base = "ad64898819efb83f3e2920cb3c1affccb6ff24cb";
    let head = "main";

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/compare/{base}...{head}"
        )))
        .and(query_param("per_page", "50"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&comparison_fixture))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/compare/{base}...{head} was not received"),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .compare_commits(base, head)
        .per_page(50)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(result.base_commit.sha, base);
}

#[tokio::test]
async fn should_compare_commits_range_via_repo_handler() {
    let comparison_fixture: CommitComparison =
        serde_json::from_str(include_str!("resources/repo_compare_commits.json")).unwrap();

    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let range = "base..head";

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/compare/{range}")))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&comparison_fixture))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/compare/{range} was not received"),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .compare_commits_range(range)
        .send()
        .await
        .unwrap();

    assert_eq!(
        result.base_commit.sha,
        "ad64898819efb83f3e2920cb3c1affccb6ff24cb"
    );
}

#[tokio::test]
async fn should_compare_range_via_commit_handler() {
    let comparison_fixture: CommitComparison =
        serde_json::from_str(include_str!("resources/repo_compare_commits.json")).unwrap();

    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let range = "base..head";

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/compare/{range}")))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&comparison_fixture))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/compare/{range} was not received"),
    )
    .await;

    let result = octocrab
        .commits(OWNER, REPO)
        .compare_range(range)
        .send()
        .await
        .unwrap();

    assert_eq!(
        result.base_commit.sha,
        "ad64898819efb83f3e2920cb3c1affccb6ff24cb"
    );
}
