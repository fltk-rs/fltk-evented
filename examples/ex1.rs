use fltk::{enums::Color, prelude::*, app, window, button};
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
