use test_engine::{
    refs::Weak,
    ui::{Container, LIGHT_BLUE, NumberView, Setup, Style, UIManager, ViewData, ViewSubviews, view},
};

use crate::interface::test_game_view::HAS_BACK_BUTTON;

const CORNER_STYLE: Style = Style::new(|v| {
    v.set_color(LIGHT_BLUE).place().size(80, 80);
});

#[view]
pub struct RootLayoutView {
    #[init]
    scale: NumberView,
}

impl Setup for RootLayoutView {
    fn setup(self: Weak<Self>) {
        UIManager::enable_debug_frames();
        UIManager::root_view().set_image("square.png");

        self.apply_style(HAS_BACK_BUTTON);

        self.add_view::<Container>().apply_style(CORNER_STYLE).place().tl(0);
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().tr(0);
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().br(0);
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().bl(0);

        self.add_view::<Container>().apply_style(CORNER_STYLE).place().t(0).center_x();
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().l(0).center_y();
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().r(0).center_y();
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().b(0).center_x();

        self.scale
            .set_min(0.2)
            .set_step(0.1)
            .set_value(1)
            .place()
            .center()
            .size(100, 200);
        self.scale.on_change(|scale| {
            UIManager::set_scale(scale);
        });
    }
}

impl Drop for RootLayoutView {
    fn drop(&mut self) {
        UIManager::disable_debug_frames();
    }
}

pub mod test {

    use anyhow::Result;
    use ctor::ctor;
    use futures::FutureExt;
    use test_engine::{
        ui::view_test,
        ui_test::{UITest, record_ui_test},
    };

    use super::{RootLayoutView, Setup, ViewData, Weak};

    #[view_test]
    struct RootLayoutViewTest {
        #[init]
        view: RootLayoutView,
    }

    impl Setup for RootLayoutViewTest {
        fn setup(self: Weak<Self>) {
            self.view.place().back();
        }
    }

    #[ctor]
    fn store_test() {
        crate::UI_TESTS
            .lock()
            .insert("RootLayoutViewTest".to_string(), || test().boxed());
    }

    macro_rules! ui_test {
        () => {
            #[test]
            fn ui_test() -> anyhow::Result<()> {
                let mut child = std::process::Command::new("cargo")
                    .args([
                        "run",
                        "-p",
                        "ui-test",
                        "--target-dir",
                        "../target/ui_tests",
                        "--",
                        "--test-name",
                        "RootLayoutViewTest",
                    ])
                    .stdin(std::process::Stdio::inherit())
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .spawn()?;

                let status = child.wait()?;

                if !status.success() {
                    std::process::exit(status.code().unwrap_or(1));
                }

                Ok(())
            }
        };
    }

    ui_test!();

    #[allow(clippy::unused_async)]
    pub async fn test() -> Result<()> {
        UITest::start::<RootLayoutViewTest>();

        record_ui_test();

        Ok(())
    }
}
