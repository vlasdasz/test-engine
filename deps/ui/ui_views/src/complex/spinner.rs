use std::f32::consts::PI;

use dispatch::on_main;
use gm::{flat::point_on_circle, Color};
use log::warn;
use refs::{Own, Weak};
use rtools::{Animation, Time};
use ui::{
    view, BaseView, UIAnimation, UIManager, View, ViewAnimation, ViewCallbacks, ViewData, ViewFrame,
    ViewSetup, ViewSubviews,
};

static CIRCLES_N: u32 = 6;
static mut SPINNER: Weak<dyn View> = Default::default();

#[view]
pub struct Spinner {
    circles: Vec<Weak<BaseView>>,
}

impl Spinner {
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
        self.place.size(140, 140).center();
        self.set_color(Color::GRAY.with_alpha(0.8));
        self.set_corner_radius(20);

        for _ in 0..CIRCLES_N {
            let mut circle = self.add_view::<BaseView>().weak();

            circle.set_size((16, 16));
            circle.set_color(Color::LIGHT_BLUE);
            circle.set_corner_radius(8);

            self.circles.push(circle);
        }
    }
}

impl ViewCallbacks for Spinner {
    fn update(&mut self) {
        const ONE_SECOND_IN_NANOSECONDS: i64 = 1_000_000_000;
        let current_time: i64 = Time::now();

        let val = ((current_time % ONE_SECOND_IN_NANOSECONDS) as f32) / ONE_SECOND_IN_NANOSECONDS as f32;

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
    pub fn start() {
        on_main(|| {
            UIManager::disable_touch();
            unsafe {
                SPINNER = UIManager::touch_root().add_subview(Own::<Spinner>::default());
            }
        });
    }

    pub fn stop() {
        on_main(|| unsafe {
            if SPINNER.is_null() {
                warn!("Spinner already stopped");
                return;
            }

            let animation = UIAnimation::new(SPINNER, Animation::new(0.8, 0, 0.4), |sp, val| {
                let color = sp.color();
                sp.set_color(color.with_alpha(val));
                for dot in sp.subviews_mut() {
                    let color = *dot.color();
                    dot.set_color(color.with_alpha(val));
                }
            });

            animation.on_finish.sub(|| {
                SPINNER.remove_from_superview();
                SPINNER = Default::default();
                UIManager::enable_touch();
            });

            SPINNER.add_animation(animation);
        });
    }
}
