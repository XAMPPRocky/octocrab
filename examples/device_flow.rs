use std::time::Duration;
const CLIENT_ID: &'static str = "********************";

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let codes = octocrab::auth::authenticate_as_device(
        secrecy::Secret::from(String::from(CLIENT_ID)),
        ["public_repo", "read:org"],
    )
    .await?;
    println!(
        "Go to {} and enter code {}",
        codes.verification_uri, codes.user_code
    );
    let mut interval = Duration::from_secs(codes.interval);
    let mut clock = tokio::time::interval(interval);
    let auth = loop {
        clock.tick().await;
        match codes.poll_once().await? {
            Ok(auth) => break auth,
            Err(cont) => match cont {
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
    return Ok(());
}
