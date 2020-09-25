use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::builder()
        .personal_token(env!("GITHUB_TOKEN").to_string())
        .build()?;

    let x = octocrab.activity().notifications().list_notifications().await?;
    println!("count: {}", x.len());

    Ok(())
}
