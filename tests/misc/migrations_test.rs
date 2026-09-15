use octocrab::models::migrations::{
    ImportStatus, LfsPreference, MapCommitAuthor, StartImport, StartMigration, UpdateImport,
};
use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_user() -> serde_json::Value {
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

fn sample_repo() -> serde_json::Value {
    json!({
        "id": 1296269,
        "node_id": "MDEwOlJlcG9zaXRvcnkxMjk2MjY5",
        "name": "Hello-World",
        "full_name": "octocat/Hello-World",
        "owner": sample_user(),
        "private": false,
        "html_url": "https://github.com/octocat/Hello-World",
        "description": "This your first repo!",
        "fork": false,
        "url": "https://api.github.com/repos/octocat/Hello-World"
    })
}

fn sample_migration(id: u64, state: &str) -> serde_json::Value {
    json!({
        "id": id,
        "node_id": "MDEyOk9yZ2FuaXphdGlvbjE=",
        "owner": sample_user(),
        "guid": "0b989ba4-242f-11e5-81e1-c7b6966d2516",
        "state": state,
        "lock_repositories": true,
        "exclude_metadata": false,
        "exclude_git_data": false,
        "exclude_attachments": false,
        "exclude_releases": false,
        "exclude_owner_projects": false,
        "org_metadata_only": false,
        "repositories": [sample_repo()],
        "url": format!("https://api.github.com/orgs/octo-org/migrations/{id}"),
        "created_at": "2015-07-06T15:33:38Z",
        "updated_at": "2015-07-06T15:33:38Z"
    })
}

fn sample_import() -> serde_json::Value {
    json!({
        "vcs": "subversion",
        "use_lfs": true,
        "vcs_url": "http://svn.example.com/svn/myproject",
        "status": "complete",
        "status_text": "Done",
        "has_large_files": true,
        "large_files_size": 132331036,
        "large_files_count": 1,
        "authors_count": 4,
        "url": "https://api.github.com/repos/owner/repo/import",
        "html_url": "https://import.github.com/owner/repo/import",
        "authors_url": "https://api.github.com/repos/owner/repo/import/authors",
        "repository_url": "https://api.github.com/repos/owner/repo"
    })
}

fn sample_import_author(id: u64) -> serde_json::Value {
    json!({
        "id": id,
        "remote_id": "nobody@fc7da526",
        "remote_name": "nobody",
        "email": "hubot@github.com",
        "name": "Hubot",
        "url": format!("https://api.github.com/repos/owner/repo/import/authors/{id}"),
        "import_url": "https://api.github.com/repos/owner/repo/import"
    })
}

fn sample_large_file() -> serde_json::Value {
    json!({
        "ref_name": "refs/heads/master",
        "path": "foo/bar/1",
        "oid": "d3d9446802a44259755d38e6d163e820",
        "size": 10485760
    })
}

// ---------------- User Migrations Tests ----------------

#[tokio::test]
async fn test_user_migrations_list() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/migrations"))
        .and(query_param("per_page", "30"))
        .and(query_param("page", "1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!([sample_migration(79, "pending")])),
        )
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let page = octo
        .migrations()
        .list()
        .per_page(30u8)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id.0, 79);
    assert_eq!(page.items[0].state, "pending");

    // Also test via octo.current().migrations()
    let current_page = octo
        .current()
        .migrations()
        .list()
        .per_page(30u8)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(current_page.items.len(), 1);
    assert_eq!(current_page.items[0].id.0, 79);
}

#[tokio::test]
async fn test_user_migrations_start() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/user/migrations"))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_migration(79, "pending")))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let migration = octo
        .migrations()
        .start(
            &StartMigration::new(["owner/repo"])
                .lock_repositories(true)
                .exclude_attachments(true),
        )
        .await
        .unwrap();

    assert_eq!(migration.id.0, 79);
    assert_eq!(migration.state, "pending");
    assert!(migration.lock_repositories);
}

#[tokio::test]
async fn test_user_migrations_get() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/migrations/79"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_migration(79, "exported")))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let migration = octo.migrations().get(79u64).await.unwrap();
    assert_eq!(migration.id.0, 79);
    assert_eq!(migration.state, "exported");
}

#[tokio::test]
async fn test_user_migrations_get_status_with_exclude() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/migrations/79"))
        .and(query_param("exclude", "repositories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_migration(79, "exported")))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let migration = octo
        .migrations()
        .get_status(79u64)
        .exclude(["repositories"])
        .send()
        .await
        .unwrap();

    assert_eq!(migration.id.0, 79);
    assert_eq!(migration.state, "exported");
}

#[tokio::test]
async fn test_user_migrations_download_archive() {
    let server = MockServer::start().await;

    let redirect_url = format!("{}/archive/download/user_79.tar.gz", server.uri());

    Mock::given(method("GET"))
        .and(path("/user/migrations/79/archive"))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", redirect_url.as_str()))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/archive/download/user_79.tar.gz"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"user-archive-bytes".to_vec()))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let archive = octo.migrations().download_archive(79u64).await.unwrap();
    assert_eq!(&archive[..], b"user-archive-bytes");
}

#[tokio::test]
async fn test_user_migrations_delete_archive() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/user/migrations/79/archive"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    octo.migrations().delete_archive(79u64).await.unwrap();
}

#[tokio::test]
async fn test_user_migrations_unlock_repo() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/user/migrations/79/repos/my-repo/lock"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    octo.migrations()
        .unlock_repo(79u64, "my-repo")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_user_migrations_list_repos() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/migrations/79/repositories"))
        .and(query_param("per_page", "10"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_repo()])))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let page = octo
        .migrations()
        .list_repos(79u64)
        .per_page(10u8)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, "Hello-World");

    // Also test alias
    let alias_page = octo
        .migrations()
        .list_repositories(79u64)
        .per_page(10u8)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(alias_page.items.len(), 1);
}

// ---------------- Organization Migrations Tests ----------------

#[tokio::test]
async fn test_org_migrations_list() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/orgs/octo-org/migrations"))
        .and(query_param("per_page", "20"))
        .and(query_param("page", "2"))
        .and(query_param("exclude", "repositories"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!([sample_migration(80, "pending")])),
        )
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let page = octo
        .orgs("octo-org")
        .migrations()
        .list()
        .per_page(20u8)
        .page(2u32)
        .exclude(["repositories"])
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id.0, 80);

    // Also test via octo.migrations().org("octo-org")
    let page_via_migrations = octo
        .migrations()
        .org("octo-org")
        .list()
        .per_page(20u8)
        .page(2u32)
        .exclude(["repositories"])
        .send()
        .await
        .unwrap();

    assert_eq!(page_via_migrations.items.len(), 1);
}

#[tokio::test]
async fn test_org_migrations_start() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/orgs/octo-org/migrations"))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_migration(80, "pending")))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let migration = octo
        .orgs("octo-org")
        .migrations()
        .start(
            &StartMigration::new(["octo-org/repo"])
                .exclude_attachments(true)
                .exclude_releases(true),
        )
        .await
        .unwrap();

    assert_eq!(migration.id.0, 80);
    assert_eq!(migration.state, "pending");
}

#[tokio::test]
async fn test_org_migrations_get() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/orgs/octo-org/migrations/80"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_migration(80, "exporting")))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let migration = octo.orgs("octo-org").migrations().get(80u64).await.unwrap();
    assert_eq!(migration.id.0, 80);
    assert_eq!(migration.state, "exporting");
}

#[tokio::test]
async fn test_org_migrations_get_status_with_exclude() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/orgs/octo-org/migrations/80"))
        .and(query_param("exclude", "repositories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_migration(80, "exporting")))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let migration = octo
        .orgs("octo-org")
        .migrations()
        .get_status(80u64)
        .exclude(["repositories"])
        .send()
        .await
        .unwrap();

    assert_eq!(migration.id.0, 80);
    assert_eq!(migration.state, "exporting");
}

#[tokio::test]
async fn test_org_migrations_download_archive() {
    let server = MockServer::start().await;

    let redirect_url = format!("{}/archive/download/org_80.tar.gz", server.uri());

    Mock::given(method("GET"))
        .and(path("/orgs/octo-org/migrations/80/archive"))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", redirect_url.as_str()))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/archive/download/org_80.tar.gz"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"org-archive-content".to_vec()))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let archive = octo
        .orgs("octo-org")
        .migrations()
        .download_archive(80u64)
        .await
        .unwrap();

    assert_eq!(&archive[..], b"org-archive-content");
}

#[tokio::test]
async fn test_org_migrations_delete_archive() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/orgs/octo-org/migrations/80/archive"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    octo.orgs("octo-org")
        .migrations()
        .delete_archive(80u64)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_org_migrations_unlock_repo() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/orgs/octo-org/migrations/80/repos/my-repo/lock"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    octo.orgs("octo-org")
        .migrations()
        .unlock_repo(80u64, "my-repo")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_org_migrations_list_repos() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/orgs/octo-org/migrations/80/repositories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_repo()])))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let page = octo
        .orgs("octo-org")
        .migrations()
        .list_repos(80u64)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, "Hello-World");
}

// ---------------- Repository Source Imports Tests ----------------

#[tokio::test]
async fn test_repo_import_get() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/import"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_import()))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let imp = octo.repos("owner", "repo").import().get().await.unwrap();
    assert_eq!(imp.vcs.as_deref(), Some("subversion"));
    assert_eq!(imp.status, ImportStatus::Complete);
    assert_eq!(imp.authors_count, Some(4));

    // Also test via alias .imports()
    let imp_alias = octo.repos("owner", "repo").imports().get().await.unwrap();
    assert_eq!(imp_alias.status, ImportStatus::Complete);
}

#[tokio::test]
async fn test_repo_import_start() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/import"))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_import()))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let imp = octo
        .repos("owner", "repo")
        .import()
        .start(
            &StartImport::new("http://svn.example.com/svn/myproject")
                .vcs("subversion")
                .vcs_username("user")
                .vcs_password("pass"),
        )
        .await
        .unwrap();

    assert_eq!(imp.vcs_url, "http://svn.example.com/svn/myproject");
    assert_eq!(imp.status, ImportStatus::Complete);
}

#[tokio::test]
async fn test_repo_import_update() {
    let server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/repos/owner/repo/import"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_import()))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let imp = octo
        .repos("owner", "repo")
        .import()
        .update(&UpdateImport::new().vcs_username("new_user"))
        .await
        .unwrap();

    assert_eq!(imp.status, ImportStatus::Complete);
}

#[tokio::test]
async fn test_repo_import_cancel() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/import"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    octo.repos("owner", "repo").import().cancel().await.unwrap();
}

#[tokio::test]
async fn test_repo_import_authors() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/import/authors"))
        .and(query_param("since", "100"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_import_author(101)])))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/import/authors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_import_author(101)])))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let authors_with_since = octo
        .repos("owner", "repo")
        .import()
        .authors(Some(100))
        .await
        .unwrap();

    assert_eq!(authors_with_since.len(), 1);
    assert_eq!(authors_with_since[0].id.0, 101);
    assert_eq!(authors_with_since[0].name, "Hubot");

    let authors_without_since = octo
        .repos("owner", "repo")
        .import()
        .authors(None)
        .await
        .unwrap();

    assert_eq!(authors_without_since.len(), 1);
}

#[tokio::test]
async fn test_repo_import_map_author() {
    let server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/repos/owner/repo/import/authors/101"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_import_author(101)))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let author = octo
        .repos("owner", "repo")
        .import()
        .map_author(
            101u64,
            &MapCommitAuthor::new()
                .email("hubot@github.com")
                .name("Hubot"),
        )
        .await
        .unwrap();

    assert_eq!(author.id.0, 101);
    assert_eq!(author.email, "hubot@github.com");
}

#[tokio::test]
async fn test_repo_import_large_files() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/import/large_files"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_large_file()])))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let files = octo
        .repos("owner", "repo")
        .import()
        .large_files()
        .await
        .unwrap();

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "foo/bar/1");
    assert_eq!(files[0].size, 10485760);
}

#[tokio::test]
async fn test_repo_import_set_lfs_preference() {
    let server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/repos/owner/repo/import/lfs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_import()))
        .mount(&server)
        .await;

    let octo = setup_octocrab(&server.uri());

    let imp = octo
        .repos("owner", "repo")
        .import()
        .set_lfs_preference(LfsPreference::OptIn)
        .await
        .unwrap();

    assert_eq!(imp.status, ImportStatus::Complete);
}
