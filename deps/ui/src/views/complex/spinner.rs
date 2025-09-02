use std::{
    f32::consts::PI,
    sync::{Mutex, MutexGuard},
};

use dispatch::{on_main, on_main_sync};
use gm::{
    Animation, LossyConvert,
    color::{GRAY, LIGHT_BLUE},
};
use ui_proc::view;
use vents::OnceEvent;

use crate::{
    Container, MICROSECONDS_IN_ONE_SECOND, ModalView, Setup, TouchStack, UIAnimation, ViewCallbacks,
    view::{View, ViewAnimation, ViewData, ViewFrame, ViewSubviews},
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use chrono::Utc;
use gm::flat::{Size, point_on_circle};
use log::{trace, warn};
use refs::Weak;

static CIRCLES_N: u32 = 6;
static SPINNER: Mutex<Weak<Spinner>> = Mutex::new(Weak::const_default());

pub struct SpinnerLock {
    stopped: bool,
}

impl SpinnerLock {
    pub fn stop(mut self) {
        self.stopped = true;
        Spinner::stop();
    }
    pub fn instant_stop(self) {}
}

impl Drop for SpinnerLock {
    fn drop(&mut self) {
        trace!("Spinner lock dropped");
        if !self.stopped {
            Spinner::instant_stop();
        }
    }
}

#[view]
pub struct Spinner {
    circles: Vec<Weak<Container>>,
    event:   OnceEvent,
}

impl Spinner {
    fn current() -> MutexGuard<'static, Weak<Spinner>> {
        SPINNER.lock().unwrap()
    }

    // fn set_alpha(&mut self, alpha: impl IntoF32) {
    //     self.set_color(self.color().with_alpha(alpha));
    //     for cir in &mut self.circles {
    //         let c = *cir.color();
    //         cir.set_color(c.with_alpha(alpha));
    //     }
    // }
}

impl Setup for Spinner {
    fn setup(mut self: Weak<Self>) {
        self.set_color(GRAY.with_alpha(0.8));
        self.set_corner_radius(20);

        for _ in 0..CIRCLES_N {
            let mut circle = self.add_view::<Container>();

            circle.set_size(16, 16);
            circle.set_color(LIGHT_BLUE);
            circle.set_corner_radius(8);

            self.circles.push(circle);
        }
    }
}

impl ViewCallbacks for Spinner {
    fn update(&mut self) {
        let current_time: i64 = Utc::now().timestamp_micros();

        let val = ((current_time % MICROSECONDS_IN_ONE_SECOND).lossy_convert())
            / MICROSECONDS_IN_ONE_SECOND.lossy_convert();

        let span = PI * 2.0;
        let start = -PI;

        let angle = start + span * val;

        let step = 2.0 * PI / CIRCLES_N.lossy_convert();

        let points: Vec<_> = (0..CIRCLES_N)
            .map(|index| point_on_circle(40.0, angle + step * index.lossy_convert(), self.size().center()))
            .collect();

        for (view, point) in self.circles.iter_mut().zip(points) {
            view.set_position((point.x - 8.0, point.y - 8.0));
        }
    }
}

impl Spinner {
    pub fn lock() -> SpinnerLock {
        trace!("Lock spinner");
        Self::start();
        SpinnerLock { stopped: false }
    }

    fn start() {
        trace!("Start spinner");

        if Self::current().is_ok() {
            warn!("Spinner already started");
            return;
        }

        on_main_sync(|| {
            *Self::current() = Self::prepare_modally();
        });
    }

    pub fn stop() {
        trace!("Stop spinner");

        if Self::current().is_null() {
            warn!("Spinner already stopped");
            return;
        }

        on_main(|| {
            let mut spinner = Self::current();
            TouchStack::pop_layer(spinner.weak_view());

            let animation = UIAnimation::new(Animation::new(0.8, 0.0, 0.4), |sp, val| {
                let color = sp.color();
                sp.set_color(color.with_alpha(val));
                for mut dot in sp.subviews_mut() {
                    let color = *dot.color();
                    dot.set_color(color.with_alpha(val));
                }
            });

            animation.on_finish.sub(|| {
                let mut spinner = Self::current();
                spinner.remove_from_superview();
                *spinner = Weak::default();
            });

            spinner.add_animation(animation);
        });
    }

    pub fn instant_stop() {
        trace!("Instant stop spinner");

        let mut spinner = Self::current();

        if spinner.is_null() {
            return;
        }

        spinner.hide_modal(());
        *spinner = Weak::default();
    }
}

impl ModalView for Spinner {
    fn modal_event(&self) -> &OnceEvent<()> {
        &self.event
    }

    fn modal_size() -> Size {
        (140, 140).into()
    }
}
