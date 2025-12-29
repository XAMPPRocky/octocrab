use crate::wasm::reqwest_tower_service::ReqwestTowerService;
use crate::{AuthState, LayerReady, NoConfig};

/// Creates an OctocrabBuilder pre-configured for WASM environments.
///
/// This builder is automatically configured with:
/// - A reqwest-based Tower service that works in WASM
/// - The GitHub API base URL (https://api.github.com)
/// - The wasm-bindgen-futures executor for spawning local tasks
/// - No authentication by default (use `.with_auth()` to add)
///
/// # Example
///
/// ```no_run
/// # #[cfg(target_arch = "wasm32")]
/// # async fn example() -> octocrab::Result<()> {
/// let mut octocrab = octocrab::wasm::wasm_builder()
///     .build()?;
///
/// // Optionally add authentication
/// octocrab = octocrab.user_access_token("your_token".to_string())?;
///
/// // Now use octocrab as normal
/// let repos = octocrab.current().list_repos_for_authenticated_user().send().await?;
/// # Ok(())
/// # }
/// ```
pub fn wasm_builder(
) -> crate::OctocrabBuilder<ReqwestTowerService, NoConfig, AuthState, LayerReady> {
    let reqwest_client = ReqwestTowerService {
        base_url: Some(("https".parse().unwrap(), "api.github.com".parse().unwrap())),
        client: reqwest::Client::new(),
    };

    let builder = crate::OctocrabBuilder::new_empty()
        .with_service(reqwest_client)
        .with_executor(Box::new(wasm_bindgen_futures::spawn_local))
        .with_auth(AuthState::None);

    builder
}
