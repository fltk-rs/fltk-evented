use fltk::{
    app,
    enums::Event,
    prelude::{WidgetBase, WidgetExt},
};
use std::sync::atomic::{AtomicBool, Ordering, AtomicI32};
use std::sync::Arc;
use crate::base::BaseListener;

#[cfg(feature = "tokio")]
use tokio::spawn;

#[cfg(feature = "async-std")]
use async_std::task::spawn;

pub struct Trig {
    triggered: Arc<AtomicBool>,
    event: Arc<AtomicI32>,
}

/// The async listener widget
pub type AsyncListener<T> = BaseListener<T, Trig>;

/// core constructor
impl<T: WidgetBase + WidgetExt> From<T> for AsyncListener<T> {
    fn from(mut wid: T) -> Self {
        let triggered = Arc::new(AtomicBool::new(false));
        wid.set_callback({
            let triggered = triggered.clone();
            move |_| {
                let triggered = triggered.clone();
                spawn(async move {
                    triggered.store(true, Ordering::Relaxed);
                    app::awake();
                });
            }
        });
        let event = Arc::new(AtomicI32::new(Event::NoEvent.bits()));
        wid.handle({
            let event = event.clone();
            move |_, evt| {
                let event = event.clone();
                spawn(async move {
                    event.store(evt.bits(), Ordering::Relaxed);
                    app::awake();
                });
                false
            }
        });
        let trig = Trig { triggered, event };
        Self { wid, trig }
    }
}

/// core implementation
impl<T: WidgetBase + WidgetExt> AsyncListener<T> {
    /// Check whether a widget was triggered
    pub async fn triggered(&self) -> bool {
        self.trig.triggered.swap(false, Ordering::Relaxed)
    }

    /// Get an event the widget received,
    /// returns [`Event::NoEvent`] if no events received
    pub fn event(&self) -> Event {
        self.trig.event.swap(Event::NoEvent.bits(), Ordering::Relaxed).into()
    }
}
