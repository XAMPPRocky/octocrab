use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let app_id = read_env_var("GITHUB_APP_ID").parse::<u64>().unwrap().into();
    let app_private_key = read_env_var("GITHUB_APP_PRIVATE_KEY");

    let octocrab = Octocrab::builder()
        .app_from_rsa_pem(app_id, app_private_key)?
        .build()?;
    let _installations = octocrab.apps().installations().send().await.unwrap();

    Ok(())
}

fn read_env_var(var_name: &str) -> String {
    let err = format!("Missing environment variable: {var_name}");
    std::env::var(var_name).expect(&err)
}
