use fltk::{
    app,
    enums::Align,
    prelude::{WidgetBase, WidgetExt, WidgetType},
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[cfg(feature = "tokio")]
use tokio::spawn;

#[cfg(feature = "async-std")]
use async_std::task::spawn;

/// A listener widget
#[derive(Default, Clone)]
pub struct AsyncListener<T: WidgetBase + WidgetExt> {
    #[allow(dead_code)]
    wid: T,
    trig: Arc<AtomicBool>,
}

impl<T: WidgetBase + WidgetExt + Default + 'static> From<T> for AsyncListener<T> {
    fn from(t: T) -> Self {
        Self::from_widget(t)
    }
}

impl<T: WidgetBase + WidgetExt> std::ops::Deref for AsyncListener<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl<T: WidgetBase + WidgetExt> std::ops::DerefMut for AsyncListener<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}

/// The listener widget's implementation
impl<T: WidgetBase + WidgetExt + Default + 'static> AsyncListener<T> {
    /// The same constructor for fltk-rs widgets can be used for Listeners
    pub fn new<S: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: S) -> Self {
        Self::from_widget(T::new(x, y, w, h, label))
    }

    fn from_widget(mut wid: T) -> Self {
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

    /// Construct a widget filling the parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent().center_of_parent()
    }

    /// Check whether a widget was triggered
    pub async fn triggered(&self) -> bool {
        self.trig.swap(false, Ordering::Relaxed)
    }

    /// Initialize to position x, y
    pub fn with_pos(self, x: i32, y: i32) -> Self {
        let wid = self.wid.with_pos(x, y);
        Self { wid, ..self }
    }

    /// Initialize to size width, height
    pub fn with_size(self, width: i32, height: i32) -> Self {
        let wid = self.wid.with_size(width, height);
        Self { wid, ..self }
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
    pub fn with_type<W: WidgetType>(self, typ: W) -> Self {
        let wid = self.wid.with_type(typ);
        Self { wid, ..self }
    }

    /// Initialize at bottom of another widget
    pub fn below_of<W: WidgetExt>(self, wid: &W, padding: i32) -> Self {
        let wid = self.wid.below_of(wid, padding);
        Self { wid, ..self }
    }

    /// Initialize above of another widget
    pub fn above_of<W: WidgetExt>(self, wid: &W, padding: i32) -> Self {
        let wid = self.wid.above_of(wid, padding);
        Self { wid, ..self }
    }

    /// Initialize right of another widget
    pub fn right_of<W: WidgetExt>(self, wid: &W, padding: i32) -> Self {
        let wid = self.wid.right_of(wid, padding);
        Self { wid, ..self }
    }

    /// Initialize left of another widget
    pub fn left_of<W: WidgetExt>(self, wid: &W, padding: i32) -> Self {
        let wid = self.wid.left_of(wid, padding);
        Self { wid, ..self }
    }

    /// Initialize center of another widget
    pub fn center_of<W: WidgetExt>(self, w: &W) -> Self {
        let wid = self.wid.center_of(w);
        Self { wid, ..self }
    }

    /// Initialize center of parent
    pub fn center_of_parent(self) -> Self {
        let wid = self.wid.center_of_parent();
        Self { wid, ..self }
    }

    /// Initialize center of another widget on the x axis
    pub fn center_x<W: WidgetExt>(self, w: &W) -> Self {
        let wid = self.wid.center_x(w);
        Self { wid, ..self }
    }

    /// Initialize center of another widget on the y axis
    pub fn center_y<W: WidgetExt>(self, w: &W) -> Self {
        let wid = self.wid.center_y(w);
        Self { wid, ..self }
    }

    /// Initialize to the size of another widget
    pub fn size_of<W: WidgetExt>(self, w: &W) -> Self {
        let wid = self.wid.size_of(w);
        Self { wid, ..self }
    }

    /// Initialize to the size of the parent
    pub fn size_of_parent(self) -> Self {
        let wid = self.wid.size_of_parent();
        Self { wid, ..self }
    }
}
