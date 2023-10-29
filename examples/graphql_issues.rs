#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()?;

    let query = r#" {
        repository(owner:"XAMPPRocky", name:"octocrab") {
            issues(last: 2, states: OPEN) {
                nodes {
                    title
                    url
                }
            }
        }
    } "#;

    let response: octocrab::Result<serde_json::Value> = octocrab
        .graphql(&serde_json::json!({ "query": query }))
        .await;

    match response {
        Ok(value) => println!("{value:#?}"),
        Err(error) => println!("{error:#?}"),
    }

    Ok(())
}
