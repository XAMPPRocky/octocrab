use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_plans_json() -> serde_json::Value {
    json!([
        {
            "url": "https://api.github.com/marketplace_listing/plans/1313",
            "accounts_url": "https://api.github.com/marketplace_listing/plans/1313/accounts",
            "id": 1313,
            "number": 1,
            "name": "Pro",
            "description": "A really good maker of things",
            "monthly_price_in_cents": 1099,
            "yearly_price_in_cents": 11870,
            "price_model": "flat-rate",
            "has_free_trial": true,
            "unit_name": null,
            "state": "published",
            "bullets": [
                "Up to 20 users",
                "24/7 support"
            ]
        }
    ])
}

fn sample_account_json() -> serde_json::Value {
    json!({
        "url": "https://api.github.com/marketplace_listing/accounts/1",
        "type": "User",
        "id": 1,
        "login": "octocat",
        "marketplace_pending_change": null,
        "marketplace_purchase": {
            "billing_cycle": "monthly",
            "next_billing_date": "2022-11-10T00:00:00Z",
            "unit_count": null,
            "on_free_trial": true,
            "free_trial_ends_on": "2022-11-11T00:00:00Z",
            "updated_at": "2022-11-03T00:00:00Z",
            "plan": {
                "url": "https://api.github.com/marketplace_listing/plans/1313",
                "accounts_url": "https://api.github.com/marketplace_listing/plans/1313/accounts",
                "id": 1313,
                "number": 1,
                "name": "Pro",
                "description": "A really good maker of things",
                "monthly_price_in_cents": 1099,
                "yearly_price_in_cents": 11870,
                "price_model": "flat-rate",
                "has_free_trial": true,
                "unit_name": null,
                "state": "published",
                "bullets": ["Up to 20 users"]
            }
        }
    })
}

fn sample_user_purchases_json() -> serde_json::Value {
    json!([
        {
            "billing_cycle": "monthly",
            "next_billing_date": "2022-11-10T00:00:00Z",
            "unit_count": null,
            "on_free_trial": true,
            "free_trial_ends_on": "2022-11-11T00:00:00Z",
            "updated_at": "2022-11-03T00:00:00Z",
            "account": {
                "type": "User",
                "id": 1,
                "node_id": "MDQ6VXNlcjE=",
                "url": "https://api.github.com/users/octocat",
                "login": "octocat",
                "organization_billing_email": null
            },
            "plan": {
                "url": "https://api.github.com/marketplace_listing/plans/1313",
                "accounts_url": "https://api.github.com/marketplace_listing/plans/1313/accounts",
                "id": 1313,
                "number": 1,
                "name": "Pro",
                "description": "A really good maker of things",
                "monthly_price_in_cents": 1099,
                "yearly_price_in_cents": 11870,
                "price_model": "flat-rate",
                "has_free_trial": true,
                "unit_name": null,
                "state": "published",
                "bullets": ["Up to 20 users"]
            }
        }
    ])
}

#[tokio::test]
async fn test_list_plans() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/marketplace_listing/plans"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_plans_json()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/marketplace_listing/stubbed/plans"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_plans_json()))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    let plans = client
        .marketplace()
        .list_plans()
        .per_page(10)
        .page(1u32)
        .send()
        .await
        .unwrap();
    assert_eq!(plans.items.len(), 1);
    assert_eq!(plans.items[0].name, "Pro");

    let stubbed_plans = client
        .marketplace()
        .list_plans_stubbed()
        .send()
        .await
        .unwrap();
    assert_eq!(stubbed_plans.items.len(), 1);

    // Via apps().marketplace()
    let apps_plans = client
        .apps()
        .marketplace()
        .list_plans()
        .send()
        .await
        .unwrap();
    assert_eq!(apps_plans.items.len(), 1);
}

#[tokio::test]
async fn test_get_plan_for_account() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/marketplace_listing/accounts/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_account_json()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/marketplace_listing/stubbed/accounts/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_account_json()))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    let account = client.marketplace().get_plan_for_account(1).await.unwrap();
    assert_eq!(account.login, "octocat");
    assert_eq!(account.marketplace_purchase.plan.name, "Pro");

    let stubbed = client
        .marketplace()
        .get_plan_for_account_stubbed(1)
        .await
        .unwrap();
    assert_eq!(stubbed.login, "octocat");
}

#[tokio::test]
async fn test_list_accounts_for_plan() {
    let mock_server = MockServer::start().await;
    let accounts_json = json!([sample_account_json()]);

    Mock::given(method("GET"))
        .and(path("/marketplace_listing/plans/1313/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&accounts_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/marketplace_listing/stubbed/plans/1313/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&accounts_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    let page = client
        .marketplace()
        .list_accounts_for_plan(1313)
        .sort("created")
        .direction("asc")
        .per_page(10)
        .page(1u32)
        .send()
        .await
        .unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].login, "octocat");

    let stubbed_page = client
        .marketplace()
        .list_accounts_for_plan_stubbed(1313)
        .send()
        .await
        .unwrap();
    assert_eq!(stubbed_page.items.len(), 1);
}

#[tokio::test]
async fn test_list_purchases_for_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/marketplace_purchases"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_user_purchases_json()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/user/marketplace_purchases/stubbed"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_user_purchases_json()))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    // Via marketplace handler
    let purchases = client
        .marketplace()
        .list_purchases_for_user()
        .per_page(10)
        .page(1u32)
        .send()
        .await
        .unwrap();
    assert_eq!(purchases.items.len(), 1);
    assert_eq!(purchases.items[0].account.login, "octocat");

    let stubbed = client
        .marketplace()
        .list_purchases_for_user_stubbed()
        .send()
        .await
        .unwrap();
    assert_eq!(stubbed.items.len(), 1);

    // Via current() handler
    let curr_purchases = client
        .current()
        .marketplace_purchases()
        .send()
        .await
        .unwrap();
    assert_eq!(curr_purchases.items.len(), 1);

    let curr_stubbed = client
        .current()
        .marketplace_purchases_stubbed()
        .send()
        .await
        .unwrap();
    assert_eq!(curr_stubbed.items.len(), 1);
}
