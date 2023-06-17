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
