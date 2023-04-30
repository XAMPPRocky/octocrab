use either::Either;
use http::header::ACCEPT;
use std::time::Duration;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let client_id = secrecy::Secret::from(std::env::var("GITHUB_CLIENT_ID").unwrap());
    let crab = octocrab::Octocrab::builder()
        .base_uri("https://github.com")?
        .add_header(ACCEPT, "application/json".to_string())
        .build()?;

    let codes = crab
        .authenticate_as_device(&client_id, ["public_repo", "read:org"])
        .await?;
    println!(
        "Go to {} and enter code {}",
        codes.verification_uri, codes.user_code
    );
    let mut interval = Duration::from_secs(codes.interval);
    let mut clock = tokio::time::interval(interval);
    let auth = loop {
        clock.tick().await;
        match codes.poll_once(&crab, &client_id).await? {
            Either::Left(auth) => break auth,
            Either::Right(cont) => match cont {
                octocrab::auth::Continue::SlowDown => {
                    // We were request to slow down. We add five seconds to the polling
                    // duration.
                    interval += Duration::from_secs(5);
                    clock = tokio::time::interval(interval);
                    // The first tick happens instantly, so we tick that off immediately.
                    clock.tick().await;
                }
                octocrab::auth::Continue::AuthorizationPending => {
                    // The user has not clicked authorize yet, but nothing has gone wrong.
                    // We keep polling.
                }
            },
        }
    };

    println!("Authorization succeeded with access to {:?}", auth.scope);
    Ok(())
}
