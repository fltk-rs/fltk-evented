#![doc = include_str!("../README.md")]

#![cfg_attr(feature = "tokio", doc = r##"
## Async usage
This crate provides an AsyncListener which can be used in async contexts. This requires enabling either the tokio or async-std features. You can check the examples directory for an example on usage.

```rust
use fltk::{prelude::*, *};
use fltk_evented::AsyncListener;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut buf = text::TextBuffer::default();
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_font_size(20);

    let mut wind = window::Window::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("Counter");
    let col = group::Pack::default()
        .with_size(400, 300)
        .center_of_parent()
        .with_type(group::PackType::Vertical);
    let mut editor = text::TextEditor::default().with_size(0, 240);
    editor.set_buffer(buf.clone());
    let getter: AsyncListener<_> = button::Button::default()
        .with_label("Get")
        .with_size(0, 60)
        .into();
    col.end();
    wind.end();
    wind.show();

    while a.wait() {
        if getter.triggered().await {
            let text = reqwest::get("https://www.rust-lang.org")
                .await?
                .text()
                .await?;
            buf.set_text(&text);
        }
    }
    Ok(())
}
```
"##)]

#![allow(clippy::needless_doctest_main)]

mod base;
pub use base::BaseListener;

mod blocking;
pub use blocking::Listener;

#[cfg(any(feature = "tokio", feature = "async-std"))]
mod asynch;
#[cfg(any(feature = "tokio", feature = "async-std"))]
pub use asynch::AsyncListener;
