use octocrab::models::billing::{
    BudgetAlerting, BudgetScope, BudgetType, CreateBudget, UpdateBudget,
};
use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};


use crate::mock_error::setup_error_handler;

const ORG: &str = "test-org";
const USER: &str = "test-user";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_actions_billing() -> serde_json::Value {
    json!({
        "total_minutes_used": 305,
        "total_paid_minutes_used": 0,
        "included_minutes": 3000,
        "minutes_used_breakdown": {
            "UBUNTU": 205,
            "MACOS": 10,
            "WINDOWS": 90,
            "ubuntu_4_core": 0,
            "macos_14_core": 5,
            "total": 310
        }
    })
}

fn sample_packages_billing() -> serde_json::Value {
    json!({
        "total_gigabytes_bandwidth_used": 50,
        "total_paid_gigabytes_bandwidth_used": 40,
        "included_gigabytes_bandwidth": 10
    })
}

fn sample_shared_storage_billing() -> serde_json::Value {
    json!({
        "days_left_in_billing_cycle": 20,
        "estimated_paid_storage_for_month": 15,
        "estimated_storage_for_month": 40
    })
}

fn sample_usage_report(org_or_user: &str, is_org: bool) -> serde_json::Value {
    let mut item = json!({
        "date": "2026-09-13",
        "product": "Actions",
        "sku": "Compute - UBUNTU",
        "quantity": 100,
        "unitType": "minute",
        "pricePerUnit": 0.008,
        "grossAmount": 0.8,
        "discountAmount": 0.0,
        "netAmount": 0.8
    });
    if is_org {
        item["organizationName"] = json!(org_or_user);
    }
    json!({
        "usageItems": [item]
    })
}

fn sample_usage_summary(org_or_user: &str, is_org: bool) -> serde_json::Value {
    let mut obj = json!({
        "timePeriod": {
            "year": 2026,
            "month": 9,
            "day": 13
        },
        "usageItems": [
            {
                "product": "Actions",
                "sku": "Compute - UBUNTU",
                "unitType": "minute",
                "pricePerUnit": 0.008,
                "grossQuantity": 100.0,
                "grossAmount": 0.8,
                "discountQuantity": 0.0,
                "discountAmount": 0.0,
                "netQuantity": 100.0,
                "netAmount": 0.8
            }
        ]
    });
    if is_org {
        obj["organization"] = json!(org_or_user);
    } else {
        obj["user"] = json!(org_or_user);
    }
    obj
}

fn sample_ai_credit_usage(org_or_user: &str, is_org: bool) -> serde_json::Value {
    let mut obj = json!({
        "timePeriod": {
            "year": 2026,
            "month": 9
        },
        "usageItems": [
            {
                "product": "Copilot",
                "sku": "Copilot Enterprise",
                "model": "gpt-4o",
                "unitType": "credit",
                "pricePerUnit": 0.01,
                "grossQuantity": 50.0,
                "grossAmount": 0.5,
                "discountQuantity": 0.0,
                "discountAmount": 0.0,
                "netQuantity": 50.0,
                "netAmount": 0.5
            }
        ]
    });
    if is_org {
        obj["organization"] = json!(org_or_user);
    } else {
        obj["user"] = json!(org_or_user);
    }
    obj
}

fn sample_premium_request_usage(org_or_user: &str, is_org: bool) -> serde_json::Value {
    let mut obj = json!({
        "timePeriod": {
            "year": 2026,
            "month": 9
        },
        "usageItems": [
            {
                "product": "Copilot",
                "sku": "Copilot Enterprise",
                "model": "claude-3-5-sonnet",
                "unitType": "request",
                "pricePerUnit": 0.02,
                "grossQuantity": 25.0,
                "grossAmount": 0.5,
                "discountQuantity": 0.0,
                "discountAmount": 0.0,
                "netQuantity": 25.0,
                "netAmount": 0.5
            }
        ]
    });
    if is_org {
        obj["organization"] = json!(org_or_user);
    } else {
        obj["user"] = json!(org_or_user);
    }
    obj
}

fn sample_budget(id: &str) -> serde_json::Value {
    json!({
        "id": id,
        "budget_type": "SkuPricing",
        "budget_amount": 500,
        "prevent_further_usage": true,
        "budget_scope": "organization",
        "budget_entity_name": ORG,
        "budget_product_sku": "Actions",
        "consumed_amount": 120.5,
        "budget_alerting": {
            "will_alert": true,
            "alert_recipients": ["octocat"]
        }
    })
}

// =========================================================================
// Actions Billing Tests
// =========================================================================

#[tokio::test]
async fn should_get_actions_billing_for_org() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/settings/billing/actions");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_actions_billing()))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /orgs/{org}/settings/billing/actions was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    // Via orgs("...").billing()
    let usage1 = client.orgs(ORG).billing().actions().await.unwrap();
    assert_eq!(usage1.total_minutes_used, 305);
    assert_eq!(usage1.total_paid_minutes_used, 0);
    assert_eq!(usage1.included_minutes, 3000);
    assert_eq!(usage1.minutes_used_breakdown.ubuntu, Some(205));
    assert_eq!(usage1.minutes_used_breakdown.macos, Some(10));
    assert_eq!(usage1.minutes_used_breakdown.windows, Some(90));
    assert_eq!(usage1.minutes_used_breakdown.total, Some(310));
    // Verify extra unrecognized runner tier captured in `extra`
    assert_eq!(
        usage1.minutes_used_breakdown.extra.get("macos_14_core"),
        Some(&5)
    );

    // Via billing().org("...")
    let usage2 = client.billing().org(ORG).actions().await.unwrap();
    assert_eq!(usage2.total_minutes_used, 305);
}

#[tokio::test]
async fn should_get_actions_billing_for_user() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/users/{USER}/settings/billing/actions");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_actions_billing()))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /users/{user}/settings/billing/actions was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    // Via users("...").billing()
    let usage1 = client.users(USER).billing().actions().await.unwrap();
    assert_eq!(usage1.total_minutes_used, 305);

    // Via billing().user("...")
    let usage2 = client.billing().user(USER).actions().await.unwrap();
    assert_eq!(usage2.total_minutes_used, 305);
}

// =========================================================================
// Packages Billing Tests
// =========================================================================

#[tokio::test]
async fn should_get_packages_billing_for_org() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/settings/billing/packages");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_packages_billing()))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /orgs/{org}/settings/billing/packages was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let usage = client.orgs(ORG).billing().packages().await.unwrap();
    assert_eq!(usage.total_gigabytes_bandwidth_used, 50);
    assert_eq!(usage.total_paid_gigabytes_bandwidth_used, 40);
    assert_eq!(usage.included_gigabytes_bandwidth, 10);

    let usage_top = client.billing().org(ORG).packages().await.unwrap();
    assert_eq!(usage_top.total_gigabytes_bandwidth_used, 50);
}

#[tokio::test]
async fn should_get_packages_billing_for_user() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/users/{USER}/settings/billing/packages");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_packages_billing()))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /users/{user}/settings/billing/packages was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let usage = client.users(USER).billing().packages().await.unwrap();
    assert_eq!(usage.total_gigabytes_bandwidth_used, 50);

    let usage_top = client.billing().user(USER).packages().await.unwrap();
    assert_eq!(usage_top.total_gigabytes_bandwidth_used, 50);
}

// =========================================================================
// Shared Storage Billing Tests
// =========================================================================

#[tokio::test]
async fn should_get_shared_storage_billing_for_org() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/settings/billing/shared-storage");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_shared_storage_billing()))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /orgs/{org}/settings/billing/shared-storage was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let usage = client.orgs(ORG).billing().shared_storage().await.unwrap();
    assert_eq!(usage.days_left_in_billing_cycle, 20);
    assert_eq!(usage.estimated_paid_storage_for_month, 15);
    assert_eq!(usage.estimated_storage_for_month, 40);

    let usage_top = client.billing().org(ORG).shared_storage().await.unwrap();
    assert_eq!(usage_top.days_left_in_billing_cycle, 20);
}

#[tokio::test]
async fn should_get_shared_storage_billing_for_user() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/users/{USER}/settings/billing/shared-storage");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_shared_storage_billing()))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /users/{user}/settings/billing/shared-storage was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let usage = client.users(USER).billing().shared_storage().await.unwrap();
    assert_eq!(usage.days_left_in_billing_cycle, 20);

    let usage_top = client.billing().user(USER).shared_storage().await.unwrap();
    assert_eq!(usage_top.days_left_in_billing_cycle, 20);
}

// =========================================================================
// Usage Reports & Summaries Tests
// =========================================================================

#[tokio::test]
async fn should_get_billing_usage_report_for_org() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/organizations/{ORG}/settings/billing/usage");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("year", "2026"))
        .and(query_param("month", "9"))
        .and(query_param("day", "13"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_usage_report(ORG, true)))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /organizations/{org}/settings/billing/usage was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let report = client
        .orgs(ORG)
        .billing()
        .usage(2026)
        .month(9u8)
        .day(13u8)
        .send()
        .await
        .unwrap();

    assert_eq!(report.usage_items.len(), 1);
    assert_eq!(report.usage_items[0].product, "Actions");
    assert_eq!(
        report.usage_items[0].organization_name.as_deref(),
        Some(ORG)
    );
}

#[tokio::test]
async fn should_get_billing_usage_report_for_user() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/users/{USER}/settings/billing/usage");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("year", "2026"))
        .and(query_param("month", "9"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_usage_report(USER, false)))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /users/{user}/settings/billing/usage was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let report = client
        .users(USER)
        .billing()
        .usage(2026)
        .month(9u8)
        .send()
        .await
        .unwrap();

    assert_eq!(report.usage_items.len(), 1);
    assert_eq!(report.usage_items[0].product, "Actions");
    assert_eq!(report.usage_items[0].quantity, 100);
}

#[tokio::test]
async fn should_get_billing_usage_summary_for_org() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/organizations/{ORG}/settings/billing/usage/summary");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("year", "2026"))
        .and(query_param("month", "9"))
        .and(query_param("day", "13"))
        .and(query_param("repository", "octocrab"))
        .and(query_param("product", "Actions"))
        .and(query_param("sku", "Compute - UBUNTU"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_usage_summary(ORG, true)))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /organizations/{org}/settings/billing/usage/summary was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let summary = client
        .orgs(ORG)
        .billing()
        .usage_summary(2026)
        .month(9u8)
        .day(13u8)
        .repository("octocrab")
        .product("Actions")
        .sku("Compute - UBUNTU")
        .send()
        .await
        .unwrap();

    assert_eq!(summary.time_period.year, 2026);
    assert_eq!(summary.organization.as_deref(), Some(ORG));
    assert_eq!(summary.usage_items.len(), 1);
}

#[tokio::test]
async fn should_get_billing_usage_summary_for_user() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/users/{USER}/settings/billing/usage/summary");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("year", "2026"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_usage_summary(USER, false)))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /users/{user}/settings/billing/usage/summary was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let summary = client
        .users(USER)
        .billing()
        .usage_summary(2026)
        .send()
        .await
        .unwrap();

    assert_eq!(summary.time_period.year, 2026);
    assert_eq!(summary.user.as_deref(), Some(USER));
}

#[tokio::test]
async fn should_get_ai_credit_usage_report() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/organizations/{ORG}/settings/billing/ai_credit/usage");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("year", "2026"))
        .and(query_param("month", "9"))
        .and(query_param("user", "mona"))
        .and(query_param("model", "gpt-4o"))
        .and(query_param("product", "Copilot"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_ai_credit_usage(ORG, true)))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /organizations/{org}/settings/billing/ai_credit/usage was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let report = client
        .orgs(ORG)
        .billing()
        .ai_credit_usage(2026)
        .month(9u8)
        .user("mona")
        .model("gpt-4o")
        .product("Copilot")
        .send()
        .await
        .unwrap();

    assert_eq!(report.time_period.year, 2026);
    assert_eq!(report.organization.as_deref(), Some(ORG));
    assert_eq!(report.usage_items[0].model, "gpt-4o");
}

#[tokio::test]
async fn should_get_premium_request_usage_report() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/organizations/{ORG}/settings/billing/premium_request/usage");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("year", "2026"))
        .and(query_param("month", "9"))
        .and(query_param("user", "mona"))
        .and(query_param("model", "claude-3-5-sonnet"))
        .and(query_param("product", "Copilot"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(sample_premium_request_usage(ORG, true)),
        )
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /organizations/{org}/settings/billing/premium_request/usage was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let report = client
        .orgs(ORG)
        .billing()
        .premium_request_usage(2026)
        .month(9u8)
        .user("mona")
        .model("claude-3-5-sonnet")
        .product("Copilot")
        .send()
        .await
        .unwrap();

    assert_eq!(report.time_period.year, 2026);
    assert_eq!(report.usage_items[0].model, "claude-3-5-sonnet");
}

// =========================================================================
// Organization Budgets Tests
// =========================================================================

#[tokio::test]
async fn should_list_budgets_for_org() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/organizations/{ORG}/settings/billing/budgets");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("page", "1"))
        .and(query_param("per_page", "10"))
        .and(query_param("scope", "organization"))
        .and(query_param("user", "octocat"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "budgets": [sample_budget("b-1")],
            "total_count": 1
        })))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /organizations/{org}/settings/billing/budgets was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let res = client
        .orgs(ORG)
        .billing()
        .budgets()
        .list()
        .page(1u32)
        .per_page(10u8)
        .scope("organization")
        .user("octocat")
        .send()
        .await
        .unwrap();

    assert_eq!(res.budgets.len(), 1);
    assert_eq!(res.budgets[0].id, "b-1");
    assert_eq!(res.budgets[0].budget_amount, 500);
}

#[tokio::test]
async fn should_create_budget_for_org() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/organizations/{ORG}/settings/billing/budgets");

    Mock::given(method("POST"))
        .and(path(&expected_path))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "message": "Budget created successfully",
            "budget": sample_budget("b-2")
        })))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "POST /organizations/{org}/settings/billing/budgets was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let payload = CreateBudget {
        budget_amount: 500,
        prevent_further_usage: true,
        budget_alerting: BudgetAlerting {
            will_alert: true,
            alert_recipients: vec!["octocat".to_string()],
        },
        budget_scope: BudgetScope::Organization,
        budget_entity_name: Some(ORG.to_string()),
        budget_type: Some(BudgetType::SkuPricing),
        budget_product_sku: "Actions".to_string(),
        user: None,
        expires_at: None,
    };

    let res = client
        .orgs(ORG)
        .billing()
        .budgets()
        .create(&payload)
        .await
        .unwrap();
    assert_eq!(res.message, "Budget created successfully");
    assert_eq!(res.budget.id, "b-2");
}

#[tokio::test]
async fn should_get_budget_by_id_for_org() {
    let mock_server = MockServer::start().await;
    let budget_id = "b-3";
    let expected_path = format!("/organizations/{ORG}/settings/billing/budgets/{budget_id}");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_budget(budget_id)))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET /organizations/{org}/settings/billing/budgets/{budget_id} was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let budget = client
        .orgs(ORG)
        .billing()
        .budgets()
        .get(budget_id)
        .await
        .unwrap();
    assert_eq!(budget.id, budget_id);
    assert_eq!(budget.budget_amount, 500);
}

#[tokio::test]
async fn should_update_budget_for_org() {
    let mock_server = MockServer::start().await;
    let budget_id = "b-4";
    let expected_path = format!("/organizations/{ORG}/settings/billing/budgets/{budget_id}");

    let mut updated = sample_budget(budget_id);
    updated["budget_amount"] = json!(600);

    Mock::given(method("PATCH"))
        .and(path(&expected_path))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "message": "Budget updated successfully",
            "budget": updated
        })))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "PATCH /organizations/{org}/settings/billing/budgets/{budget_id} was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let patch = UpdateBudget {
        budget_amount: Some(600),
        ..Default::default()
    };

    let res = client
        .orgs(ORG)
        .billing()
        .budgets()
        .update(budget_id, &patch)
        .await
        .unwrap();

    assert_eq!(res.budget.budget_amount, 600);
}

#[tokio::test]
async fn should_delete_budget_for_org() {
    let mock_server = MockServer::start().await;
    let budget_id = "b-5";
    let expected_path = format!("/organizations/{ORG}/settings/billing/budgets/{budget_id}");

    Mock::given(method("DELETE"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "message": "Budget deleted successfully",
            "id": budget_id
        })))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "DELETE /organizations/{org}/settings/billing/budgets/{budget_id} was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let res = client
        .orgs(ORG)
        .billing()
        .budgets()
        .delete(budget_id)
        .await
        .unwrap();
    assert_eq!(res.id, budget_id);
}

// =========================================================================
// Error Handling Test
// =========================================================================

#[tokio::test]
async fn should_map_errors_for_billing() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/settings/billing/actions");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "message": "Not Found",
            "documentation_url": "https://docs.github.com/rest"
        })))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let err = client.orgs(ORG).billing().actions().await.unwrap_err();

    match err {
        octocrab::Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::NOT_FOUND);
            assert_eq!(source.message, "Not Found");
        }
        other => panic!("expected GitHub error, got: {:?}", other),
    }
}
