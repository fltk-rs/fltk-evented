use fltk::{
    enums::Align,
    prelude::{WidgetBase, WidgetExt, WidgetType},
};

/// The base listener widget, wraps a fltk [`WidgetBase`].
#[derive(Debug, Clone)]
pub struct BaseListener<T: WidgetBase + WidgetExt, TRIG> {
    #[allow(dead_code)]
    pub(crate) wid: T,
    pub(crate) trig: TRIG,
}

/// `#[derive(Default)]` won't register callbacks, so we must impl `Default` manually.
impl<T: WidgetBase + WidgetExt + Default + Into<BaseListener<T, TRIG>>, TRIG> Default for BaseListener<T, TRIG> {
    fn default() -> Self {
        T::default().into()
    }
}

/// Used to call methods like [`WidgetExt::x`].
impl<T: WidgetBase + WidgetExt, TRIG> std::ops::Deref for BaseListener<T, TRIG> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

/// Used to call methods like [`WidgetExt::set_pos`].
impl<T: WidgetBase + WidgetExt, TRIG> std::ops::DerefMut for BaseListener<T, TRIG> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}

/// Constructors, depends on `impl From<T> for BaseListener<T, TRIG>`, see [`crate::blocking::Listener::from`]
impl<T: WidgetBase + WidgetExt + Into<BaseListener<T, TRIG>>, TRIG> BaseListener<T, TRIG> {
    /// Not recommanded, use [`Into<BaseListener>`] like `let btn: Listener<_> = btn.into();`
    pub fn from_widget(wid: T) -> Self {
        wid.into()
    }

    /// Creates a new widget, takes an x, y coordinates, as well as a width and height, plus a title.
    /// Same as [`WidgetBase::new`]
    pub fn new<S: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: S) -> Self {
        T::new(x, y, w, h, label).into()
    }

    /// Construct a widget filling the parent.
    /// Same as [`WidgetBase::default_fill`]
    pub fn default_fill() -> Self {
        T::default_fill().into()
    }
}

/// Builder functions, delegated to [`WidgetBase`]
impl<T: WidgetBase + WidgetExt, TRIG> BaseListener<T, TRIG> {
    /// Initialize to position x, y
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.wid = self.wid.with_pos(x, y);
        self
    }

    /// Initialize to size width, height
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.wid = self.wid.with_size(width, height);
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
        self.wid = self.wid.with_type(typ);
        self
    }

    /// Initialize at bottom of another widget
    pub fn below_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
        self.wid = self.wid.below_of(wid, padding);
        self
    }

    /// Initialize above of another widget
    pub fn above_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
        self.wid = self.wid.above_of(wid, padding);
        self
    }

    /// Initialize right of another widget
    pub fn right_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
        self.wid = self.wid.right_of(wid, padding);
        self
    }

    /// Initialize left of another widget
    pub fn left_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
        self.wid = self.wid.left_of(wid, padding);
        self
    }

    /// Initialize center of another widget
    pub fn center_of<W: WidgetExt>(mut self, w: &W) -> Self {
        self.wid = self.wid.center_of(w);
        self
    }

    /// Initialize center of parent
    pub fn center_of_parent(mut self) -> Self {
        self.wid = self.wid.center_of_parent();
        self
    }

    /// Initialize center of another widget on the x axis
    pub fn center_x<W: WidgetExt>(mut self, w: &W) -> Self {
        self.wid = self.wid.center_x(w);
        self
    }

    /// Initialize center of another widget on the y axis
    pub fn center_y<W: WidgetExt>(mut self, w: &W) -> Self {
        self.wid = self.wid.center_y(w);
        self
    }

    /// Initialize to the size of another widget
    pub fn size_of<W: WidgetExt>(mut self, w: &W) -> Self {
        self.wid = self.wid.size_of(w);
        self
    }

    /// Initialize to the size of the parent
    pub fn size_of_parent(mut self) -> Self {
        self.wid = self.wid.size_of_parent();
        self
    }
}
