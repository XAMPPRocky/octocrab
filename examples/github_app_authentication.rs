use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let app_id = read_env_var("GITHUB_APP_ID").parse::<u64>().unwrap().into();
    let app_private_key = read_env_var("GITHUB_APP_PRIVATE_KEY");
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(app_private_key.as_bytes()).unwrap();
    let installation_id = read_env_var("GITHUB_INSTALLATION_ID")
        .parse::<u64>()
        .unwrap()
        .into();
    let owner = read_env_var("GITHUB_OWNER");
    let repo = read_env_var("GITHUB_REPO");

    let octocrab = Octocrab::builder()
        .app(app_id, key)
        .build()?
        .installation(installation_id)?;

    let _content = octocrab
        .repos(owner, repo)
        .get_content()
        .path("README.md")
        .send()
        .await?;

    Ok(())
}

fn read_env_var(var_name: &str) -> String {
    let err = format!("Missing environment variable: {var_name}");
    std::env::var(var_name).expect(&err)
}
