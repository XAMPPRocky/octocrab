pub mod middleware;

#[cfg(all(target_arch = "wasm32", feature = "wasm-client"))]
#[cfg_attr(docsrs, doc(cfg(all(target_arch = "wasm32", feature = "wasm-client"))))]
pub mod wasm;
