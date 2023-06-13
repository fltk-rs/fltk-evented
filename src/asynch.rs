use fltk::{
    app,
    prelude::{WidgetBase, WidgetExt},
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::base::BaseListener;

#[cfg(feature = "tokio")]
use tokio::spawn;

#[cfg(feature = "async-std")]
use async_std::task::spawn;

/// The async listener widget
pub type AsyncListener<T> = BaseListener<T, Arc<AtomicBool>>;

/// core constructor
impl<T: WidgetBase + WidgetExt> From<T> for AsyncListener<T> {
    fn from(mut wid: T) -> Self {
        let trig = Arc::new(AtomicBool::new(false));
        wid.set_callback({
            let trig = trig.clone();
            move |_| {
                let trig = trig.clone();
                spawn(async move {
                    trig.store(true, Ordering::Relaxed);
                    app::awake();
                });
            }
        });
        Self { wid, trig }
    }
}

/// core implementation
impl<T: WidgetBase + WidgetExt> AsyncListener<T> {
    /// Check whether a widget was triggered
    pub async fn triggered(&self) -> bool {
        self.trig.swap(false, Ordering::Relaxed)
    }
}
