#[cfg(target_arch = "wasm32")]
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use http::{header::USER_AGENT, Uri};
#[cfg(target_arch = "wasm32")]
use octocrab::service::middleware::base_uri::BaseUriLayer;
#[cfg(target_arch = "wasm32")]
use octocrab::service::middleware::extra_headers::ExtraHeadersLayer;
#[cfg(target_arch = "wasm32")]
use octocrab::service::wasm::ReqwestService;
#[cfg(target_arch = "wasm32")]
use octocrab::{AuthState, Octocrab, OctocrabBuilder};

#[cfg(target_arch = "wasm32")]
fn build_octocrab() -> octocrab::Result<Octocrab, std::convert::Infallible> {
    OctocrabBuilder::new_empty()
        .with_service(ReqwestService::new())
        .with_layer(&BaseUriLayer::new(Uri::from_static(
            "https://api.github.com",
        )))
        .with_layer(&ExtraHeadersLayer::new(Arc::new(vec![(
            USER_AGENT,
            "octocrab".parse().unwrap(),
        )])))
        .with_auth(AuthState::None)
        .build()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let _octocrab = build_octocrab().expect("build wasm client");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
