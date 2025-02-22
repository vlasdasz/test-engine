use refs::Own;
use ui::View;

pub trait App {
    fn new() -> Self
    where Self: Sized;
    fn setup(&self);
    fn make_root_view(&self) -> Own<dyn View>;
}

unsafe extern "C" {
    #[allow(improper_ctypes_definitions)]
    #[allow(improper_ctypes)]
    pub fn test_engine_create_app() -> Box<dyn App>;
}

#[macro_export]
macro_rules! register_app {
    ($app:ty) => {
        #[unsafe(no_mangle)]
        #[allow(improper_ctypes_definitions)]
        pub extern "C" fn test_engine_create_app() -> Box<dyn test_engine::App> {
            use test_engine::App;

            fn check_trait<T: test_engine::App>() {}
            check_trait::<$app>();

            Box::new(<$app>::new())
        }
    };
}
