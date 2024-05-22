use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{
        ui_test::helpers::check_colors, view, Color, Container, Label, ModalView, Size, Sub, ViewData,
        ViewFrame, ViewSetup, ViewSubviews, WeakView, UI,
    },
    wait_for_next_frame, OnceEvent,
};

#[view]
struct ModalTestView {}

impl ViewSetup for ModalTestView {
    fn setup(mut self: Weak<Self>) {
        let mut view = WeakView::default();

        for _ in 0..280 {
            if view.is_ok() {
                view = view.add_view::<Container>();
                view.set_color(Color::random()).place().all_sides(1);
            } else {
                view = self.add_dummy_view();
                view.set_color(Color::random()).place().all_sides(1);
                assert_eq!(view.z_position(), 0.49999797);
            }
        }

        assert_eq!(view.z_position(), 0.49971527);
    }
}

#[view]
struct Modal {
    label: Sub<Label>,
    event: OnceEvent,
}

impl ViewSetup for Modal {
    fn setup(mut self: Weak<Self>) {
        self.label.place().back();
        self.label.set_text_size(100);
        self.label.set_text("Hello");
    }
}

impl ModalView for Modal {
    fn modal_event(&self) -> &OnceEvent<()> {
        &self.event
    }

    fn modal_size() -> Size {
        (400, 400).into()
    }
}

pub async fn test_modal() -> Result<()> {
    UI::init_test_view::<ModalTestView>().await;

    Modal::show_modally((), |_| {});

    wait_for_next_frame().await;

    check_colors(
        r#"
             156  279 - 255 255 255
             170  282 - 255 255 255
             188  284 - 255 255 255
             210  289 -   0   0   0
             223  290 -   0   0   0
             252  293 - 255 255 255
             271  296 -  85  85  85
             308  300 -   0   0   0
             332  301 - 255 255 255
             347  302 -   0   0   0
             362  302 -   1   1   1
             382  303 - 255 255 255
             400  304 - 113 113 113
             421  302 - 255 255 255
             426  298 - 255 255 255
             429  267 - 255 255 255
             395  248 - 255 255 255
             364  250 - 255 255 255
             357  265 - 255 255 255
             334  285 - 255 255 255
             313  293 - 255 255 255
             287  302 -   0   0   0
             252  322 - 255 255 255
             211  348 - 255 255 255
             194  397 - 255 255 255
             197  414 - 255 255 255
             311  435 - 255 255 255
             457  135 - 255 255 255
             158  142 - 255 255 255
             148  443 - 255 255 255
    "#,
    )
    .await?;

    debug!("Modal test: OK");

    Ok(())
}
