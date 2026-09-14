mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::git::CreateTreeEntry;
use octocrab::models::RepositoryId;
use octocrab::params::repos::Reference;
use octocrab::Octocrab;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const OWNER: &str = "owner";
const REPO: &str = "repo";
const BLOB_SHA: &str = "3a0f86fb8db8eea7ccbb9a95f325ddbedfb25e15";
const COMMIT_SHA: &str = "7638417db6d59f3c431d3e1f261cc637155684cd";
const TAG_SHA: &str = "940bd336248efae0f9ee5bc7b2d5c985887b16ac";
const TREE_SHA: &str = "9fb037999f264ba9a7fc6274d15fa3ae2ab98312";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_blob_json() -> serde_json::Value {
    serde_json::json!({
        "sha": BLOB_SHA,
        "node_id": "MDQ6QmxvYjM0NTEzNDQ=",
        "size": 12,
        "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/blobs/{BLOB_SHA}"),
        "content": "SGVsbG8gV29ybGQ=\n",
        "encoding": "base64"
    })
}

fn sample_commit_json() -> serde_json::Value {
    serde_json::json!({
        "sha": COMMIT_SHA,
        "node_id": "MDY6Q29tbWl0NmRjYjA5YjViNTc4NzA4MjM0ZjFjOGM4ODI1Zjc5ZTlmOTMwNTg2OQ==",
        "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/commits/{COMMIT_SHA}"),
        "author": {
            "date": "2014-11-07T22:01:45Z",
            "name": "Monalisa Octocat",
            "email": "octocat@github.com"
        },
        "committer": {
            "date": "2014-11-07T22:01:45Z",
            "name": "Monalisa Octocat",
            "email": "octocat@github.com"
        },
        "message": "commit message",
        "tree": {
            "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/trees/691272480426f78a0138979dd3ce63b77f706feb"),
            "sha": "691272480426f78a0138979dd3ce63b77f706feb"
        },
        "parents": [
            {
                "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/commits/1acc419d4d6a9ce985db7be48c6349a0475975b5"),
                "sha": "1acc419d4d6a9ce985db7be48c6349a0475975b5"
            }
        ],
        "verification": {
            "verified": false,
            "reason": "unsigned",
            "signature": null,
            "payload": null
        },
        "html_url": format!("https://github.com/{OWNER}/{REPO}/commit/{COMMIT_SHA}")
    })
}

fn sample_ref_json(ref_name: &str) -> serde_json::Value {
    serde_json::json!({
        "ref": ref_name,
        "node_id": "MDM6UmVmZmVhdHVyZS1h",
        "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/{ref_name}"),
        "object": {
            "sha": "aa218f56b14c9653891f9e74264a383fa43fefbd",
            "type": "commit",
            "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/commits/aa218f56b14c9653891f9e74264a383fa43fefbd")
        }
    })
}

fn sample_tag_json() -> serde_json::Value {
    serde_json::json!({
        "node_id": "MDM6VGFnOTQwYmQzMzYyNDhlZmFlMGY5ZWU1YmM3YjJkNWM5ODU4ODdiMTZhYw==",
        "tag": "v0.0.1",
        "sha": TAG_SHA,
        "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/tags/{TAG_SHA}"),
        "message": "initial version",
        "tagger": {
            "name": "Monalisa Octocat",
            "email": "octocat@github.com",
            "date": "2014-11-07T22:01:45Z"
        },
        "object": {
            "sha": "c3d0be41ecbe669545ee3e94d31ed9a4bc91ee3c",
            "type": "commit",
            "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/commits/c3d0be41ecbe669545ee3e94d31ed9a4bc91ee3c")
        },
        "verification": {
            "verified": false,
            "reason": "unsigned",
            "signature": null,
            "payload": null
        }
    })
}

fn sample_tree_json() -> serde_json::Value {
    serde_json::json!({
        "sha": TREE_SHA,
        "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/trees/{TREE_SHA}"),
        "tree": [
            {
                "path": "file.rb",
                "mode": "100644",
                "type": "blob",
                "size": 30,
                "sha": "44b4fc6d56897b048c772eb4087f854f46256132",
                "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/blobs/44b4fc6d56897b048c772eb4087f854f46256132")
            }
        ],
        "truncated": false
    })
}

// ---------------------------------------------------------------------------
// Blob tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_get_blob_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/blobs/{BLOB_SHA}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_blob_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on git blobs failed").await;

    let blob = octocrab.git(OWNER, REPO).get_blob(BLOB_SHA).await.unwrap();

    assert_eq!(blob.sha, BLOB_SHA);
    assert_eq!(blob.encoding, "base64");
    assert_eq!(blob.size, 12);
}

#[tokio::test]
async fn should_create_blob_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let response_json = serde_json::json!({
        "url": format!("https://api.github.com/repos/{OWNER}/{REPO}/git/blobs/{BLOB_SHA}"),
        "sha": BLOB_SHA
    });

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/blobs")))
        .and(body_json(serde_json::json!({
            "content": "Hello World",
            "encoding": "utf-8"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(&server, "POST on git blobs failed").await;

    let created = octocrab
        .git(OWNER, REPO)
        .create_blob("Hello World")
        .encoding("utf-8")
        .send()
        .await
        .unwrap();

    assert_eq!(created.sha, BLOB_SHA);
}

// ---------------------------------------------------------------------------
// Commit tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_get_commit_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/git/commits/{COMMIT_SHA}"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_commit_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on git commits failed").await;

    let commit = octocrab
        .git(OWNER, REPO)
        .get_commit(COMMIT_SHA)
        .await
        .unwrap();

    assert_eq!(commit.sha, COMMIT_SHA);
    assert_eq!(commit.message, "commit message");
    assert_eq!(commit.author.name, "Monalisa Octocat");
}

#[tokio::test]
async fn should_create_commit_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/commits")))
        .and(body_json(serde_json::json!({
            "message": "commit message",
            "tree": "691272480426f78a0138979dd3ce63b77f706feb",
            "parents": ["1acc419d4d6a9ce985db7be48c6349a0475975b5"]
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_commit_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "POST on git commits failed").await;

    let commit = octocrab
        .git(OWNER, REPO)
        .create_commit("commit message", "691272480426f78a0138979dd3ce63b77f706feb")
        .parents(vec!["1acc419d4d6a9ce985db7be48c6349a0475975b5".to_string()])
        .send()
        .await
        .unwrap();

    assert_eq!(commit.sha, COMMIT_SHA);
    assert_eq!(commit.message, "commit message");
}

// ---------------------------------------------------------------------------
// Reference tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_get_ref_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/ref/heads/master")))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(sample_ref_json("refs/heads/master")),
        )
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on git ref failed").await;

    let reference = octocrab
        .git(OWNER, REPO)
        .get_ref(&Reference::Branch("master".to_string()))
        .await
        .unwrap();

    assert_eq!(reference.ref_field, "refs/heads/master");
}

#[tokio::test]
async fn should_create_ref_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/refs")))
        .and(body_json(serde_json::json!({
            "ref": "refs/tags/v1.0",
            "sha": "aa218f56b14c9653891f9e74264a383fa43fefbd"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_ref_json("refs/tags/v1.0")))
        .mount(&server)
        .await;

    setup_error_handler(&server, "POST on git refs failed").await;

    let reference = octocrab
        .git(OWNER, REPO)
        .create_ref(
            &Reference::Tag("v1.0".to_string()),
            "aa218f56b14c9653891f9e74264a383fa43fefbd",
        )
        .await
        .unwrap();

    assert_eq!(reference.ref_field, "refs/tags/v1.0");
}

#[tokio::test]
async fn should_update_ref_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("PATCH"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/git/refs/heads/feature-a"
        )))
        .and(body_json(serde_json::json!({
            "sha": "aa218f56b14c9653891f9e74264a383fa43fefbd",
            "force": true
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(sample_ref_json("refs/heads/feature-a")),
        )
        .mount(&server)
        .await;

    setup_error_handler(&server, "PATCH on git refs failed").await;

    let reference = octocrab
        .git(OWNER, REPO)
        .update_ref(
            &Reference::Branch("feature-a".to_string()),
            "aa218f56b14c9653891f9e74264a383fa43fefbd",
        )
        .force(true)
        .send()
        .await
        .unwrap();

    assert_eq!(reference.ref_field, "refs/heads/feature-a");
}

#[tokio::test]
async fn should_delete_ref_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/git/refs/heads/temporary-branch"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    setup_error_handler(&server, "DELETE on git refs failed").await;

    let result = octocrab
        .git(OWNER, REPO)
        .delete_ref(&Reference::Branch("temporary-branch".to_string()))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_list_matching_refs_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/git/matching-refs/heads"
        )))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([sample_ref_json("refs/heads/master")])),
        )
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on matching-refs failed").await;

    let refs = octocrab
        .git(OWNER, REPO)
        .list_matching_refs("heads")
        .await
        .unwrap();

    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].ref_field, "refs/heads/master");
}

// ---------------------------------------------------------------------------
// Tag tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_get_tag_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/tags/{TAG_SHA}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_tag_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on git tags failed").await;

    let tag = octocrab.git(OWNER, REPO).get_tag(TAG_SHA).await.unwrap();

    assert_eq!(tag.sha, TAG_SHA);
    assert_eq!(tag.tag, "v0.0.1");
    assert_eq!(tag.message, "initial version");
    assert_eq!(tag.tagger.as_ref().unwrap().name, "Monalisa Octocat");
    assert_eq!(
        tag.object.as_ref().unwrap().sha,
        "c3d0be41ecbe669545ee3e94d31ed9a4bc91ee3c"
    );
}

#[tokio::test]
async fn should_create_tag_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/tags")))
        .and(body_json(serde_json::json!({
            "tag": "v0.0.1",
            "message": "initial version",
            "object": "c3d0be41ecbe669545ee3e94d31ed9a4bc91ee3c",
            "type": "commit"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_tag_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "POST on git tags failed").await;

    let tag = octocrab
        .git(OWNER, REPO)
        .create_tag(
            "v0.0.1",
            "initial version",
            "c3d0be41ecbe669545ee3e94d31ed9a4bc91ee3c",
            "commit",
        )
        .send()
        .await
        .unwrap();

    assert_eq!(tag.tag, "v0.0.1");
    assert_eq!(tag.sha, TAG_SHA);
}

// ---------------------------------------------------------------------------
// Tree tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_get_tree_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/trees/{TREE_SHA}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_tree_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on git trees failed").await;

    let tree = octocrab
        .git(OWNER, REPO)
        .get_tree(TREE_SHA)
        .send()
        .await
        .unwrap();

    assert_eq!(tree.sha, TREE_SHA);
    assert_eq!(tree.tree.len(), 1);
    assert_eq!(tree.tree[0].path, "file.rb");
    assert_eq!(tree.tree[0].mode, "100644");
    assert_eq!(tree.tree[0].r#type, "blob");
}

#[tokio::test]
async fn should_get_tree_recursive_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/trees/{TREE_SHA}")))
        .and(query_param("recursive", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_tree_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on recursive git trees failed").await;

    let tree = octocrab
        .git(OWNER, REPO)
        .get_tree(TREE_SHA)
        .recursive(true)
        .send()
        .await
        .unwrap();

    assert_eq!(tree.sha, TREE_SHA);
}

#[tokio::test]
async fn should_create_tree_via_git_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/trees")))
        .and(body_json(serde_json::json!({
            "tree": [
                {
                    "path": "file.rb",
                    "mode": "100644",
                    "type": "blob",
                    "sha": "44b4fc6d56897b048c772eb4087f854f46256132"
                }
            ]
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_tree_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "POST on git trees failed").await;

    let entry = CreateTreeEntry::new("file.rb", "100644", "blob")
        .with_sha("44b4fc6d56897b048c772eb4087f854f46256132");

    let tree = octocrab
        .git(OWNER, REPO)
        .create_tree(vec![entry])
        .send()
        .await
        .unwrap();

    assert_eq!(tree.sha, TREE_SHA);
}

// ---------------------------------------------------------------------------
// RepoHandler & RepositoryId integration
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_access_git_via_repos_handler() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/git/blobs/{BLOB_SHA}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_blob_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on git blobs via repos failed").await;

    let blob = octocrab
        .repos(OWNER, REPO)
        .git()
        .get_blob(BLOB_SHA)
        .await
        .unwrap();

    assert_eq!(blob.sha, BLOB_SHA);

    // Also test convenience method on RepoHandler
    let blob_convenience = octocrab
        .repos(OWNER, REPO)
        .get_blob(BLOB_SHA)
        .await
        .unwrap();

    assert_eq!(blob_convenience.sha, BLOB_SHA);
}

#[tokio::test]
async fn should_access_git_by_id() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());
    let repo_id: u64 = 123456;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repositories/{repo_id}/git/blobs/{BLOB_SHA}"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_blob_json()))
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on git blobs by id failed").await;

    let blob = octocrab
        .git_by_id(RepositoryId(repo_id))
        .get_blob(BLOB_SHA)
        .await
        .unwrap();

    assert_eq!(blob.sha, BLOB_SHA);
}
