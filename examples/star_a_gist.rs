use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let gist_id = if let Some(gist_id) = std::env::args().nth(1) {
        gist_id
    } else {
        eprintln!("error: Need to pass gist id on argv");
        std::process::exit(1);
    };

    let octocrab = Octocrab::builder().personal_token(token).build()?;
    octocrab.gists().star(gist_id).await?;
    Ok(())
}
