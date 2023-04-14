use std::{marker::PhantomData, path::PathBuf};

use gm::flat::Size;
use ui::{refs::Own, View};

use crate::{app::MakeApp, App, AppCore};

pub struct ViewApp<T> {
    core: AppCore,
    _v:   PhantomData<*const T>,
}

impl<T: View + Default + 'static> ViewApp<T> {
    pub fn start() {
        Self::make_app().launch();
    }
}

impl<T: View + Default + 'static> App for ViewApp<T> {
    fn screen_size() -> Size
    where Self: Sized {
        T::expected_size()
    }

    fn assets_path() -> PathBuf
    where Self: Sized {
        Default::default()
    }

    fn make_root_view() -> Own<dyn View>
    where Self: Sized {
        Own::<T>::default()
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
}

#[cfg(test)]
mod tes {
    use ui::{
        refs::{is_main_thread, thread_id},
        Container,
    };

    use crate::ViewApp;

    #[test]
    fn test() {
        dbg!(thread_id());

        dbg!(is_main_thread());

        ViewApp::<Container>::start();
    }
}
