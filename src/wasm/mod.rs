//! WASM support for octocrab
//!
//! This module provides the necessary infrastructure to use octocrab in WebAssembly environments.
//! The main issue with using octocrab in WASM is that reqwest's futures are not `Send` in WASM,
//! which conflicts with octocrab's requirements. This module works around this by providing a
//! custom Tower service implementation that properly handles WASM's single-threaded nature.
//!
//! # Usage
//!
//! To use octocrab in a WASM environment:
//!
//! 1. Add octocrab to your Cargo.toml with default features disabled:
//!    ```toml
//!    octocrab = { version = "0.49", default-features = false }
//!    ```
//!
//! 2. Use the `wasm_builder()` function to create an octocrab instance:
//!    ```no_run
//!    # #[cfg(target_arch = "wasm32")]
//!    # async fn example() -> octocrab::Result<()> {
//!    let octocrab = octocrab::wasm::wasm_builder()
//!        .build()?;
//!    # Ok(())
//!    # }
//!    ```
//!
//! # Features
//!
//! This module is only available when compiling for the `wasm32` target architecture.
//! It will not be included in non-WASM builds.

mod reqwest_tower_service;
mod wasm;

pub use reqwest_tower_service::{ReqwestTowerError, ReqwestTowerService};
pub use wasm::wasm_builder;
