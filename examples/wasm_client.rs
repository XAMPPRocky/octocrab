#[cfg(target_arch = "wasm32")]
fn main() {
    let _octocrab = octocrab::Octocrab::builder()
        .build()
        .expect("build wasm client");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
