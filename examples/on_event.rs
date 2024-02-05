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