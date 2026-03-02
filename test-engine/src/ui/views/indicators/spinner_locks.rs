use log::trace;
use refs::Weak;
use ui::ViewSubviews;

use crate::ui::Spinner;

pub struct SpinnerLockGlobal {
    pub(crate) stopped: bool,
}

impl SpinnerLockGlobal {
    pub fn animated_stop(mut self) {
        self.stopped = true;
        Spinner::stop();
    }
    pub fn stop(self) {}
}

impl Drop for SpinnerLockGlobal {
    fn drop(&mut self) {
        trace!("SpinnerLockGlobal dropped");
        if !self.stopped {
            Spinner::instant_stop();
        }
    }
}

pub struct SpinnerLockOnView {
    spinner: Weak<Spinner>,
}

impl SpinnerLockOnView {
    pub fn stop(self) {}
}

impl Drop for SpinnerLockOnView {
    fn drop(&mut self) {
        trace!("SpinnerLockOnView dropped");
        self.spinner.remove_from_superview();
    }
}
