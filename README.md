# fltk-evented

This crate provides a Listener widget which can basically wrap any fltk-rs widget (implementing WidgetBase and WidgetExt) and provides an `on_<event>` interface.

## Usage
```toml
fltk = "1.2"
fltk-evented = "0.1"
```

## Example
```rust
use fltk::{prelude::*, enums::Color, *};
use fltk_evented::Listener;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(400, 300);
    wind.set_color(Color::White);
    let mut but: Listener<_> = button::Button::new(160, 210, 80, 40, "Click me!").into();
    but.set_color(Color::Cyan);
    but.set_selection_color(Color::Cyan.darker());
    but.clear_visible_focus();
    wind.end();
    wind.show();

    but.on_hover(|b| {
        b.set_color(Color::Cyan.lighter().lighter());
    });

    but.on_leave(|b| {
        b.set_color(Color::Cyan);
    });

    but.on_click(|_| {
        println!("Clicked");
    });

    app.run().unwrap();
}
```