#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "tokio", doc = concat!(r##"
## Async usage
This crate provides an AsyncListener which can be used in async contexts. This requires enabling either the tokio or async-std features. You can check the examples directory for an example on usage.

```rust
"##,
include_str!("../examples/async_examples/tokio_ex/src/main.rs"),
r##"
```
"##))]
#![allow(clippy::needless_doctest_main)]

mod base;
pub use base::BaseListener;

mod blocking;
pub use blocking::Listener;

pub fn event() -> bool {
    fltk::app::event() != fltk::enums::Event::NoEvent
}

#[cfg(all(feature = "tokio", feature = "async-std"))]
compile_error!("Features `tokio` and `async-std` are mutually exclusive.");

#[cfg(any(feature = "tokio", feature = "async-std"))]
mod asynch;
#[cfg(any(feature = "tokio", feature = "async-std"))]
pub use asynch::AsyncListener;
