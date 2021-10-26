use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let repo = octocrab.repos("rust-lang", "rust").get().await?;

    println!(
        "{} has {} stars",
        repo.full_name.unwrap(),
        repo.stargazers_count.unwrap_or(0)
    );

    Ok(())
}
