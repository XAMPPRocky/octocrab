use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let created = octocrab
        .issues("owner", "repo")
        // Parent issue configuration
        .create("Parent issue title")
        .body("Parent issue body")
        .labels(vec!["bug".to_string(), "enhancement".to_string()])
        .assignees(vec!["octocat".to_string()])
        // First sub-issue: labels/assignees above applied to parent,
        // .body/.labels/.assignees below apply to this sub-issue
        .add_sub_issue("Sub-issue A")
        .body("Body for sub-issue A")
        .labels(vec!["good-first-issue".to_string()])
        .assignees(vec!["octocat".to_string()])
        // Second sub-issue
        .add_sub_issue("Sub-issue B")
        .body("Body for sub-issue B")
        .labels(vec!["documentation".to_string()])
        .send()
        .await?;

    println!("Parent issue: {}", created.parent.html_url);
    for sub in &created.sub_issues {
        println!("Sub-issue: {}", sub.html_url);
    }

    Ok(())
}
