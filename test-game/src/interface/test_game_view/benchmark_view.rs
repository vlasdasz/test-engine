use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use test_engine::{
    AppRunner,
    dispatch::{from_main, on_main},
    refs::Weak,
    ui::{Alert, Anchor, Color, Label, Setup, ViewData, ViewSubviews, view},
};

use crate::interface::test_game_view::HAS_BACK_BUTTON;

static FINISHED: AtomicBool = AtomicBool::new(false);
static VIEWS_COUNT: AtomicU64 = AtomicU64::new(0);

const TARGET_FPS: f32 = 40.0;

#[view]
pub struct UIBenchmarkView {
    index: u32,

    #[init]
    label: Label,
}

impl UIBenchmarkView {
    fn filled(&self) -> bool {
        self.index == 4
    }

    fn add_bench_view(mut self: Weak<Self>) {
        if AppRunner::fps() < TARGET_FPS {
            return;
        }

        VIEWS_COUNT.fetch_add(1, Ordering::AcqRel);

        let view = self.add_view::<UIBenchmarkView>();
        view.place().relative(Anchor::Width, self, 0.5);
        view.place().relative(Anchor::Height, self, 0.5);
        match self.index {
            0 => view.place().t(0).l(0),
            1 => view.place().t(0).r(0),
            2 => view.place().b(0).r(0),
            3 => view.place().b(0).l(0),
            _ => unreachable!(),
        };

        self.index += 1;
    }

    #[allow(clippy::unused_self)]
    fn start_spawning_views(self: Weak<Self>) {
        test_engine::dispatch::spawn(async move {
            for _ in 0..5 {
                test_engine::dispatch::sleep(0.5).await;

                if FINISHED.load(Ordering::Relaxed) {
                    return;
                }

                let finish = from_main(move || {
                    let filled = self.filled();

                    if !filled {
                        self.add_bench_view();
                    }

                    filled
                });

                if finish {
                    on_main(move || {
                        if AppRunner::fps() < TARGET_FPS {
                            if FINISHED.load(Ordering::Relaxed) {
                                return;
                            }

                            Alert::show_callback(
                                format!("Views: {}", VIEWS_COUNT.load(Ordering::Acquire)),
                                move || {
                                    self.apply_style(HAS_BACK_BUTTON);
                                },
                            );

                            FINISHED.store(true, Ordering::Relaxed);

                            return;
                        }

                        for view in self.subviews() {
                            let Some(be) = view.downcast::<Self>() else {
                                continue;
                            };
                            be.start_spawning_views();
                        }
                    });
                    return;
                }
            }
        });
    }
}

impl Setup for UIBenchmarkView {
    fn setup(self: Weak<Self>) {
        self.label.set_text("djkshdsjkhjkds");
        self.label.place().back();
        self.label.set_gradient(Color::random(), Color::random());
        self.label.set_text_color(Color::random());
        self.start_spawning_views();
    }
}
