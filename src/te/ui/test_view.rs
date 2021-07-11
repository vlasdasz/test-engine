use crate::gm::{Color, Rect};
use crate::image::Image;
use crate::ui::basic::Button;
use crate::ui::view::View;
use crate::ui::{ImageView, Label, Layout, ViewBase};
use std::any::Any;
use tools::refs::{make_shared, Shared};
use tools::{AsAny, HasNew};

static mut COUNTER: u32 = 0;

#[derive(Debug)]
pub struct TestView {
    base: ViewBase,
    pub data: u128,
    pub clicks: u128,
    pub image: Option<Shared<ImageView>>,
    pub label: Option<Shared<Label>>,
}

impl View for TestView {
    fn setup(&mut self, _this: Shared<dyn View>) {
        self.set_frame(Rect::make(10, 10, 1000, 500));

        let mut cat_image = ImageView::new();
        cat_image.image = Image::load(&crate::te::paths::images().join("cat.jpg"));
        cat_image.set_frame(Rect::make(200, 20, 100, 120));
        let shared_cat = make_shared(cat_image);
        self.image = Some(shared_cat.clone());
        self.add_subview(shared_cat);

        let mut label = Label::from_rect(Rect::make(5, 200, 100, 100));
        label.set_text("ti stragadag stragadag4naja stragadag stragadag stragadakt4ka");

        let shared_label = make_shared(label);
        self.label = Some(shared_label.clone());
        self.add_subview(shared_label.clone());

        let mut view = ViewBase::new();
        view.set_frame(Rect::make(10, 20, 50, 50));
        view.set_color(Color::WHITE);

        let mut button = Button::new();
        button.set_frame(Rect::make(10, 10, 20, 20));
        button.set_color(Color::RED);

        button.on_tap.subscribe(move |_| {
            // match this.try_borrow_mut() {
            //     Ok(this) => {
            //         dbg!(this);
            //     },
            //     Err(error) => {
            //         dbg!(error);
            //     }
            // };

            // let mut this = this.try_borrow_mut().unwrap();
            // let this = this.as_any_mut().downcast_mut::<Self>().unwrap();
            // this.clicks += 1;
            unsafe {
                shared_label
                    .borrow_mut()
                    .set_text(&format!("kok: {}", COUNTER));
                COUNTER += 1;
            }
        });

        view.add_subview(make_shared(button));

        self.add_subview(make_shared(view));
    }

    fn update(&mut self) {
        guard!(let Some(_label) = &self.label else {
           return;
        });

        // let mut label = label.try_borrow_mut().unwrap();
        //
        // label.set_text(&format!(
        //     "ti stragadag stragadag4naja stragadag stragadag stragadakt4ka: {}",
        //     self.data
        // ));

        self.data += 1;
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }

    fn layout(&mut self, _super_frame: &Rect) {
        Layout::br(self.frame_mut(), _super_frame);
    }
}

impl HasNew for TestView {
    fn new() -> Self {
        TestView {
            base: ViewBase::new(),
            data: 0,
            clicks: 0,
            image: None,
            label: None,
        }
    }
}

impl AsAny for TestView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
