# fltk-evented

This crate provides Listener widgets both for sync and async
 which can basically wrap any fltk-rs widget (implementing [WidgetBase](fltk::prelude::WidgetBase) and [WidgetExt](fltk::prelude::WidgetExt))
 and provides methods `triggered() -> bool`, `event() -> Event` and `on_<event>(callback)`to handle events, without requiring callbacks.

## Usage
```toml
[dependencies]
fltk = "1.4"
fltk-evented = "0.5"
```

## Example

```rust
use fltk::{
    prelude::*, 
    app,
    enums::{Color, Event},
    button::Button, frame::Frame, group::Flex, window::Window,
};
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

        fn button_color(btn: &mut Button, c: Color) {
            btn.set_color(c);
            btn.redraw();
        }

        match but_inc.event() {
            Event::Enter | Event::Move => button_color(&mut but_inc, Color::White),
            Event::Leave => button_color(&mut but_inc, Color::BackGround),
            _ => (),
        }

        match but_dec.event() {
            Event::Enter | Event::Move => button_color(&mut but_dec, Color::White),
            Event::Leave => button_color(&mut but_dec, Color::BackGround),
            _ => (),
        }
    }
}
```

Using `on_<event>` methods:
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