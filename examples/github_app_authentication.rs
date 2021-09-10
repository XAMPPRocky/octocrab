use octocrab::models::{CreateInstallationAccessToken, Installation, InstallationToken};
use octocrab::{create_authenticate_as_app_jwt, Octocrab};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let app_id = read_env_var("GITHUB_APP_ID");
    let app_private_key = read_env_var("GITHUB_APP_PRIVATE_KEY");

    let octocrab = Octocrab::builder()
        .personal_token(create_authenticate_as_app_jwt(&app_id, &app_private_key).unwrap())
        .build()?;

    let installations: Vec<Installation> = octocrab
        .get("/app/installations", None::<&()>)
        .await
        .unwrap();

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

    let comment = octocrab
        .issues("XAMPPRocky", "octocrab")
        .create_comment(0, "Created programmatically!")
        .await
        .unwrap();

    dbg!(comment);

    Ok(())
}

fn read_env_var(var_name: &str) -> String {
    let err = format!("Missing environment variable: {}", var_name);
    std::env::var(var_name).expect(&err)
}
