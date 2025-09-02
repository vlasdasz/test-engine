use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use test_engine::{
    AppRunner,
    refs::Weak,
    ui::{Anchor, Color, HasText, Label, Setup, ViewData, ViewSubviews, view},
};

static _FINISHED: AtomicBool = AtomicBool::new(false);
static VIEWS_COUNT: AtomicU64 = AtomicU64::new(0);

const TARGET_FPS: f32 = 40.0;

#[view]
pub struct BenchmarkView {
    index: u32,

    #[init]
    label: Label,
}

impl BenchmarkView {
    fn filled(&self) -> bool {
        self.index == 4
    }

    fn add_bench_view(mut self: Weak<Self>) {
        if AppRunner::fps() < TARGET_FPS {
            return;
        }

        VIEWS_COUNT.fetch_add(1, Ordering::Relaxed);

        let view = self.add_view::<BenchmarkView>();
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
        // spawn(async move {
        //     loop {
        //         sleep(Duration::from_secs_f32(0.05));
        //         let finish = from_main(move || {
        //             let filled = self.filled();
        //
        //             if !filled {
        //                 self.add_bench_view();
        //             }
        //
        //             filled
        //         })
        //         .await;
        //
        //         if finish {
        //             on_main(move || {
        //                 if AppRunner::fps() < TARGET_FPS {
        //                     if FINISHED.load(Ordering::Relaxed) {
        //                         return;
        //                     }
        //
        //                     Alert::show(format!("Views: {}",
        // VIEWS_COUNT.load(Ordering::Relaxed)));
        //
        //                     FINISHED.store(true, Ordering::Relaxed);
        //                     return;
        //                 }
        //
        //                 for view in self.subviews() {
        //                     let Some(be) = view.downcast::<Self>() else {
        //                         continue;
        //                     };
        //                     be.start_spawning_views();
        //                 }
        //             });
        //             return;
        //         }
        //     }
        // });
    }
}

impl Setup for BenchmarkView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("djkshdsjkhjkds");
        self.label.place().back();
        self.label.set_gradient(Color::random(), Color::random());
        self.label.set_text_color(Color::random());
        self.start_spawning_views();
    }
}
