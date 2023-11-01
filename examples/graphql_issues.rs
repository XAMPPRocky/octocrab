#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()?;

    let mut variables = serde_json::json!({
        "owner": "XAMPPRocky",
        "name": "octocrab",
        "page_size": 5,
    });

    let pages_to_show = 3;
    let mut page = 1;
    loop {
        if page > pages_to_show {
            break;
        }

        let response: octocrab::Result<serde_json::Value> = octocrab
            .graphql(&serde_json::json!({
                "query": QUERY,
                "variables": variables,
            }))
            .await;

        match response {
            Ok(value) => {
                println!("Page {page}:");
                print_issues(&value);
                if !update_page_info(&mut variables, &value) {
                    break;
                }
            }
            Err(error) => {
                println!("{error:#?}");
                break;
            }
        }

        page += 1;
    }

    Ok(())
}

fn print_issues(value: &serde_json::Value) {
    let issues = value["data"]["repository"]["issues"]["nodes"]
        .as_array()
        .unwrap();

    for issue in issues {
        println!(
            "{} {}",
            issue["url"].as_str().unwrap(),
            issue["title"].as_str().unwrap()
        );
    }
}

fn update_page_info(variables: &mut serde_json::Value, value: &serde_json::Value) -> bool {
    let page_info = value["data"]["repository"]["issues"]["pageInfo"].clone();
    if page_info["hasPreviousPage"].as_bool().unwrap() {
        variables["before"] = page_info["startCursor"].clone();
        true
    } else {
        false
    }
}

const QUERY: &str = r#" query ($owner: String!, $name: String!, $page_size: Int!, $before: String) {
    repository(owner: $owner, name: $name) {
        issues(last: $page_size, before: $before, states: OPEN) {
            nodes {
                title
                url
            }
            pageInfo {
                hasPreviousPage
                startCursor
            }
        }
    }
} "#;
