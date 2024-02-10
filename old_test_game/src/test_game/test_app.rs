use std::path::PathBuf;

use log::trace;
use old_engine::{app::AppCore, gm::flat::IntSize, paths::git_root, OldApp};
use refs::{enable_ref_stats_counter, Own};
use ui::{Container, NavigationView, View, ViewSetup};

pub struct TestApp {
    core: AppCore,
}

impl OldApp for TestApp {
    fn setup() {
        enable_ref_stats_counter(true);
        trace!("TestApp setup: OK");
    }

    fn screen_size() -> IntSize {
        (1000, 600).into()
    }

    fn make_root_view() -> Own<dyn View> {
        trace!("make_root_view");
        NavigationView::with_view(Container::new())
    }

    fn with_core(core: AppCore) -> Self
    where Self: Sized {
        Self { core }
    }

    fn core(&mut self) -> &mut AppCore {
        &mut self.core
    }

    fn assets_path() -> PathBuf {
        git_root().expect("git_root()")
    }
}
