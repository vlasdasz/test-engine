use std::{
    f32::consts::PI,
    sync::{Mutex, MutexGuard},
};

use chrono::Utc;
use dispatch::{on_main, on_main_sync};
use gm::{
    flat::{point_on_circle, Size},
    Color,
};
use log::{trace, warn};
use refs::Weak;
use rtools::Animation;
use ui::{
    view, Container, ModalView, OnceEvent, TouchStack, UIAnimation, View, ViewAnimation, ViewCallbacks,
    ViewData, ViewFrame, ViewSetup, ViewSubviews, MICROSECONDS_IN_ONE_SECOND,
};

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

impl ViewSetup for Spinner {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::GRAY.with_alpha(0.8));
        self.set_corner_radius(20);

        for _ in 0..CIRCLES_N {
            let mut circle = self.__internal_add_view::<Container>().weak();

            circle.set_size((16, 16));
            circle.set_color(Color::LIGHT_BLUE);
            circle.set_corner_radius(8);

            self.circles.push(circle);
        }
    }
}

impl ViewCallbacks for Spinner {
    fn update(&mut self) {
        let current_time: i64 = Utc::now().timestamp_micros();

        let val = ((current_time % MICROSECONDS_IN_ONE_SECOND) as f32) / MICROSECONDS_IN_ONE_SECOND as f32;

        let span = PI * 2.0;
        let start = -PI;

        let angle = start + span * val;

        let step = 2.0 * PI / CIRCLES_N as f32;

        let points: Vec<_> = (0..CIRCLES_N)
            .map(|index| point_on_circle(40.0, angle + step * index as f32, self.size().center()))
            .collect();

        for (view, point) in self.circles.iter_mut().zip(points) {
            view.set_origin((point.x - 8.0, point.y - 8.0));
        }
    }
}

impl Spinner {
    pub fn lock() -> SpinnerLock {
        trace!("Lock spinner");
        Self::start();
        SpinnerLock { stopped: false }
    }

    pub fn start() {
        trace!("Start spinner");

        if Self::current().is_ok() {
            warn!("Spinner already started");
            return;
        }

        on_main_sync(|| {
            *Self::current() = Self::prepare_modally(());
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

            let animation = UIAnimation::new(Animation::new(0.8, 0, 0.4), |sp, val| {
                let color = sp.color();
                sp.set_color(color.with_alpha(val));
                for dot in sp.subviews_mut() {
                    let color = *dot.color();
                    dot.set_color(color.with_alpha(val));
                }
            });

            animation.on_finish.sub(|| {
                let mut spinner = Self::current();
                spinner.remove_from_superview();
                *spinner = Default::default();
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
        *spinner = Default::default();
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
