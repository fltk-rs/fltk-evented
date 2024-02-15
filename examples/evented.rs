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
            if but_inc.triggered() {
                count += 1;
            }
    
            if but_dec.triggered() {
                count -= 1;
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

            frame.set_label(&count.to_string());
        }
    }
}
