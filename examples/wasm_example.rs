//! This example demonstrates how to use octocrab in a WASM environment.
//!
//! To compile this for WASM, run:
//! ```sh
//! rustup target add wasm32-unknown-unknown
//! cargo build --target wasm32-unknown-unknown --example wasm_example
//! ```
//!
//! Note: This example is designed to show the API usage. To actually run it,
//! you would need to integrate it into a web application using a framework
//! like Yew, Leptos, or similar.

#[cfg(target_arch = "wasm32")]
use octocrab::wasm::wasm_builder;

#[cfg(target_arch = "wasm32")]
pub async fn example_usage() -> octocrab::Result<()> {
    // Create an octocrab instance for WASM
    let mut octocrab = wasm_builder().build()?;

    // Optionally add authentication
    // If you have a GitHub token, you can authenticate like this:
    // octocrab = octocrab.user_access_token("your_github_token".to_string())?;

    // Now you can use octocrab as you normally would!
    
    // Example: Get information about a repository
    let repo = octocrab
        .repos("XAMPPRocky", "octocrab")
        .get()
        .await?;
    
    println!("Repository: {}", repo.name);
    println!("Stars: {}", repo.stargazers_count.unwrap_or(0));
    println!("Description: {}", repo.description.unwrap_or_default());

    // Example: List issues
    let issues = octocrab
        .issues("XAMPPRocky", "octocrab")
        .list()
        .per_page(10)
        .send()
        .await?;

    println!("Found {} issues", issues.items.len());

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[cfg(target_arch = "wasm32")]
fn main() {
    // In a real WASM application, you would spawn this using your framework's
    // async runtime. For example, in Yew you might use:
    // wasm_bindgen_futures::spawn_local(async {
    //     example_usage().await.expect("Failed to run example");
    // });
    
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = example_usage().await {
            eprintln!("Error: {}", e);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This example is only meant to be compiled for WASM targets.");
    println!("Run: cargo build --target wasm32-unknown-unknown --example wasm_example");
}
