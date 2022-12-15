use refs::ToWeak;
use ui::{view, Property, SubView, UIManager, ViewCallbacks, ViewData, ViewFrame, ViewSubviews};

use crate::Label;

#[view]
#[derive(Default)]
pub struct DebugView {
    fps_label:          SubView<Label>,
    frame_drawn_label:  SubView<Label>,
    url_label:          SubView<Label>,
    ui_scale_label:     SubView<Label>,
    screen_scale_label: SubView<Label>,
    root_frame:         SubView<Label>,

    pub fps: Property<u64>,
    pub url: Property<String>,

    frame_drawn: u64,
}

impl ViewCallbacks for DebugView {
    fn setup(&mut self) {
        self.set_hidden(false);

        self.manually_set_superview(UIManager::root_view());

        self.place.size(400, 200).bl(10).all_ver();

        self.fps_label.set_text("fps label");
        self.fps_label.free_text = true;

        self.frame_drawn_label.set_text("frame drawn label");
        self.frame_drawn_label.free_text = true;

        self.ui_scale_label.set_text("ui scale");
        self.ui_scale_label.free_text = true;

        self.screen_scale_label.set_text("screen scale");
        self.screen_scale_label.free_text = true;

        self.root_frame.set_text("root frame");
        self.root_frame.free_text = true;

        let mut this = self.weak();
        self.fps.on_set.sub(move |fps| {
            this.fps_label.set_text(format!("FPS: {fps}"));
        });

        self.url.on_set.sub(move |url| {
            this.url_label.set_text(url);
        });
    }

    fn update(&mut self) {
        self.frame_drawn += 1;
        self.frame_drawn_label.set_text(format!("Frame drawn: {}", self.frame_drawn));

        // let ui_scale = UIManager::ui_scale();
        // self.ui_scale_label.set_text(format!("UI scale: {ui_scale}"));

        let screen_scale = UIManager::display_scale();
        self.screen_scale_label.set_text(format!("Screen scale: {screen_scale}"));

        self.root_frame.set_text(format!(
            "Root frame: {:?}",
            UIManager::root_view().frame().short_display()
        ));
    }
}
