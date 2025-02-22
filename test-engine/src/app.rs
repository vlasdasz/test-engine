use refs::Own;
use ui::View;

pub trait App {
    fn setup(&self);
    fn make_root_view(&self) -> Own<dyn View>;
}

unsafe extern "C" {
    #[allow(improper_ctypes_definitions)]
    #[allow(improper_ctypes)]
    pub fn test_engine_create_app() -> Box<dyn App>;
}
