use crate::gm::Size;
use crate::te::Screen;
use crate::te::ScreenBase;

pub struct TestScreen {
    screen: ScreenBase
}

impl Screen<TestScreen> for TestScreen {
    fn with_size(size: Size) -> TestScreen {
        let screen = TestScreen { screen: ScreenBase::with_size(size) };
        screen.screen.start_main_loop(|| {
            println!("A");
        });
        screen
    }
}