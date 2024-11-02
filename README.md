# fltk-evented

This crate provides Listener widgets both for sync and async
 which can basically wrap any fltk-rs widget (implementing [WidgetBase](fltk::prelude::WidgetBase) and [WidgetExt](fltk::prelude::WidgetExt))
 and provides methods `triggered() -> bool` and `event() -> Event` to handle events in the event loop, without requiring callbacks.
 It also provides `on_<event>(callback)` methods which simplify handling events whereas you would've had to use the widget's handle method directly. 

## Usage
```toml
[dependencies]
fltk = "1.4"
fltk-evented = "0.5"
```

## Examples

```rust,no_run
use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
use fltk_evented::Listener;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_font_size(20);

    let mut count = 0;

    let mut wind = Window::default()
        .with_size(160, 200)
        .with_label("Counter");
    let flex = Flex::default()
        .with_size(120, 160)
        .center_of_parent()
        .column();
    let but_inc: Listener<_> = Button::default().with_label("+").into();
    let mut frame = Frame::default().with_label(&count.to_string());
    let but_dec: Listener<_> = Button::default().with_label("-").into();
    flex.end();
    wind.end();
    wind.show();

    while a.wait() {
        if fltk_evented::event() {
            if but_inc.triggered() {
                count += 1;
            }
            
            if but_dec.triggered() {
                count -= 1;
            }

            frame.set_label(&count.to_string());
        }
    }
}
```

```rust,no_run
use fltk::{
    app,
    button::Button,
    enums::{Color, Event},
    frame::Frame,
    group::Flex,
    prelude::*,
    window::Window,
};
use fltk_evented::Listener;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_font_size(20);

    let mut count = 0;

    let mut wind = Window::default()
        .with_size(160, 200)
        .with_label("Counter");
    let flex = Flex::default()
        .with_size(120, 160)
        .center_of_parent()
        .column();
    let mut but_inc: Listener<_> = Button::default().with_label("+").into();
    let mut frame = Frame::default().with_label(&count.to_string());
    let mut but_dec: Listener<_> = Button::default().with_label("-").into();
    flex.end();
    wind.end();
    wind.show();

    fn button_color(btn: &mut Button, c: Color) {
        btn.set_color(c);
        btn.redraw();
    }

    while a.wait() {
        if fltk_evented::event() {
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
}
```

Using `on_<event>` methods:
```rust,no_run
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

## Async Examples
fltk-evented can be used with either tokio or async-std to handle non-blocking async calls in the event loop. The following examples shows usage with tokio. Another example using async-std can be found in the examples directory:
```rust,ignore
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