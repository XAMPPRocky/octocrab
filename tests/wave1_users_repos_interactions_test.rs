#![recursion_limit = "256"]
use octocrab::models::interaction_limits::PullRequestCreationCap;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_user_json() -> serde_json::Value {
    json!({
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
    })
}

fn sample_repo_json() -> serde_json::Value {
    json!({
        "id": 1296269,
        "node_id": "MDEwOlJlcG9zaXRvcnkxMjk2MjY5",
        "name": "Hello-World",
        "full_name": "octocat/Hello-World",
        "owner": sample_user_json(),
        "private": false,
        "html_url": "https://github.com/octocat/Hello-World",
        "description": "This your first repo!",
        "fork": false,
        "url": "https://api.github.com/repos/octocat/Hello-World",
        "archive_url": "https://api.github.com/repos/octocat/Hello-World/{archive_format}{/ref}",
        "assignees_url": "https://api.github.com/repos/octocat/Hello-World/assignees{/user}",
        "blobs_url": "https://api.github.com/repos/octocat/Hello-World/git/blobs{/sha}",
        "branches_url": "https://api.github.com/repos/octocat/Hello-World/branches{/branch}",
        "collaborators_url": "https://api.github.com/repos/octocat/Hello-World/collaborators{/collaborator}",
        "comments_url": "https://api.github.com/repos/octocat/Hello-World/comments{/number}",
        "commits_url": "https://api.github.com/repos/octocat/Hello-World/commits{/sha}",
        "compare_url": "https://api.github.com/repos/octocat/Hello-World/compare/{base}...{head}",
        "contents_url": "https://api.github.com/repos/octocat/Hello-World/contents/{+path}",
        "contributors_url": "https://api.github.com/repos/octocat/Hello-World/contributors",
        "deployments_url": "https://api.github.com/repos/octocat/Hello-World/deployments",
        "downloads_url": "https://api.github.com/repos/octocat/Hello-World/downloads",
        "events_url": "https://api.github.com/repos/octocat/Hello-World/events",
        "forks_url": "https://api.github.com/repos/octocat/Hello-World/forks",
        "git_commits_url": "https://api.github.com/repos/octocat/Hello-World/git/commits{/sha}",
        "git_refs_url": "https://api.github.com/repos/octocat/Hello-World/git/refs{/sha}",
        "git_tags_url": "https://api.github.com/repos/octocat/Hello-World/git/tags{/sha}",
        "git_url": "git:github.com/octocat/Hello-World.git",
        "issue_comment_url": "https://api.github.com/repos/octocat/Hello-World/issues/comments{/number}",
        "issue_events_url": "https://api.github.com/repos/octocat/Hello-World/issues/events{/number}",
        "issues_url": "https://api.github.com/repos/octocat/Hello-World/issues{/number}",
        "keys_url": "https://api.github.com/repos/octocat/Hello-World/keys{/key_id}",
        "labels_url": "https://api.github.com/repos/octocat/Hello-World/labels{/name}",
        "languages_url": "https://api.github.com/repos/octocat/Hello-World/languages",
        "merges_url": "https://api.github.com/repos/octocat/Hello-World/merges",
        "milestones_url": "https://api.github.com/repos/octocat/Hello-World/milestones{/number}",
        "notifications_url": "https://api.github.com/repos/octocat/Hello-World/notifications{?since,all,participating}",
        "pulls_url": "https://api.github.com/repos/octocat/Hello-World/pulls{/number}",
        "releases_url": "https://api.github.com/repos/octocat/Hello-World/releases{/id}",
        "ssh_url": "git@github.com:octocat/Hello-World.git",
        "stargazers_url": "https://api.github.com/repos/octocat/Hello-World/stargazers",
        "statuses_url": "https://api.github.com/repos/octocat/Hello-World/statuses/{sha}",
        "subscribers_url": "https://api.github.com/repos/octocat/Hello-World/subscribers",
        "subscription_url": "https://api.github.com/repos/octocat/Hello-World/subscription",
        "tags_url": "https://api.github.com/repos/octocat/Hello-World/tags",
        "teams_url": "https://api.github.com/repos/octocat/Hello-World/teams",
        "trees_url": "https://api.github.com/repos/octocat/Hello-World/git/trees{/sha}",
        "clone_url": "https://github.com/octocat/Hello-World.git",
        "mirror_url": "git:git.example.com/octocat/Hello-World",
        "hooks_url": "https://api.github.com/repos/octocat/Hello-World/hooks",
        "svn_url": "https://svn.github.com/octocat/Hello-World",
        "homepage": "https://github.com",
        "language": null,
        "forks_count": 9,
        "stargazers_count": 80,
        "watchers_count": 80,
        "size": 108,
        "default_branch": "master",
        "open_issues_count": 0,
        "is_template": false,
        "topics": ["octocat", "atom", "electron", "api"],
        "has_issues": true,
        "has_projects": true,
        "has_wiki": true,
        "has_pages": false,
        "has_downloads": true,
        "archived": false,
        "disabled": false,
        "visibility": "public",
        "pushed_at": "2011-01-26T19:06:43Z",
        "created_at": "2011-01-26T19:01:12Z",
        "updated_at": "2011-01-26T19:14:43Z",
        "permissions": {
            "admin": false,
            "push": false,
            "pull": true
        }
    })
}

#[tokio::test]
async fn test_users_endpoints() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. List users
    Mock::given(method("GET"))
        .and(path("/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_user_json()])))
        .mount(&mock_server)
        .await;

    let users = octocrab
        .all_users()
        .since(1u64)
        .per_page(10)
        .send()
        .await
        .unwrap();
    assert_eq!(users.items.len(), 1);

    // 2. Check if user follows another user
    Mock::given(method("GET"))
        .and(path("/users/octocat/following/hubot"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let is_following = octocrab
        .users("octocat")
        .is_following("hubot")
        .await
        .unwrap();
    assert!(is_following);

    // 3. List public SSH keys for a user
    Mock::given(method("GET"))
        .and(path("/users/octocat/keys"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "id": 1, "key": "ssh-rsa AAAAB3NzaC1yc2EAAA..." }
        ])))
        .mount(&mock_server)
        .await;

    let keys = octocrab
        .users("octocat")
        .list_user_git_ssh_keys()
        .send()
        .await
        .unwrap();
    assert_eq!(keys.items.len(), 1);

    // 4. List social accounts for a user
    Mock::given(method("GET"))
        .and(path("/users/octocat/social_accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "provider": "youtube", "url": "https://youtube.com/@github" }
        ])))
        .mount(&mock_server)
        .await;

    let accounts = octocrab
        .users("octocat")
        .list_user_social_accounts()
        .send()
        .await
        .unwrap();
    assert_eq!(accounts.items.len(), 1);

    // 5. List SSH signing keys for a user
    Mock::given(method("GET"))
        .and(path("/users/octocat/ssh_signing_keys"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 1,
                "key": "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI...",
                "title": "Personal Key",
                "created_at": "2023-01-01T00:00:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let signing_keys = octocrab
        .users("octocat")
        .list_user_ssh_signing_keys()
        .send()
        .await
        .unwrap();
    assert_eq!(signing_keys.items.len(), 1);

    // 6. User attestations list and delete
    Mock::given(method("GET"))
        .and(path("/users/octocat/attestations/sha256:abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "attestations": [{
                "bundle_url": "https://example.com/attestation.json"
            }]
        })))
        .mount(&mock_server)
        .await;

    let attestations = octocrab
        .users("octocat")
        .attestations()
        .list("sha256:abc")
        .send()
        .await
        .unwrap();
    assert_eq!(attestations.attestations.len(), 1);

    Mock::given(method("DELETE"))
        .and(path("/users/octocat/attestations/digest/sha256:abc"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .users("octocat")
        .attestations()
        .delete_by_digest("sha256:abc")
        .await
        .is_ok());
}

#[tokio::test]
async fn test_repositories_endpoints() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Repo attestations create and list
    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/attestations"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 42 })))
        .mount(&mock_server)
        .await;

    let created_attestation = octocrab
        .repos("owner", "repo")
        .attestations()
        .create(json!({ "sigstore": "bundle" }))
        .await
        .unwrap();
    assert_eq!(created_attestation.id, 42);

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/attestations/sha256:abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "attestations": [{
                "bundle_url": "https://example.com/bundle.json"
            }]
        })))
        .mount(&mock_server)
        .await;

    let repo_attestations = octocrab
        .repos("owner", "repo")
        .attestations()
        .list("sha256:abc")
        .send()
        .await
        .unwrap();
    assert_eq!(repo_attestations.attestations.len(), 1);

    // 2. Issue types
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issue-types"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 410,
                "node_id": "IT_1",
                "name": "Task"
            }
        ])))
        .mount(&mock_server)
        .await;

    let issue_types = octocrab.repos("owner", "repo").issue_types().await.unwrap();
    assert_eq!(issue_types.len(), 1);
    assert_eq!(issue_types[0].name, "Task");

    // 3. Create org repo
    Mock::given(method("POST"))
        .and(path("/orgs/my-org/repos"))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_repo_json()))
        .mount(&mock_server)
        .await;

    let org_repo = octocrab
        .orgs("my-org")
        .create_repo("new-repo")
        .private(true)
        .send()
        .await
        .unwrap();
    assert_eq!(org_repo.name, "Hello-World");

    // 4. Create user repo
    Mock::given(method("POST"))
        .and(path("/user/repos"))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_repo_json()))
        .mount(&mock_server)
        .await;

    let user_repo = octocrab
        .current()
        .create_repo("my-repo")
        .send()
        .await
        .unwrap();
    assert_eq!(user_repo.name, "Hello-World");

    // 5. Update repo
    Mock::given(method("PATCH"))
        .and(path("/repos/owner/repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_repo_json()))
        .mount(&mock_server)
        .await;

    let updated_repo = octocrab
        .repos("owner", "repo")
        .update()
        .description("Updated description")
        .send()
        .await
        .unwrap();
    assert_eq!(updated_repo.name, "Hello-World");

    // 6. Hash algorithm
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/hash-algorithm"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "hash_algorithm": "sha256" })),
        )
        .mount(&mock_server)
        .await;

    let hash_algo = octocrab
        .repos("owner", "repo")
        .hash_algorithm()
        .await
        .unwrap();
    assert_eq!(hash_algo.hash_algorithm, "sha256");

    // 7. Immutable releases (check, enable, disable)
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/immutable-releases"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "enabled": true })))
        .mount(&mock_server)
        .await;

    let immut = octocrab
        .repos("owner", "repo")
        .immutable_releases()
        .await
        .unwrap();
    assert!(immut.enabled);

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/immutable-releases"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .repos("owner", "repo")
        .enable_immutable_releases()
        .await
        .is_ok());

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/immutable-releases"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .repos("owner", "repo")
        .disable_immutable_releases()
        .await
        .is_ok());

    // 8. Ruleset history and version
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/rulesets/1/history"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "version_id": 1,
                "updated_at": "2024-01-01T00:00:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let history = octocrab
        .repos("owner", "repo")
        .rulesets()
        .history(1u64)
        .await
        .unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].version_id, 1);

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/rulesets/1/history/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "version_id": 1,
            "updated_at": "2024-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let version = octocrab
        .repos("owner", "repo")
        .rulesets()
        .version(1u64, 1)
        .await
        .unwrap();
    assert_eq!(version.version_id, 1);
}

#[tokio::test]
async fn test_interactions_endpoints() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Org PR creation cap
    Mock::given(method("GET"))
        .and(path("/orgs/my-org/interaction-limits/pulls/creation-cap"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "enabled": true,
            "max_open_pull_requests": 5
        })))
        .mount(&mock_server)
        .await;

    let cap = octocrab
        .orgs("my-org")
        .pull_request_creation_cap()
        .await
        .unwrap();
    assert!(cap.enabled);
    assert_eq!(cap.max_open_pull_requests, Some(5));

    Mock::given(method("PATCH"))
        .and(path("/orgs/my-org/interaction-limits/pulls/creation-cap"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "enabled": false,
            "max_open_pull_requests": 0
        })))
        .mount(&mock_server)
        .await;

    let updated_cap = octocrab
        .orgs("my-org")
        .update_pull_request_creation_cap(&{
            let mut cap = PullRequestCreationCap::new(false);
            cap.max_open_pull_requests = Some(0);
            cap
        })
        .await
        .unwrap();
    assert!(!updated_cap.enabled);

    // 2. Repo PR creation cap
    Mock::given(method("GET"))
        .and(path(
            "/repos/owner/repo/interaction-limits/pulls/creation-cap",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "enabled": true,
            "max_open_pull_requests": 3
        })))
        .mount(&mock_server)
        .await;

    let repo_cap = octocrab
        .repos("owner", "repo")
        .pull_request_creation_cap()
        .await
        .unwrap();
    assert!(repo_cap.enabled);

    // 3. Repo PR creation cap bypass list
    Mock::given(method("GET"))
        .and(path(
            "/repos/owner/repo/interaction-limits/pulls/bypass-list",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_user_json()])))
        .mount(&mock_server)
        .await;

    let bypass_users = octocrab
        .repos("owner", "repo")
        .pull_request_creation_cap_bypass_list()
        .await
        .unwrap();
    assert_eq!(bypass_users.len(), 1);

    Mock::given(method("PUT"))
        .and(path(
            "/repos/owner/repo/interaction-limits/pulls/bypass-list",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .repos("owner", "repo")
        .add_users_to_pull_request_creation_cap_bypass_list(&["octocat"])
        .await
        .is_ok());

    Mock::given(method("DELETE"))
        .and(path(
            "/repos/owner/repo/interaction-limits/pulls/bypass-list",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .repos("owner", "repo")
        .remove_users_from_pull_request_creation_cap_bypass_list(&["octocat"])
        .await
        .is_ok());
}
