use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::builder()
        .personal_token(env!("GITHUB_TOKEN").to_string())
        .build()?;

    let notifications = octocrab.activity().notifications().list().send().await?;
    for n in notifications {
        println!("unread notification: {}", n.subject.title);
    }

    Ok(())
}
