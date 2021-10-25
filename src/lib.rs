/*!
# fltk-evented

This crate provides a Listener widget which can basically wrap any fltk-rs widget (implementing WidgetBase and WidgetExt) and provides an `on_<event>` interface.

## Usage
```toml,no_run
fltk = "1.2"
fltk-evented = "0.1"
```

## Example
```rust,no_run
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
*/

#![allow(clippy::needless_doctest_main)]

use fltk::{
    enums::{Align, Event},
    prelude::{WidgetBase, WidgetExt, WidgetType},
};
use std::cell::RefCell;
use std::rc::Rc;

type WidgetCb<T> = Rc<RefCell<Option<Box<dyn FnMut(&mut T)>>>>;

/// A listener widget
#[derive(Default, Clone)]
pub struct Listener<T: WidgetBase + WidgetExt> {
    #[allow(dead_code)]
    wid: T,
    hover_cb: WidgetCb<T>,
    leave_cb: WidgetCb<T>,
    push_cb: WidgetCb<T>,
    released_cb: WidgetCb<T>,
    drag_cb: WidgetCb<T>,
    focus_cb: WidgetCb<T>,
    unfocus_cb: WidgetCb<T>,
    keydown_cb: WidgetCb<T>,
    keyup_cb: WidgetCb<T>,
    close_cb: WidgetCb<T>,
    move_cb: WidgetCb<T>,
    shortcut_cb: WidgetCb<T>,
    deactivate_cb: WidgetCb<T>,
    activate_cb: WidgetCb<T>,
    hide_cb: WidgetCb<T>,
    show_cb: WidgetCb<T>,
    paste_cb: WidgetCb<T>,
    selection_clear_cb: WidgetCb<T>,
    mouse_wheel_cb: WidgetCb<T>,
    dnd_enter_cb: WidgetCb<T>,
    dnd_drag_cb: WidgetCb<T>,
    dnd_leave_cb: WidgetCb<T>,
    dnd_release_cb: WidgetCb<T>,
    screen_config_changed_cb: WidgetCb<T>,
    fullscreen_cb: WidgetCb<T>,
    zoom_gesture_cb: WidgetCb<T>,
    zoom_event_cb: WidgetCb<T>,
    resize_cb: WidgetCb<T>,
}

impl<T: WidgetBase + WidgetExt> std::ops::Deref for Listener<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl<T: WidgetBase + WidgetExt> std::ops::DerefMut for Listener<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}

/// The listener widget's implementation
impl<T: WidgetBase + WidgetExt + Default + 'static> Listener<T> {
    pub fn new<S: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: S) -> Self {
        let mut wid = T::new(x, y, w, h, label);
        let hover_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let leave_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let push_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let released_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let drag_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let focus_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let unfocus_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let keydown_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let keyup_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let close_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let move_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let shortcut_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let deactivate_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let activate_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let hide_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let show_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let paste_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let selection_clear_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let mouse_wheel_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let dnd_enter_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let dnd_drag_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let dnd_leave_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let dnd_release_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let screen_config_changed_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let fullscreen_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let zoom_gesture_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let zoom_event_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let resize_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        wid.handle({
            let hover_cb = hover_cb.clone();
            let leave_cb = leave_cb.clone();
            let push_cb = push_cb.clone();
            let released_cb = released_cb.clone();
            let drag_cb = drag_cb.clone();
            let focus_cb = focus_cb.clone();
            let unfocus_cb = unfocus_cb.clone();
            let keydown_cb = keydown_cb.clone();
            let keyup_cb = keyup_cb.clone();
            let close_cb = close_cb.clone();
            let move_cb = move_cb.clone();
            let shortcut_cb = shortcut_cb.clone();
            let deactivate_cb = deactivate_cb.clone();
            let activate_cb = activate_cb.clone();
            let hide_cb = hide_cb.clone();
            let show_cb = show_cb.clone();
            let paste_cb = paste_cb.clone();
            let selection_clear_cb = selection_clear_cb.clone();
            let mouse_wheel_cb = mouse_wheel_cb.clone();
            let dnd_enter_cb = dnd_enter_cb.clone();
            let dnd_drag_cb = dnd_drag_cb.clone();
            let dnd_leave_cb = dnd_leave_cb.clone();
            let dnd_release_cb = dnd_release_cb.clone();
            let screen_config_changed_cb = screen_config_changed_cb.clone();
            let fullscreen_cb = fullscreen_cb.clone();
            let zoom_gesture_cb = zoom_gesture_cb.clone();
            let zoom_event_cb = zoom_event_cb.clone();
            let resize_cb = resize_cb.clone();
            move |b, ev| match ev {
                Event::Enter => {
                    if let Some(cb) = hover_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Leave => {
                    if let Some(cb) = leave_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Push => {
                    if let Some(cb) = push_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Released => {
                    if let Some(cb) = released_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Drag => {
                    if let Some(cb) = drag_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Focus => {
                    if let Some(cb) = focus_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Unfocus => {
                    if let Some(cb) = unfocus_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::KeyDown => {
                    if let Some(cb) = keydown_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::KeyUp => {
                    if let Some(cb) = keyup_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Close => {
                    if let Some(cb) = close_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Move => {
                    if let Some(cb) = move_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Shortcut => {
                    if let Some(cb) = shortcut_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Deactivate => {
                    if let Some(cb) = deactivate_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Activate => {
                    if let Some(cb) = activate_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Hide => {
                    if let Some(cb) = hide_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Show => {
                    if let Some(cb) = show_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Paste => {
                    if let Some(cb) = paste_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::SelectionClear => {
                    if let Some(cb) = selection_clear_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::MouseWheel => {
                    if let Some(cb) = mouse_wheel_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::DndEnter => {
                    if let Some(cb) = dnd_enter_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::DndDrag => {
                    if let Some(cb) = dnd_drag_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::DndLeave => {
                    if let Some(cb) = dnd_leave_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::DndRelease => {
                    if let Some(cb) = dnd_release_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::ScreenConfigChanged => {
                    if let Some(cb) = screen_config_changed_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Fullscreen => {
                    if let Some(cb) = fullscreen_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::ZoomGesture => {
                    if let Some(cb) = zoom_gesture_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::ZoomEvent => {
                    if let Some(cb) = zoom_event_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Resize => {
                    if let Some(cb) = resize_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                _ => false,
            }
        });
        Self {
            wid,
            hover_cb,
            leave_cb,
            push_cb,
            released_cb,
            drag_cb,
            focus_cb,
            unfocus_cb,
            keydown_cb,
            keyup_cb,
            close_cb,
            move_cb,
            shortcut_cb,
            deactivate_cb,
            activate_cb,
            hide_cb,
            show_cb,
            paste_cb,
            selection_clear_cb,
            mouse_wheel_cb,
            dnd_enter_cb,
            dnd_drag_cb,
            dnd_leave_cb,
            dnd_release_cb,
            screen_config_changed_cb,
            fullscreen_cb,
            zoom_gesture_cb,
            zoom_event_cb,
            resize_cb,
        }
    }

    fn from_widget(mut wid: T) -> Self {
        let hover_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let leave_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let push_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let released_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let drag_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let focus_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let unfocus_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let keydown_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let keyup_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let close_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let move_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let shortcut_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let deactivate_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let activate_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let hide_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let show_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let paste_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let selection_clear_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let mouse_wheel_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let dnd_enter_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let dnd_drag_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let dnd_leave_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let dnd_release_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let screen_config_changed_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let fullscreen_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let zoom_gesture_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let zoom_event_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        let resize_cb: WidgetCb<T> = Rc::from(RefCell::from(None));
        wid.handle({
            let hover_cb = hover_cb.clone();
            let leave_cb = leave_cb.clone();
            let push_cb = push_cb.clone();
            let released_cb = released_cb.clone();
            let drag_cb = drag_cb.clone();
            let focus_cb = focus_cb.clone();
            let unfocus_cb = unfocus_cb.clone();
            let keydown_cb = keydown_cb.clone();
            let keyup_cb = keyup_cb.clone();
            let close_cb = close_cb.clone();
            let move_cb = move_cb.clone();
            let shortcut_cb = shortcut_cb.clone();
            let deactivate_cb = deactivate_cb.clone();
            let activate_cb = activate_cb.clone();
            let hide_cb = hide_cb.clone();
            let show_cb = show_cb.clone();
            let paste_cb = paste_cb.clone();
            let selection_clear_cb = selection_clear_cb.clone();
            let mouse_wheel_cb = mouse_wheel_cb.clone();
            let dnd_enter_cb = dnd_enter_cb.clone();
            let dnd_drag_cb = dnd_drag_cb.clone();
            let dnd_leave_cb = dnd_leave_cb.clone();
            let dnd_release_cb = dnd_release_cb.clone();
            let screen_config_changed_cb = screen_config_changed_cb.clone();
            let fullscreen_cb = fullscreen_cb.clone();
            let zoom_gesture_cb = zoom_gesture_cb.clone();
            let zoom_event_cb = zoom_event_cb.clone();
            let resize_cb = resize_cb.clone();
            move |b, ev| match ev {
                Event::Enter => {
                    if let Some(cb) = hover_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Leave => {
                    if let Some(cb) = leave_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Push => {
                    if let Some(cb) = push_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Released => {
                    if let Some(cb) = released_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Drag => {
                    if let Some(cb) = drag_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Focus => {
                    if let Some(cb) = focus_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Unfocus => {
                    if let Some(cb) = unfocus_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::KeyDown => {
                    if let Some(cb) = keydown_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::KeyUp => {
                    if let Some(cb) = keyup_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Close => {
                    if let Some(cb) = close_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Move => {
                    if let Some(cb) = move_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Shortcut => {
                    if let Some(cb) = shortcut_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Deactivate => {
                    if let Some(cb) = deactivate_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Activate => {
                    if let Some(cb) = activate_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Hide => {
                    if let Some(cb) = hide_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Show => {
                    if let Some(cb) = show_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Paste => {
                    if let Some(cb) = paste_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::SelectionClear => {
                    if let Some(cb) = selection_clear_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::MouseWheel => {
                    if let Some(cb) = mouse_wheel_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::DndEnter => {
                    if let Some(cb) = dnd_enter_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::DndDrag => {
                    if let Some(cb) = dnd_drag_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::DndLeave => {
                    if let Some(cb) = dnd_leave_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::DndRelease => {
                    if let Some(cb) = dnd_release_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::ScreenConfigChanged => {
                    if let Some(cb) = screen_config_changed_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Fullscreen => {
                    if let Some(cb) = fullscreen_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::ZoomGesture => {
                    if let Some(cb) = zoom_gesture_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::ZoomEvent => {
                    if let Some(cb) = zoom_event_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                Event::Resize => {
                    if let Some(cb) = resize_cb.borrow_mut().as_mut() {
                        cb(b);
                        b.redraw();
                    }
                    true
                }
                _ => false,
            }
        });
        Self {
            wid,
            hover_cb,
            leave_cb,
            push_cb,
            released_cb,
            drag_cb,
            focus_cb,
            unfocus_cb,
            keydown_cb,
            keyup_cb,
            close_cb,
            move_cb,
            shortcut_cb,
            deactivate_cb,
            activate_cb,
            hide_cb,
            show_cb,
            paste_cb,
            selection_clear_cb,
            mouse_wheel_cb,
            dnd_enter_cb,
            dnd_drag_cb,
            dnd_leave_cb,
            dnd_release_cb,
            screen_config_changed_cb,
            fullscreen_cb,
            zoom_gesture_cb,
            zoom_event_cb,
            resize_cb,
        }
    }

    /// Construct a widget filling the parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent().center_of_parent()
    }

    /// What the widget should do on hover
    pub fn on_hover(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.hover_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on leave
    pub fn on_leave(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.leave_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on click
    pub fn on_click(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.push_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on release
    pub fn on_release(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.released_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on drag
    pub fn on_drag(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.drag_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on focus
    pub fn on_focus(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.focus_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on unfocus
    pub fn on_unfocus(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.unfocus_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on keydown
    pub fn on_keydown(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.keydown_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on keyup
    pub fn on_keyup(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.keyup_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on close
    pub fn on_close(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.close_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on move
    pub fn on_move(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.move_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on shortcut
    pub fn on_shortcut(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.shortcut_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on deactivate
    pub fn on_deactivate(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.deactivate_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on activate
    pub fn on_activate(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.activate_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on hide
    pub fn on_hide(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.hide_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on show
    pub fn on_show(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.show_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on paste
    pub fn on_paste(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.paste_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on selection_clear
    pub fn on_selection_clear(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.selection_clear_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on mousewheel
    pub fn on_mousewheel(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.mouse_wheel_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on dnd_enter
    pub fn on_dnd_enter(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.dnd_enter_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on dnd_drag
    pub fn on_dnd_drag(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.dnd_drag_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on dnd_leave
    pub fn on_dnd_leave(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.dnd_leave_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on dnd_release
    pub fn on_dnd_release(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.dnd_release_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on screen_config_chaged
    pub fn on_screen_config_chaged(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.screen_config_changed_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on fullscreen
    pub fn on_fullscreen(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.fullscreen_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on zoom_gesture
    pub fn on_zoom_gesture(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.zoom_gesture_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on zoom
    pub fn on_zoom(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.zoom_event_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// What the widget should do on resize
    pub fn on_resize(&mut self, cb: impl FnMut(&mut T) + 'static) {
        *self.resize_cb.borrow_mut() = Some(Box::new(cb));
    }

    /// Initialize to position x, y
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        let w = self.w();
        let h = self.h();
        self.resize(x, y, w, h);
        self
    }

    /// Initialize to size width, height
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        let x = self.x();
        let y = self.y();
        let w = self.width();
        let h = self.height();
        if w == 0 || h == 0 {
            self.widget_resize(x, y, width, height);
        } else {
            self.resize(x, y, width, height);
        }
        self
    }

    /// Initialize with a label
    pub fn with_label(mut self, title: &str) -> Self {
        self.set_label(title);
        self
    }

    /// Initialize with alignment
    pub fn with_align(mut self, align: Align) -> Self {
        self.set_align(align);
        self
    }

    /// Initialize with type
    pub fn with_type<W: WidgetType>(mut self, typ: W) -> Self {
        assert!(!self.was_deleted());
        self.set_type(typ);
        self
    }

    /// Initialize at bottom of another widget
    pub fn below_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
        assert!(!wid.was_deleted());
        assert!(!self.was_deleted());
        let w = self.w();
        let h = self.h();
        debug_assert!(
            w != 0 && h != 0,
            "below_of requires the size of the widget to be known!"
        );
        self.resize(wid.x(), wid.y() + wid.h() + padding, w, h);
        self
    }

    /// Initialize above of another widget
    pub fn above_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
        assert!(!wid.was_deleted());
        assert!(!self.was_deleted());
        let w = self.w();
        let h = self.h();
        debug_assert!(
            w != 0 && h != 0,
            "above_of requires the size of the widget to be known!"
        );
        self.resize(wid.x(), wid.y() - padding - h, w, h);
        self
    }

    /// Initialize right of another widget
    pub fn right_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
        assert!(!wid.was_deleted());
        assert!(!self.was_deleted());
        let w = self.w();
        let h = self.h();
        debug_assert!(
            w != 0 && h != 0,
            "right_of requires the size of the widget to be known!"
        );
        self.resize(wid.x() + wid.width() + padding, wid.y(), w, h);
        self
    }

    /// Initialize left of another widget
    pub fn left_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
        assert!(!wid.was_deleted());
        assert!(!self.was_deleted());
        let w = self.w();
        let h = self.h();
        debug_assert!(
            w != 0 && h != 0,
            "left_of requires the size of the widget to be known!"
        );
        self.resize(wid.x() - w - padding, wid.y(), w, h);
        self
    }

    /// Initialize center of another widget
    pub fn center_of<W: WidgetExt>(mut self, w: &W) -> Self {
        assert!(!w.was_deleted());
        assert!(!self.was_deleted());
        debug_assert!(
            w.width() != 0 && w.height() != 0,
            "center_of requires the size of the widget to be known!"
        );
        let sw = self.width() as f64;
        let sh = self.height() as f64;
        let ww = w.width() as f64;
        let wh = w.height() as f64;
        let sx = (ww - sw) / 2.0;
        let sy = (wh - sh) / 2.0;
        let wx = if w.as_window().is_some() { 0 } else { w.x() };
        let wy = if w.as_window().is_some() { 0 } else { w.y() };
        self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
        self.redraw();
        self
    }

    /// Initialize center of parent
    pub fn center_of_parent(mut self) -> Self {
        assert!(!self.was_deleted());
        if let Some(w) = self.parent() {
            debug_assert!(
                w.width() != 0 && w.height() != 0,
                "center_of requires the size of the widget to be known!"
            );
            let sw = self.width() as f64;
            let sh = self.height() as f64;
            let ww = w.width() as f64;
            let wh = w.height() as f64;
            let sx = (ww - sw) / 2.0;
            let sy = (wh - sh) / 2.0;
            let wx = if w.as_window().is_some() { 0 } else { w.x() };
            let wy = if w.as_window().is_some() { 0 } else { w.y() };
            self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
            self.redraw();
        }
        self
    }

    /// Initialize to the size of another widget
    pub fn size_of<W: WidgetExt>(mut self, w: &W) -> Self {
        assert!(!w.was_deleted());
        assert!(!self.was_deleted());
        debug_assert!(
            w.width() != 0 && w.height() != 0,
            "size_of requires the size of the widget to be known!"
        );
        let x = self.x();
        let y = self.y();
        self.resize(x, y, w.width(), w.height());
        self
    }

    /// Initialize to the size of the parent
    pub fn size_of_parent(mut self) -> Self {
        assert!(!self.was_deleted());
        if let Some(parent) = self.parent() {
            let w = parent.width();
            let h = parent.height();
            let x = self.x();
            let y = self.y();
            self.resize(x, y, w, h);
        }
        self
    }
}

impl<T: WidgetBase + WidgetExt + Default + 'static> From<T> for Listener<T> {
    fn from(t: T) -> Self {
        Self::from_widget(t)
    }
}
