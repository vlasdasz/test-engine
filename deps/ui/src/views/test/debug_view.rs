use std::{collections::HashMap, fmt::Display};

use gm::{Color, Platform};
use refs::{MainLock, Own, Weak, dump_ref_stats};

use crate::has_data::HasText;

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use ui_proc::view;
use utils::Every;

use crate::{
    Button, Label, MovableView, Setup, ToLabel, TouchStack, UIManager, View, ViewCallbacks, ViewSubviews,
    view::{ViewData, ViewFrame, ViewInternalSetup},
};

pub(crate) static DEBUG_VIEW: MainLock<Option<Own<dyn View>>> = MainLock::new();

static CURRENT: MainLock<Weak<DebugView>> = MainLock::new();

#[view]
pub struct DebugView {
    custom_labels: HashMap<String, Weak<Label>>,

    #[init]
    fps_label:          Label,
    frame_drawn_label:  Label,
    screen_scale_label: Label,
    touch_enabled:      Label,
    exit:               Button,
    dump_mem:           Button,
    touch_root:         Label,

    hide: Button,
}

impl DebugView {
    pub fn enable() {
        let new = MovableView::<Self>::new();
        let mut container = new.weak();
        *DEBUG_VIEW.get_mut() = Some(new);
        container.__internal_before_setup();
        container.set_z_position(UIManager::DEBUG_Z_OFFSET);
        container.__manually_set_superview(UIManager::root_view_weak());
        container.init_views();
        container.__internal_setup();
        container.set_title("Debug");
        if Platform::MOBILE {
            container.set_y(400);
        }
        container.set_size(200, 280);
    }

    pub fn disable() {
        DEBUG_VIEW.get_mut().take();
    }

    pub fn custom_button(&mut self, label: impl ToLabel, action: impl FnMut() + Send + 'static) {
        let mut button = self.add_view::<Button>();
        button.set_text(label);
        button.on_tap(action);
    }

    pub fn set_custom(&mut self, label: impl Display, value: impl Display) {
        let label_text = label.to_string();

        let label = if let Some(label) = self.custom_labels.get_mut(&label_text) {
            label
        } else {
            let label_view = self.add_view::<Label>();
            self.custom_labels.insert(label_text.clone(), label_view);
            self.custom_labels.get_mut(&label_text).unwrap()
        };

        label.set_text(format!("{label_text}: {value}"));
    }
}

impl Setup for DebugView {
    fn setup(mut self: Weak<Self>) {
        *CURRENT.get_mut() = self;

        self.set_hidden(false);
        self.set_color(Color::WHITE);

        self.place().all_ver();

        self.fps_label.set_text("fps label");

        self.frame_drawn_label.set_text("frame drawn label");

        self.screen_scale_label.set_text("screen scale");

        self.exit.set_text("exit");
        self.exit.on_tap(|| {
            panic!("Exit pressed. Panic to stop the app. Bye.");
        });

        self.hide.set_text("hide");
        self.hide.on_tap(move || {
            DEBUG_VIEW.get_mut().take();
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
    }
}

impl ViewCallbacks for DebugView {
    fn update(&mut self) {
        Every::second(|| {
            self.fps_label.set_text(format!("FPS: {:.1}", UIManager::fps()));
        });

        self.frame_drawn_label.set_text(format!("Frames: {}", UIManager::frame_drawn()));

        let screen_scale = UIManager::display_scale();
        self.screen_scale_label.set_text(format!("Scale: {screen_scale}"));

        self.touch_enabled.set_text(format!("Touch: {}", !UIManager::touch_disabled()));

        self.touch_root.set_text(format!("Root: {}", TouchStack::root_name()));
    }
}
