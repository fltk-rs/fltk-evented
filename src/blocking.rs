use fltk::enums::Event;
use fltk::prelude::{WidgetBase, WidgetExt};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::base::BaseListener;
use std::collections::HashMap;

type EventMap<T> = HashMap<i32, Option<Box<dyn FnMut(&mut T)>>>;

#[derive(Clone)]
pub struct Trig<T> {
    triggered: Rc<Cell<bool>>,
    event: Rc<Cell<Event>>,
    events: Rc<RefCell<EventMap<T>>>,
}

/// The blocking widget listener recieves both `triggered: bool` from [`Listener<T>::triggered()`],
/// and [`Event`] from [`Listener<T>::event()`].
pub type Listener<T> = BaseListener<T, Trig<T>>;

/// core constructor
impl<T: WidgetBase + WidgetExt + 'static> From<T> for Listener<T> {
    fn from(mut wid: T) -> Self {
        let triggered = Rc::new(Cell::new(false));
        wid.set_callback({
            let triggered = triggered.clone();
            move |_| {
                triggered.set(true);
            }
        });
        let event = Rc::new(Cell::new(Event::NoEvent));
        let events: EventMap<T> = HashMap::new();
        let events = Rc::from(RefCell::from(events));
        wid.handle({
            let event = event.clone();
            let events = events.clone();
            move |w, evt| {
                let ret = if !events.borrow().is_empty() {
                    if let Some(Some(cb)) = events.borrow_mut().get_mut(&(evt.bits())) {
                        cb(w);
                        w.redraw();
                        true
                    } else {
                        false
                    }
                } else {
                    event.set(evt);
                    false
                };
                ret
            }
        });
        let trig = Trig { triggered, event, events };
        Self { wid, trig }
    }
}

/// core implementation
impl<T: WidgetBase + WidgetExt> Listener<T> {
    /// Check whether a widget was triggered
    pub fn triggered(&self) -> bool {
        self.trig.triggered.replace(false)
    }

    /// Get an event the widget received,
    /// returns [`Event::NoEvent`] if no events received
    pub fn event(&self) -> Event {
        self.trig.event.replace(Event::NoEvent)
    }

    /// What the widget should do on a custom event
    pub fn on(&mut self, ev: Event, cb: impl FnMut(&mut T) + 'static) {
        self.trig.events
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
}
