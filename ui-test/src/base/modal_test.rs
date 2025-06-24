use anyhow::Result;
use test_engine::{
    OnceEvent,
    dispatch::wait_for_next_frame,
    refs::Weak,
    ui::{
        Color, Container, HasText, Label, ModalView, Setup, Size, UI, ViewData, ViewFrame, ViewSubviews,
        WeakView, ui_test::helpers::check_colors, view,
    },
};

#[view]
struct ShowModally {}

impl Setup for ShowModally {
    fn setup(mut self: Weak<Self>) {
        let mut view = WeakView::default();

        for _ in 0..200 {
            if view.is_ok() {
                view = view.add_view::<Container>();
                view.set_color(Color::random()).place().all_sides(1);
            } else {
                view = self.add_dummy_view();
                view.set_color(Color::random()).place().tl(1).size(400, 400);
                assert_eq!(view.z_position(), 0.49_996_987);
            }
        }

        assert_eq!(view.z_position(), 0.49_797_717);
    }
}

#[view]
struct Modal {
    event: OnceEvent,

    #[init]
    label: Label,
}

impl Setup for Modal {
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
    UI::init_test_view::<ShowModally>().await;

    Modal::show_modally_with_input((), |()| {});

    wait_for_next_frame().await;

    check_colors(
        r#"
             158  295 - 255 255 255
             163  294 - 255 255 255
             188  292 - 255 255 255
             204  289 -   0   0   0
             215  289 -   0   0   0
             241  288 -   0   0   0
             250  288 - 108 108 108
             273  293 - 255 255 255
             283  293 - 255 255 255
             320  298 - 201 201 201
             330  300 - 255 255 255
             347  302 -   0   0   0
             371  299 - 255 255 255
             383  298 - 255 255 255
             397  293 - 255 255 255
             402  292 -  14  14  14
             419  287 - 255 255 255
             432  286 - 255 255 255
             444  287 - 255 255 255
        "#,
    )
    .await?;

    Ok(())
}
