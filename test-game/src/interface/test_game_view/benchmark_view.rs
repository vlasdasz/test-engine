use std::{
    sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering},
    time::{Duration, Instant},
};

use test_engine::{
    refs::{Weak, weak_from_ref},
    ui::{
        Alert,
        Anchor::{Left, Top},
        Color, Label, Setup, UIManager, ViewCallbacks, ViewData, ViewSubviews, view,
    },
};

static TOTAL_VIEWS: AtomicUsize = AtomicUsize::new(0);
static REPORTED: AtomicBool = AtomicBool::new(false);
static LOWEST_FPS: AtomicU32 = AtomicU32::new(f32::MAX.to_bits());
const TARGET_FPS: f32 = 30.0;

fn lowest_fps() -> f32 {
    f32::from_bits(LOWEST_FPS.load(Ordering::Relaxed))
}

fn update_fps() {
    let current_fps = UIManager::fps();
    let lowest = lowest_fps();

    if current_fps < lowest {
        LOWEST_FPS.store(current_fps.to_bits(), Ordering::Relaxed);
    }
}

#[view]
pub struct UIBenchmarkView {
    total_spawned: usize,

    #[educe(Default = Instant::now() + Duration::from_hours(1_000_000))]
    next_spawn: Instant,

    tl: Weak<Self>,
    tr: Weak<Self>,
    bl: Weak<Self>,
    br: Weak<Self>,

    #[init]
    label: Label,
}

impl Setup for UIBenchmarkView {
    fn setup(mut self: Weak<Self>) {
        TOTAL_VIEWS.fetch_add(1, Ordering::Relaxed);

        self.label
            .set_text(std::iter::repeat_with(fastrand::alphanumeric).take(10).collect::<String>());
        self.label.place().back();
        self.label.set_gradient(Color::random(), Color::random());
        self.label.set_text_color(Color::random());
        self.next_spawn = Instant::now() + Duration::from_secs_f32(0.5 + fastrand::f32());
        update_fps();
    }
}

impl ViewCallbacks for UIBenchmarkView {
    fn update(&mut self) {
        if lowest_fps() <= TARGET_FPS {
            if !REPORTED.load(Ordering::Relaxed) {
                Alert::show(format!("Total views: {}", TOTAL_VIEWS.load(Ordering::Relaxed)));
                REPORTED.store(true, Ordering::Relaxed);
            }
            return;
        }

        if self.total_spawned == 4 {
            return;
        }

        if self.next_spawn <= Instant::now() {
            self.spawn_next();
        }
    }
}

impl UIBenchmarkView {
    fn spawn_next(&mut self) {
        match self.total_spawned {
            0 => {
                self.tl = self.add_view();
                self.tl.place().tl(0).relative_size(weak_from_ref(self), 0.5);
            }
            1 => {
                self.tr = self.add_view();
                self.tr.place().tr(0).same_size(self.tl);
            }
            2 => {
                self.bl = self.add_view();
                self.bl.place().below(self.tl, 0);
            }
            3 => {
                self.br = self.add_view();
                self.br.place().anchor(Top, self.tr, 0).anchor(Left, self.bl, 0).br(0);
            }

            _ => unreachable!(),
        }

        self.total_spawned += 1;
        self.next_spawn = Instant::now() + Duration::from_secs_f32(0.5 + fastrand::f32());
    }
}
