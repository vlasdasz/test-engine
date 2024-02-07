#![cfg(desktop)]

use std::{future::Future, marker::PhantomData, path::PathBuf};

use anyhow::Result;
use gm::flat::IntSize;
use ui::{
    refs::{set_current_thread_as_main, Own},
    View, ViewTest,
};

use crate::{app::MakeApp, AppCore, OldApp};

pub struct ViewApp<T> {
    core: AppCore,
    _v:   PhantomData<*const T>,
}

impl<T: View + Default + 'static> ViewApp<T> {
    pub fn start() -> Result<()> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            set_current_thread_as_main();
            Self::make_app().launch()
        })
    }

    pub fn start_with_actor(_actions: impl Future<Output = Result<()>> + Send + 'static) -> Result<()> {
        // tokio::runtime::Runtime::new().unwrap().block_on(async {
        //     set_current_thread_as_main();
        //
        //     // Self::make_app().launch_with_callback(|| {
        //     //     spawn(async {
        //     //         SystemEvents::terminate(actions.await);
        //     //     });
        //     // })
        // })
        Ok(())
    }
}

impl<T: View + Default + 'static> OldApp for ViewApp<T> {
    fn setup()
    where Self: Sized {
    }

    fn screen_size() -> IntSize
    where Self: Sized {
        T::test_size()
    }

    fn make_root_view() -> Own<dyn View>
    where Self: Sized {
        let view = Own::<T>::default();
        view.weak().test_setup();
        view
    }

    fn with_core(core: AppCore) -> Self
    where Self: Sized {
        Self {
            core,
            _v: Default::default(),
        }
    }

    fn core(&mut self) -> &mut AppCore {
        &mut self.core
    }

    fn assets_path() -> PathBuf
    where Self: Sized {
        Default::default()
    }
}

#[macro_export]
macro_rules! test_view {
    ($view_type:ty) => {
        #[ignore]
        #[test]
        fn test_view() {
            ViewApp::<$view_type>::start();
        }
    };
}

#[ignore]
#[test]
fn test() -> Result<()> {
    ViewApp::<ui::Container>::start()
}
