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
