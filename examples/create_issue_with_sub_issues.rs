use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let created = octocrab
        .issues("owner", "repo")
        .create("Parent")
        // Parent labels/assignees apply to the parent issue.
        .labels(vec!["bug".to_string(), "enhancement".to_string()])
        // New sub-issue of Parent
        .add_sub_issue("Sub-issue A", |sub| {
            sub.body("Body for sub-issue A")
                .labels(vec!["good-first-issue".to_string()])
                // Nested sub-issue
                .add_sub_issue("Nested under A", |nested| {
                    nested
                        .body("Body for the nested sub-issue")
                        .assignees(vec!["ahmed-mekky".to_string()])
                })
        })
        // Another Parent sub-issue
        .add_sub_issue("Sub-issue B", |sub| {
            sub.body("Body for sub-issue B")
                .assignees(vec!["ahmed-mekky".to_string()])
                .replace_parent(true)
        })
        .body("Parent issue body")
        .send()
        .await?;

    println!("Parent issue: {}", created.html_url);

    Ok(())
}
