# fltk-evented

This crate provides a Listener widget which can basically wrap any fltk-rs widget (implementing WidgetBase and WidgetExt) and provides an `triggered` interface, without requiring callbacks.

## Usage
```toml
[dependencies]
fltk = "1.2"
fltk-evented = "0.2"
```

## Example

```rust
use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
use fltk_evented::Listener;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_font_size(20);

    let mut wind = Window::default()
        .with_size(160, 200)
        .center_screen()
        .with_label("Counter");
    let flex = Flex::default()
        .with_size(120, 160)
        .center_of_parent()
        .column();
    let but_inc: Listener<_> = Button::default().with_label("+").into();
    let mut frame = Frame::default();
    let but_dec: Listener<_> = Button::default().with_label("-").into();
    flex.end();
    wind.end();
    wind.show();

    let mut val = 0;
    frame.set_label(&val.to_string());

    while a.wait() {
        if but_inc.triggered() {
            val += 1;
            frame.set_label(&val.to_string());
        }

        if but_dec.triggered() {
            val -= 1;
            frame.set_label(&val.to_string());
        }
    }
}
```

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