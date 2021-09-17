use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let content = octocrab
        .repos("rust-lang", "rust")
        .get_content()
        .path(String::from("CONTRIBUTING.md"))
        .r#ref("master")
        .send()
        .await?;

    println!("{} has SHA {}", content.name, content.sha);

    Ok(())
}
