use std::{collections::HashMap, fmt::Display};

use refs::{dump_ref_stats, Weak};
use ui::{
    view, Property, SubView, ToLabel, TouchStack, UIManager, ViewCallbacks, ViewData, ViewFrame, ViewSetup,
    ViewSubviews,
};

use crate::{Button, Label};

#[view]
pub struct DebugView {
    fps_label:          SubView<Label>,
    frame_drawn_label:  SubView<Label>,
    ui_scale_label:     SubView<Label>,
    screen_scale_label: SubView<Label>,
    root_frame:         SubView<Label>,
    touch_enabled:      SubView<Label>,
    exit:               SubView<Button>,
    dump_mem:           SubView<Button>,
    touch_root:         SubView<Label>,

    custom_labels: HashMap<String, SubView<Label>>,

    pub fps: Property<u64>,

    frame_drawn: u64,
}

impl DebugView {
    pub fn custom_button(&mut self, label: impl ToLabel, action: impl FnMut() + 'static) {
        let mut button = self.__internal_add_view::<Button>();
        button.set_text(label);
        button.on_tap.sub(action);
    }

    pub fn set_custom(&mut self, label: impl Display, value: impl Display) {
        let label_text = label.to_string();

        let label = if let Some(label) = self.custom_labels.get_mut(&label_text) {
            label
        } else {
            let label_view = self.__internal_add_view::<Label>();
            self.custom_labels.insert(label_text.clone(), label_view);
            self.custom_labels.get_mut(&label_text).unwrap()
        };

        label.set_text(format!("{label_text}: {value}"));
    }
}

impl ViewSetup for DebugView {
    fn setup(mut self: Weak<Self>) {
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

        self.exit.set_text("exit");
        self.exit.on_tap.sub(|| {
            panic!("bye");
        });

        self.dump_mem.set_text("dump mem");
        self.dump_mem.on_tap.sub(|| {
            dump_ref_stats();
        });

        self.fps.on_set.val(move |fps| {
            self.fps_label.set_text(format!("FPS: {fps}"));
        });
    }
}

impl ViewCallbacks for DebugView {
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

        self.touch_enabled
            .set_text(format!("Touch enabled: {}", !UIManager::touch_disabled()));

        self.touch_root.set_text(format!("Touch root: {}", TouchStack::root_name()));
    }
}
