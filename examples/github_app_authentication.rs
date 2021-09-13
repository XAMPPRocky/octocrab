use octocrab::models::{InstallationToken, Repository};
use octocrab::params::apps::CreateInstallationAccessToken;
use octocrab::{create_jwt, Octocrab};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let app_id = read_env_var("GITHUB_APP_ID");
    let app_private_key = read_env_var("GITHUB_APP_PRIVATE_KEY");

    let token = create_jwt(app_id.parse::<u64>().unwrap().into(), app_private_key).unwrap();

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let installations = octocrab
        .apps()
        .installations()
        .send()
        .await
        .unwrap()
        .take_items();

    let mut create_access_token = CreateInstallationAccessToken::default();
    create_access_token.repositories = vec!["octocrab".to_string()];

    let access: InstallationToken = octocrab
        .post(
            installations[0].access_tokens_url.as_ref().unwrap(),
            Some(&create_access_token),
        )
        .await
        .unwrap();

    let octocrab = octocrab::OctocrabBuilder::new()
        .personal_token(access.token)
        .build()
        .unwrap();

    let _repos: Vec<Repository> = octocrab
        .get("/installation/repositories", None::<&()>)
        .await
        .unwrap();

    Ok(())
}

fn read_env_var(var_name: &str) -> String {
    let err = format!("Missing environment variable: {}", var_name);
    std::env::var(var_name).expect(&err)
}
