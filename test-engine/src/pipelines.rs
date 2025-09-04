use log::debug;
use refs::main_lock::MainLock;
use render::UIRectPipepeline;

static PIPELINES: MainLock<Pipelines> = MainLock::new();

pub(crate) struct Pipelines {
    pub rect: UIRectPipepeline,
}

impl Pipelines {
    pub fn initialize() {
        assert!(!PIPELINES.is_set(), "Double pipelines init");

        PIPELINES.set(Pipelines {
            rect: UIRectPipepeline::default(),
        });

        debug!("pipelines ready");
    }

    fn get() -> &'static mut Self {
        PIPELINES.try_get_mut().expect("Pipelines not initialized yet")
    }

    pub fn rect() -> &'static mut UIRectPipepeline {
        &mut Self::get().rect
    }
}
