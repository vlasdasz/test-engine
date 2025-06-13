use refs::MainLock;
use render::{UIImageRectPipepeline, UIPathPipeline, UIRectPipepeline};

pub(crate) static UI_RECT: MainLock<UIRectPipepeline> = MainLock::new();
pub(crate) static IMAGE_DRAWER: MainLock<UIImageRectPipepeline> = MainLock::new();

pub(crate) static PATH: MainLock<UIPathPipeline> = MainLock::new();
