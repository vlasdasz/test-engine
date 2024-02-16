use test_engine::{ui::ViewSetup, App};

use crate::interface::test_game_view::TestGameView;

pub fn start_test_game() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime
        .block_on(async {
            #[cfg(mobile)]
            test_engine::refs::set_current_thread_as_main();
            App::start(TestGameView::new()).await
        })
        .unwrap();
}
