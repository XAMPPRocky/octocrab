use octocrab::{models::issues::SubIssuePriority, Octocrab};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;
    let issues = octocrab.issues("owner", "repo");

    // Create a normal parent issue
    let parent = issues
        .create("Example parent for manage_sub_issues")
        .body("This is the parent issue")
        .send()
        .await?;

    // Create two normal child issues separately
    let sub_a = issues
        .create("Sub-issue A")
        .body("First child issue")
        .send()
        .await?;

    let sub_b = issues
        .create("Sub-issue B")
        .body("Second child issue")
        .send()
        .await?;

    // Link both as sub-issues to the parent
    issues.add_sub_issue(parent.number, sub_a.id, None).await?;
    println!("Linked issue #{} as sub-issue", sub_a.number);

    issues.add_sub_issue(parent.number, sub_b.id, None).await?;
    println!("Linked issue #{} as sub-issue", sub_b.number);

    // List sub-issues
    let list = issues
        .list_sub_issues(parent.number)
        .per_page(10)
        .send()
        .await?;
    println!("Listed {} sub-issues", list.items.len());

    // Get parent of a sub-issue
    let parent_of_a = issues.get_parent_issue(sub_a.number).await?;
    println!(
        "Parent of sub-issue #{} is issue #{}",
        sub_a.number, parent_of_a.number
    );

    // Reprioritize sub-issue B to be after sub-issue A
    issues
        .reprioritize_sub_issue(parent.number, sub_b.id, SubIssuePriority::After(sub_a.id))
        .await?;
    println!(
        "Reprioritized sub-issue #{} after #{}",
        sub_b.number, sub_a.number
    );

    // Remove sub-issue A link
    issues.remove_sub_issue(parent.number, sub_a.id).await?;
    println!("Removed sub-issue #{}", sub_a.number);

    // Link it back as an existing sub-issue
    issues.add_sub_issue(parent.number, sub_a.id, None).await?;
    println!("Re-linked existing issue #{} as sub-issue", sub_a.number);

    Ok(())
}
