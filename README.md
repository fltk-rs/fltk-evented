# fltk-evented

This crate provides a Listener widget which can basically wrap any fltk-rs widget (implementing WidgetBase and WidgetExt) and provides an `on_<event>` interface.

## Usage
```toml
[dependencies]
fltk = "1.2"
fltk-evented = "0.1"
```

## Example
```rust
use fltk::{
    app, button,
    enums::{Color, FrameType},
    prelude::*,
    window,
};
use fltk_evented::Listener;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);
    wind.set_color(Color::White);
    let mut but: Listener<_> = button::Button::new(160, 210, 80, 35, "Click me!").into();
    but.set_frame(FrameType::FlatBox);
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

    but.on_click(|b| {
        println!("Clicked");
        b.set_label_color(Color::White);
    });

    but.on_release(move |b| {
        wind.set_label("Button Released!");
        b.set_label_color(Color::Black);
    });

    app.run().unwrap();
}
```

Another aspect of the Listener widget is that it can be queried on whether it was triggered in the event loop, negating the need for a callback, and the hassle of sending difficult to move types to the closure.

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

    while a.wait() {
        if but_inc.triggered() {
            val += 1;
        }

        if but_dec.triggered() {
            val -= 1;
        }

        frame.set_label(&val.to_string());
    }
}
```

You can also check for other events in the event loop:
```rust
use fltk::{app, button::Button, enums::Color, frame::Frame, group::Flex, prelude::*, window::Window};
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
    let mut but_inc: Listener<_> = Button::default().with_label("+").into();
    let mut frame = Frame::default();
    let mut but_dec: Listener<_> = Button::default().with_label("-").into();
    flex.end();
    wind.end();
    wind.show();

    but_inc.on_hover(|b| {
        b.set_color(Color::Red);
    });

    let mut val = 0;

    while a.wait() {
        if but_inc.triggered() {
            val += 1;
        }

        if but_inc.hovered() {
            but_inc.set_color(Color::White);
        }

        if but_inc.left() {
            but_inc.set_color(Color::BackGround);
        }

        if but_dec.triggered() {
            val -= 1;
        }

        if but_dec.hovered() {
            but_dec.set_color(Color::White);
        }

        if but_dec.left() {
            but_dec.set_color(Color::BackGround);
        }

        frame.set_label(&val.to_string());
    }
}
```