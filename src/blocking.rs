use fltk::{
    enums::Align,
    prelude::{WidgetBase, WidgetExt, WidgetType},
};
use std::cell::RefCell;
use std::rc::Rc;

/// A listener widget
#[derive(Default, Clone)]
pub struct Listener<T: WidgetBase + WidgetExt> {
    #[allow(dead_code)]
    wid: T,
    trig: Rc<RefCell<bool>>,
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
        wid.set_callback({
            let trig = trig.clone();
            move |_| {
                *trig.borrow_mut() = true;
            }
        });
        Self {
            wid,
            trig,
        }
    }

    fn from_widget(mut wid: T) -> Self {
        let trig = Rc::new(RefCell::new(false));
        wid.set_callback({
            let trig = trig.clone();
            move |_| {
                *trig.borrow_mut() = true;
            }
        });
        Self {
            wid,
            trig,
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
