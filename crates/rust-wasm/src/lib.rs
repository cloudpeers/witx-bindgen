pub use async_trait::async_trait;
pub use handle::Handle;
pub use wai_bindgen_rust_impl::{export, import};

//#[cfg(target_family = "wasm")]
#[path = "buffer/exports.rs"]
pub mod exports;

#[cfg(target_family = "wasm")]
#[path = "futures/wasm.rs"]
mod futures;
#[cfg(not(target_family = "wasm"))]
#[path = "futures/native.rs"]
mod futures;

//#[cfg(target_family = "wasm")]
#[path = "buffer/imports.rs"]
pub mod imports;

#[cfg(target_family = "wasm")]
#[path = "handle/wasm.rs"]
pub mod handle;
#[cfg(not(target_family = "wasm"))]
#[path = "handle/native.rs"]
pub mod handle;

#[cfg(target_family = "wasm")]
#[path = "runtime/wasm.rs"]
#[doc(hidden)]
pub mod rt;
#[cfg(not(target_family = "wasm"))]
#[path = "runtime/native.rs"]
#[doc(hidden)]
pub mod rt;
