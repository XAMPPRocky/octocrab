std::cfg_select! {
    target_arch = "wasm32" => {
        fn main() {
            let _octocrab = octocrab::Octocrab::builder()
                .build()
                .expect("build wasm client");
        }
    }
    _ => {
        fn main() {}
    }
}
