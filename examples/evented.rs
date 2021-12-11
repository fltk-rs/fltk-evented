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
