use crate::EventMap;
use fltk::{
    enums::{Align, Event},
    prelude::{WidgetBase, WidgetExt, WidgetType},
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A listener widget
#[derive(Default, Clone)]
pub struct Listener<T: WidgetBase + WidgetExt> {
    #[allow(dead_code)]
    wid: T,
    events: Rc<RefCell<EventMap<T>>>,
    trig: Rc<RefCell<bool>>,
    evented: Rc<RefCell<HashMap<i32, bool>>>,
}

impl<T: WidgetBase + WidgetExt + Default + 'static> From<T> for Listener<T> {
    fn from(t: T) -> Self {
        Self::from_widget(t)
    }
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
    /// The same constructor for fltk-rs widgets can be used for Listeners
    pub fn new<S: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: S) -> Self {
        let mut wid = T::new(x, y, w, h, label);
        let trig = Rc::new(RefCell::new(false));
        let mut evented = HashMap::new();
        for i in 0..30 {
            evented.insert(i, false);
        }
        let evented = Rc::from(RefCell::from(evented));
        let events: EventMap<T> = HashMap::new();
        let events = Rc::from(RefCell::from(events));
        wid.handle({
            let events = events.clone();
            let evented = evented.clone();
            move |b, ev| {
                let ret1 = if let Some(Some(cb)) = events.borrow_mut().get_mut(&(ev.bits())) {
                    cb(b);
                    b.redraw();
                    true
                } else {
                    false
                };
                let ret2 = if let Some(t) = evented.borrow_mut().get_mut(&(ev.bits())) {
                    *t = true;
                    b.redraw();
                    true
                } else {
                    false
                };
                ret1 || ret2
            }
        });
        wid.set_callback({
            let trig = trig.clone();
            move |_| {
                *trig.borrow_mut() = true;
            }
        });
        Self {
            wid,
            events,
            trig,
            evented,
        }
    }

    fn from_widget(mut wid: T) -> Self {
        let events: EventMap<T> = HashMap::new();
        let events = Rc::from(RefCell::from(events));
        let trig = Rc::new(RefCell::new(false));
        let mut evented = HashMap::new();
        for i in 0..30 {
            evented.insert(i, false);
        }
        let evented = Rc::from(RefCell::from(evented));
        wid.handle({
            let events = events.clone();
            let evented = evented.clone();
            move |b, ev| {
                let ret1 = if let Some(Some(cb)) = events.borrow_mut().get_mut(&(ev.bits())) {
                    cb(b);
                    b.redraw();
                    true
                } else {
                    false
                };
                let ret2 = if let Some(t) = evented.borrow_mut().get_mut(&(ev.bits())) {
                    *t = true;
                    b.redraw();
                    true
                } else {
                    false
                };
                ret1 || ret2
            }
        });
        wid.set_callback({
            let trig = trig.clone();
            move |_| {
                *trig.borrow_mut() = true;
            }
        });
        Self {
            wid,
            events,
            trig,
            evented,
        }
    }

    /// Construct a widget filling the parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent().center_of_parent()
    }

    /// Check whether a widget was triggered
    pub fn triggered(&self) -> bool {
        let curr = *self.trig.borrow();
        *self.trig.borrow_mut() = false;
        curr
    }

    /// Check if an event was triggered
    pub fn done(&self, ev: Event) -> bool {
        let curr = *self.evented.borrow().get(&ev.bits()).unwrap();
        *self.evented.borrow_mut().get_mut(&ev.bits()).unwrap() = false;
        curr
    }

    /// What the widget should do on a custom event
    pub fn on(&mut self, ev: Event, cb: impl FnMut(&mut T) + 'static) {
        self.events
            .borrow_mut()
            .insert(ev.bits(), Some(Box::new(cb)));
    }

    /// What the widget should do on hover
    pub fn on_hover(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Enter, cb);
    }

    /// What the widget should do on leave
    pub fn on_leave(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Leave, cb);
    }

    /// What the widget should do on click
    pub fn on_click(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Push, cb);
    }

    /// What the widget should do on release
    pub fn on_release(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Released, cb);
    }

    /// What the widget should do on drag
    pub fn on_drag(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Drag, cb);
    }

    /// What the widget should do on focus
    pub fn on_focus(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Focus, cb);
    }

    /// What the widget should do on unfocus
    pub fn on_unfocus(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Unfocus, cb);
    }

    /// What the widget should do on keydown
    pub fn on_keydown(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::KeyDown, cb);
    }

    /// What the widget should do on keyup
    pub fn on_keyup(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::KeyUp, cb);
    }

    /// What the widget should do on close
    pub fn on_close(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Close, cb);
    }

    /// What the widget should do on move
    pub fn on_move(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Move, cb);
    }

    /// What the widget should do on shortcut
    pub fn on_shortcut(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Shortcut, cb);
    }

    /// What the widget should do on deactivate
    pub fn on_deactivate(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Deactivate, cb);
    }

    /// What the widget should do on activate
    pub fn on_activate(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Activate, cb);
    }

    /// What the widget should do on hide
    pub fn on_hide(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Hide, cb);
    }

    /// What the widget should do on show
    pub fn on_show(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Show, cb);
    }

    /// What the widget should do on paste
    pub fn on_paste(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Paste, cb);
    }

    /// What the widget should do on selection_clear
    pub fn on_selection_clear(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::SelectionClear, cb);
    }

    /// What the widget should do on mousewheel
    pub fn on_mousewheel(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::MouseWheel, cb);
    }

    /// What the widget should do on dnd_enter
    pub fn on_dnd_enter(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::DndEnter, cb);
    }

    /// What the widget should do on dnd_drag
    pub fn on_dnd_drag(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::DndDrag, cb);
    }

    /// What the widget should do on dnd_leave
    pub fn on_dnd_leave(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::DndLeave, cb);
    }

    /// What the widget should do on dnd_release
    pub fn on_dnd_release(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::DndRelease, cb);
    }

    /// What the widget should do on screen_config_changed
    pub fn on_screen_config_changed(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::ScreenConfigChanged, cb);
    }

    /// What the widget should do on fullscreen
    pub fn on_fullscreen(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Fullscreen, cb);
    }

    /// What the widget should do on zoom_gesture
    pub fn on_zoom_gesture(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::ZoomGesture, cb);
    }

    /// What the widget should do on zoom
    pub fn on_zoom(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::ZoomEvent, cb);
    }

    /// What the widget should do on resize
    pub fn on_resize(&mut self, cb: impl FnMut(&mut T) + 'static) {
        self.on(Event::Resize, cb);
    }

    /// Return whether the widget was hovered
    pub fn hovered(&self) -> bool {
        self.done(Event::Enter)
    }

    /// Return whether the widget was left
    pub fn left(&self) -> bool {
        self.done(Event::Leave)
    }

    /// Return whether the widget was clicked
    pub fn clicked(&self) -> bool {
        self.done(Event::Push)
    }

    /// Return whether the widget was released
    pub fn released(&self) -> bool {
        self.done(Event::Released)
    }

    /// Return whether the widget was dragged
    pub fn dragged(&self) -> bool {
        self.done(Event::Drag)
    }

    /// Return whether the widget was focused
    pub fn focused(&self) -> bool {
        self.done(Event::Focus)
    }

    /// Return whether the widget was unfocused
    pub fn unfocused(&self) -> bool {
        self.done(Event::Unfocus)
    }

    /// Return whether the widget was keydowned
    pub fn keydowned(&self) -> bool {
        self.done(Event::KeyDown)
    }

    /// Return whether the widget was keyuped
    pub fn keyuped(&self) -> bool {
        self.done(Event::KeyUp)
    }

    /// Return whether the widget was closed
    pub fn closed(&self) -> bool {
        self.done(Event::Close)
    }

    /// Return whether the widget was moved
    pub fn moved(&self) -> bool {
        self.done(Event::Move)
    }

    /// Return whether the widget was shortcuted
    pub fn shortcuted(&self) -> bool {
        self.done(Event::Shortcut)
    }

    /// Return whether the widget was deactivated
    pub fn deactivated(&self) -> bool {
        self.done(Event::Deactivate)
    }

    /// Return whether the widget was activated
    pub fn activated(&self) -> bool {
        self.done(Event::Activate)
    }

    /// Return whether the widget was hiden
    pub fn hiden(&self) -> bool {
        self.done(Event::Hide)
    }

    /// Return whether the widget was showed
    pub fn showed(&self) -> bool {
        self.done(Event::Show)
    }

    /// Return whether the widget was pasted to
    pub fn pasted_to(&self) -> bool {
        self.done(Event::Paste)
    }

    /// Return whether the widget was selection_cleared
    pub fn selection_cleared(&self) -> bool {
        self.done(Event::SelectionClear)
    }

    /// Return whether the widget was mousewheeled
    pub fn mousewheeled(&self) -> bool {
        self.done(Event::MouseWheel)
    }

    /// Return whether the widget was dnd_entered
    pub fn dnd_entered(&self) -> bool {
        self.done(Event::DndEnter)
    }

    /// Return whether the widget was dnd_dragged
    pub fn dnd_dragged(&self) -> bool {
        self.done(Event::DndDrag)
    }

    /// Return whether the widget was dnd_left
    pub fn dnd_left(&self) -> bool {
        self.done(Event::DndLeave)
    }

    /// Return whether the widget was dnd_released
    pub fn dnd_released(&self) -> bool {
        self.done(Event::DndRelease)
    }

    /// Return whether the widget was screen_config_changed
    pub fn screen_config_changed(&self) -> bool {
        self.done(Event::ScreenConfigChanged)
    }

    /// Return whether the widget was fullscreened
    pub fn fullscreened(&self) -> bool {
        self.done(Event::Fullscreen)
    }

    /// Return whether the widget was zoom_gestured
    pub fn zoom_gestured(&self) -> bool {
        self.done(Event::ZoomGesture)
    }

    /// Return whether the widget was zoomed
    pub fn zoomed(&self) -> bool {
        self.done(Event::ZoomEvent)
    }

    /// Return whether the widget was resized
    pub fn resized(&self) -> bool {
        self.done(Event::Resize)
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

    /// Initialize center of another widget on the x axis
    pub fn center_x<W: WidgetExt>(mut self, w: &W) -> Self {
        assert!(!w.was_deleted());
        assert!(!self.was_deleted());
        debug_assert!(
            w.width() != 0 && w.height() != 0,
            "center_of requires the size of the widget to be known!"
        );
        let sw = self.width() as f64;
        let sh = self.height() as f64;
        let ww = w.width() as f64;
        let sx = (ww - sw) / 2.0;
        let sy = self.y();
        let wx = if w.as_window().is_some() { 0 } else { w.x() };
        self.resize(sx as i32 + wx, sy, sw as i32, sh as i32);
        self.redraw();
        self
    }

    /// Initialize center of another widget on the y axis
    pub fn center_y<W: WidgetExt>(mut self, w: &W) -> Self {
        assert!(!w.was_deleted());
        assert!(!self.was_deleted());
        debug_assert!(
            w.width() != 0 && w.height() != 0,
            "center_of requires the size of the widget to be known!"
        );
        let sw = self.width() as f64;
        let sh = self.height() as f64;
        let wh = w.height() as f64;
        let sx = self.x();
        let sy = (wh - sh) / 2.0;
        let wy = if w.as_window().is_some() { 0 } else { w.y() };
        self.resize(sx, sy as i32 + wy, sw as i32, sh as i32);
        self.redraw();
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
