use fltk::enums::Event;
use fltk::prelude::{WidgetBase, WidgetExt};
use std::cell::Cell;
use std::rc::Rc;
use crate::base::BaseListener;

pub struct Trig {
    triggered: Rc<Cell<bool>>,
    event: Rc<Cell<Event>>,
}

/// The blocking listener widget
pub type Listener<T> = BaseListener<T, Trig>;

/// core constructor
impl<T: WidgetBase + WidgetExt> From<T> for Listener<T> {
    fn from(mut wid: T) -> Self {
        let triggered = Rc::new(Cell::new(false));
        wid.set_callback({
            let triggered = triggered.clone();
            move |_| {
                triggered.set(true);
            }
        });
        let event = Rc::new(Cell::new(Event::NoEvent));
        wid.handle({
            let event = event.clone();
            move |_, evt| {
                event.set(evt);
                false
            }
        });
        let trig = Trig { triggered, event };
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
}
