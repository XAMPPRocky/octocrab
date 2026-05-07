cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
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

        fn main() {
            let _octocrab = build_octocrab().expect("build wasm client");
        }
    } else {
        fn main() {}
    }
}
