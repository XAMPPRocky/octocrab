use octocrab::{models::workflows::ReviewDeploymentState, Octocrab};
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_workflow_lifecycle_and_timing() {
    let mock_server = MockServer::start().await;
    let workflow_json = json!({
        "id": 161335,
        "node_id": "MDg6V29ya2Zsb3cxNjEzMzU=",
        "name": "CI",
        "path": ".github/workflows/ci.yml",
        "state": "active",
        "created_at": "2020-01-08T23:48:37.000-08:00",
        "updated_at": "2020-01-08T23:50:21.000-08:00",
        "url": "https://api.github.com/repos/owner/repo/actions/workflows/161335",
        "html_url": "https://github.com/owner/repo/blob/master/.github/workflows/ci.yml",
        "badge_url": "https://github.com/owner/repo/workflows/CI/badge.svg"
    });
    let timing_json = json!({
        "billable": {
            "UBUNTU": {
                "total_ms": 180000
            }
        }
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/workflows/ci.yml"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&workflow_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/actions/workflows/ci.yml/enable"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/actions/workflows/ci.yml/disable"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/workflows/ci.yml/timing"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&timing_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let workflows = client.workflows("owner", "repo");

    let wf = workflows.get_workflow("ci.yml").await.unwrap();
    assert_eq!(wf.name, "CI");

    assert!(workflows.enable_workflow("ci.yml").await.is_ok());
    assert!(workflows.disable_workflow("ci.yml").await.is_ok());

    let timing = workflows.get_timing("ci.yml").await.unwrap();
    assert_eq!(timing.billable.ubuntu.unwrap().total_ms, 180000);
}

#[tokio::test]
async fn test_workflow_runs_and_approvals() {
    let mock_server = MockServer::start().await;

    let run_timing_json = json!({
        "billable": {
            "UBUNTU": {
                "total_ms": 60000,
                "jobs": 1,
                "job_runs": [
                    {
                        "job_id": 101,
                        "duration_ms": 60000
                    }
                ]
            }
        },
        "run_duration_ms": 65000
    });

    let approvals_json = json!([
        {
            "environments": [
                {
                    "id": 1,
                    "node_id": "MDg6RW52aXJvbm1lbnQx",
                    "name": "production",
                    "url": "https://api.github.com/repos/owner/repo/environments/production",
                    "html_url": "https://github.com/owner/repo/deployments/activity_log?environments=production"
                }
            ],
            "state": "approved",
            "user": {
                "login": "octocat",
                "id": 1,
                "node_id": "MDQ6VXNlcjE=",
                "avatar_url": "https://github.com/images/error/octocat_happy.gif",
                "gravatar_id": "",
                "url": "https://api.github.com/users/octocat",
                "html_url": "https://github.com/octocat",
                "followers_url": "https://api.github.com/users/octocat/followers",
                "following_url": "https://api.github.com/users/octocat/following{/other_user}",
                "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
                "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
                "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
                "organizations_url": "https://api.github.com/users/octocat/orgs",
                "repos_url": "https://api.github.com/users/octocat/repos",
                "events_url": "https://api.github.com/users/octocat/events{/privacy}",
                "received_events_url": "https://api.github.com/users/octocat/received_events",
                "type": "User",
                "site_admin": false
            },
            "comment": "Ship it!"
        }
    ]);

    let pending_json = json!([
        {
            "environment": {
                "id": 1,
                "node_id": "MDg6RW52aXJvbm1lbnQx",
                "name": "production",
                "url": "https://api.github.com/repos/owner/repo/environments/production",
                "html_url": "https://github.com/owner/repo/deployments/activity_log?environments=production"
            },
            "wait_timer": 0,
            "current_user_can_approve": true,
            "reviewers": []
        }
    ]);

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/actions/runs/42/force-cancel"))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/actions/runs/42/rerun"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/actions/runs/42/rerun-failed-jobs"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/runs/42/timing"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&run_timing_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/runs/42/approvals"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&approvals_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/actions/runs/42/approve"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path(
            "/repos/owner/repo/actions/runs/42/pending_deployments",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&pending_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path(
            "/repos/owner/repo/actions/runs/42/deployment_protection_rule",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let workflows = client.workflows("owner", "repo");

    assert!(workflows.force_cancel_run(42u64.into()).await.is_ok());
    assert!(workflows.rerun_run(42u64.into()).await.is_ok());
    assert!(workflows.rerun_failed_jobs(42u64.into()).await.is_ok());

    let run_timing = workflows.get_run_timing(42u64.into()).await.unwrap();
    assert_eq!(run_timing.run_duration_ms, Some(65000));

    let approvals = workflows.get_run_approvals(42u64.into()).await.unwrap();
    assert_eq!(approvals.len(), 1);
    assert_eq!(approvals[0].comment, "Ship it!");

    assert!(workflows.approve_run(42u64.into()).await.is_ok());

    let pending = workflows
        .get_run_pending_deployments(42u64.into())
        .await
        .unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].environment.name, "production");

    assert!(workflows
        .review_custom_deployment_protection_rule(
            42u64.into(),
            "production",
            ReviewDeploymentState::Approved,
            "LGTM"
        )
        .await
        .is_ok());
}

#[tokio::test]
async fn test_jobs_and_attempts() {
    let mock_server = MockServer::start().await;
    let job_json = json!({
        "id": 500,
        "run_id": 42,
        "workflow_name": "Build CI",
        "head_branch": "main",
        "run_attempt": 1,
        "run_url": "https://api.github.com/repos/owner/repo/actions/runs/42",

        "node_id": "MDg6V29ya2Zsb3dKb2I1MDA=",
        "head_sha": "009b8a3a9ccbb128af87f9b1c0f4c62e8a304f6d",
        "url": "https://api.github.com/repos/owner/repo/actions/jobs/500",
        "html_url": "https://github.com/owner/repo/runs/500",
        "status": "completed",
        "conclusion": "success",
        "created_at": "2020-01-08T22:40:00Z",
        "started_at": "2020-01-08T22:40:01Z",
        "completed_at": "2020-01-08T22:42:00Z",
        "name": "build",
        "steps": [],
        "check_run_url": "https://api.github.com/repos/owner/repo/check-runs/500",
        "labels": ["ubuntu-latest"]
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/jobs/500"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&job_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/actions/jobs/500/rerun"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let workflows = client.workflows("owner", "repo");

    let job = workflows.get_job(500u64.into()).await.unwrap();
    assert_eq!(job.name, "build");

    assert!(workflows.rerun_job(500u64.into()).await.is_ok());
}
