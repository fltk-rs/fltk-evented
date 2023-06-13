use fltk::prelude::{WidgetBase, WidgetExt};
use std::cell::RefCell;
use std::rc::Rc;
use crate::base::BaseListener;

/// The blocking listener widget
pub type Listener<T> = BaseListener<T, Rc<RefCell<bool>>>;

/// core constructor
impl<T: WidgetBase + WidgetExt> From<T> for Listener<T> {
    fn from(mut wid: T) -> Self {
        let trig = Rc::new(RefCell::new(false));
        wid.set_callback({
            let trig = trig.clone();
            move |_| {
                *trig.borrow_mut() = true;
            }
        });
        Self { wid, trig }
    }
}

/// core implementation
impl<T: WidgetBase + WidgetExt> Listener<T> {
    /// Check whether a widget was triggered
    pub fn triggered(&self) -> bool {
        self.trig.replace(false)
    }
}
