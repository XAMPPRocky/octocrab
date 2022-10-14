use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let member_list = octocrab.teams("wpmedia").members("AppSec").send().await;

    println!("Members: {:?}", member_list);

    Ok(())
}
