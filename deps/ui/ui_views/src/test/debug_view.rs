use std::{
    collections::HashMap,
    fmt::Display,
    sync::{atomic::AtomicBool, OnceLock},
};

use gm::Color;
use refs::{dump_ref_stats, Weak};
use ui::{
    view, Event, SubView, ToLabel, TouchStack, UIManager, ViewCallbacks, ViewData, ViewFrame, ViewSetup,
    ViewSubviews,
};

use crate::{Button, Label};

pub static SHOW_DEBUG_VIEW: AtomicBool = AtomicBool::new(false);

static CURRENT: OnceLock<Weak<DebugView>> = OnceLock::new();

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

    hide: SubView<Button>,

    custom_labels: HashMap<String, SubView<Label>>,

    pub fps: Event<u64>,

    frame_drawn: u64,
}

impl DebugView {
    pub fn current() -> Weak<Self> {
        *CURRENT.get().unwrap()
    }

    pub fn custom_button(&mut self, label: impl ToLabel, action: impl FnMut() + 'static) {
        let mut button = self.__internal_add_view::<Button>();
        button.set_text(label);
        button.on_tap(action);
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
        CURRENT.set(self).unwrap();

        self.set_hidden(false);
        self.set_color(Color::WHITE);

        self.manually_set_superview(UIManager::root_view());

        self.place().size(400, 200).bl(10).all_ver();

        self.fps_label.set_text("fps label");

        self.frame_drawn_label.set_text("frame drawn label");

        self.ui_scale_label.set_text("ui scale");

        self.screen_scale_label.set_text("screen scale");

        self.root_frame.set_text("root frame");

        self.exit.set_text("exit");
        self.exit.on_tap(|| {
            panic!("Exit pressed. Panic to stop the app. Bye.");
        });

        self.hide.set_text("hide");
        self.hide.on_tap(move || {
            self.remove_from_superview();
        });

        self.dump_mem.set_text("dump mem");
        self.dump_mem.on_tap(|| {
            // dbg!(Image::storage().len());
            //
            // for (_key, val) in Image::storage() {
            //     dbg!(val.name());
            // }

            dump_ref_stats();
        });

        self.fps.val(move |fps| {
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
