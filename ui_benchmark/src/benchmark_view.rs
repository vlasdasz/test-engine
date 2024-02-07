// use std::sync::atomic::{AtomicU64, Ordering};
//
// use old_engine::{from_main, gm::flat::IntSize, on_main, rtools::Random,
// ui_layer::UILayer}; use rtools::sleep;
// use tokio::spawn;
// use ui::{layout::Anchor, refs::Weak, view, SubView, ViewData, ViewSetup,
// ViewSubviews, ViewTest}; use ui_views::Label;
//
// static VIEWS_COUNT: AtomicU64 = AtomicU64::new(0);
//
// #[view]
// pub struct BenchmarkView {
//     label: SubView<Label>,
//     index: u32,
// }
//
// impl BenchmarkView {
//     fn filled(&self) -> bool {
//         self.index == 4
//     }
//
//     fn add_bench_view(mut self: Weak<Self>) {
//         if UILayer::get().fps < 60 {
//             return;
//         }
//
//         VIEWS_COUNT.fetch_add(1, Ordering::Relaxed);
//
//         let view = self.add_view::<BenchmarkView>();
//         view.place().relative(Anchor::Width, self, 0.5);
//         view.place().relative(Anchor::Height, self, 0.5);
//         match self.index {
//             0 => view.place().t(0).l(0),
//             1 => view.place().t(0).r(0),
//             2 => view.place().b(0).r(0),
//             3 => view.place().b(0).l(0),
//             _ => unreachable!(),
//         };
//
//         self.index += 1;
//     }
//
//     fn start_spawning_views(self: Weak<Self>) {
//         spawn(async move {
//             loop {
//                 sleep(0.1);
//                 let finish = from_main(move || {
//                     let filled = self.filled();
//
//                     if !filled {
//                         self.add_bench_view();
//                     }
//
//                     filled
//                 })
//                 .await;
//
//                 if finish {
//                     on_main(move || {
//                         if UILayer::get().fps < 60 {
//                             dbg!(VIEWS_COUNT.load(Ordering::Relaxed));
//                             return;
//                         }
//
//                         for view in self.subviews() {
//                             let Some(be) = view.downcast::<Self>() else {
//                                 continue;
//                             };
//                             be.start_spawning_views();
//                         }
//                     });
//                     return;
//                 }
//             }
//         });
//     }
// }
//
// impl ViewSetup for BenchmarkView {
//     fn setup(mut self: Weak<Self>) {
//         self.label.set_text(String::random());
//         self.label.place().back();
//         self.start_spawning_views();
//     }
// }
//
// impl ViewTest for BenchmarkView {
//     fn test_size() -> IntSize
//     where Self: Sized {
//         (600, 600).into()
//     }
// }
