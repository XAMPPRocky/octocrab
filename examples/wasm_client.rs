#[cfg(target_arch = "wasm32")]
mod wasm_client {
    use std::sync::Arc;

    use http::{header::USER_AGENT, Uri};
    use octocrab::service::middleware::base_uri::BaseUriLayer;
    use octocrab::service::middleware::extra_headers::ExtraHeadersLayer;
    use octocrab::service::wasm::ReqwestService;
    use octocrab::{AuthState, Octocrab, OctocrabBuilder};

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

    pub fn run() {
        let _octocrab = build_octocrab().expect("build wasm client");
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_client::run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
}
