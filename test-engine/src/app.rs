use gm::flat::Size;
use refs::Own;
use ui::View;

pub trait App {
    fn new() -> Self
    where Self: Sized;
    fn setup(&self) {}
    fn make_root_view(&self) -> Own<dyn View>;
    fn initial_size(&self) -> Size {
        (1200, 1000).into()
    }
}

#[cfg(ios)]
unsafe extern "C" {
    #[allow(improper_ctypes_definitions)]
    #[allow(improper_ctypes)]
    pub fn test_engine_create_app() -> Box<dyn App>;
}

#[cfg(not(ios))]
#[unsafe(no_mangle)]
#[linkage = "weak"]
#[allow(improper_ctypes_definitions)]
#[allow(improper_ctypes)]
pub extern "C" fn test_engine_create_app() -> Box<dyn App> {
    panic!("you need to use test_engine::register_app!(YourApp) macro")
}

#[macro_export]
macro_rules! register_app {
    ($app:ty) => {
        pub use test_engine;

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
