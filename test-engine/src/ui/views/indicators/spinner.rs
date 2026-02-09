use std::f32::consts::PI;

use chrono::Utc;
use gm::{
    Animation, LossyConvert,
    color::{Color, GRAY, LIGHT_BLUE},
    flat::{Size, point_on_circle},
};
use hreads::on_main;
use log::{trace, warn};
use parking_lot::{Mutex, MutexGuard};
use refs::Weak;
use ui::{
    Container, ModalView, Setup, TouchStack, UIAnimation, View, ViewAnimation, ViewCallbacks, ViewData,
    ViewFrame, ViewSubviews,
};
use ui_proc::view;
use vents::OnceEvent;

use crate as test_engine;

static CIRCLES_N: u32 = 6;
static SPINNER: Mutex<Weak<Spinner>> = Mutex::new(Weak::const_default());

pub struct SpinnerLock {
    stopped: bool,
}

impl SpinnerLock {
    pub fn animated_stop(mut self) {
        self.stopped = true;
        Spinner::stop();
    }
    pub fn stop(self) {}
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
    pub dot_color:      Color,
    pub rotation_speed: f32,

    circles: Vec<Weak<Container>>,
    event:   OnceEvent,
}

impl Spinner {
    fn current() -> MutexGuard<'static, Weak<Spinner>> {
        SPINNER.lock()
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
        self.dot_color = LIGHT_BLUE;
        self.rotation_speed = 1.5;

        for _ in 0..CIRCLES_N {
            let circle = self.add_view::<Container>();
            self.circles.push(circle);
        }
    }
}

impl ViewCallbacks for Spinner {
    fn update(&mut self) {
        let duration_scale = self.rotation_speed;
        let microseconds_in_one_second = 1_000_000.0;

        let cycle_duration = (microseconds_in_one_second * duration_scale) as i64;

        let current_time: i64 = Utc::now().timestamp_micros();

        let val = (current_time % cycle_duration) as f32 / cycle_duration as f32;

        let span = PI * 2.0;
        let start = -PI;

        let angle = start + span * val;

        let step = 2.0 * PI / CIRCLES_N.lossy_convert();

        let parent_size = self.size().smallest_side();
        let size = parent_size * 0.115;
        let half_size = size / 2.0;

        let points: Vec<_> = (0..CIRCLES_N)
            .map(|index| {
                point_on_circle(
                    parent_size * 0.285,
                    angle + step * index.lossy_convert(),
                    self.size().center(),
                )
            })
            .collect();

        for (circle, point) in self.circles.iter_mut().zip(points) {
            circle
                .set_size(size, size)
                .set_corner_radius(half_size)
                .set_position((point.x - half_size, point.y - half_size))
                .set_color(self.dot_color);
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

        on_main(|| {
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

            if spinner.is_null() {
                warn!("Spinner already stopped");
                return;
            }

            TouchStack::pop_layer(spinner.weak_view());

            let animation = UIAnimation::new(Animation::new(0.8, 0.0, 0.4), |sp, val| {
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

mod test {

    use anyhow::Result;
    use gm::color::{BLACK, LIGHTER_GRAY, WHITE};
    use hreads::from_main;
    use refs::Weak;
    use ui::{Container, Setup, ViewData, ViewFrame, ViewSubviews, ViewTest, view_test};

    use crate::{self as test_engine, ui::Spinner, ui_test::check_colors};

    #[view_test]
    struct TestSpinner {
        #[init]
        content: Container,
    }

    impl Setup for TestSpinner {
        fn setup(self: Weak<Self>) {
            self.content.set_color(LIGHTER_GRAY).set_frame((20, 20, 50, 50));
        }
    }

    impl ViewTest for TestSpinner {
        fn perform_test(view: refs::Weak<Self>) -> Result<()> {
            let lock = Spinner::lock();

            check_colors(
                r"
                         221  290 -  89 124 149
                         242  290 - 173 177 181
                         309  306 - 173 177 181
                         362  301 - 173 177 181
                         397  301 -  89 124 149
                    ",
            )?;

            lock.stop();

            check_colors(
                r"
                         136  288 -  89 124 149
                         215  288 -  89 124 149
                         269  287 -  89 124 149
                         340  284 -  89 124 149
                         395  284 -  89 124 149
                         465  284 -  89 124 149
                    ",
            )?;

            let _lock = Spinner::lock();

            from_main(move || {
                let mut spinner = view.content.add_view::<Spinner>();
                spinner.place().back();
                spinner.dot_color = BLACK;
                spinner.rotation_speed = 2.2;

                let mut spinner = view.add_view::<Spinner>();
                spinner.place().back();
                spinner.dot_color = WHITE;
                spinner.rotation_speed = 5.0;
            });

            crate::ui_test::record_ui_test();

            Ok(())
        }
    }
}
