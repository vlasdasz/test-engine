use std::{collections::HashMap, fmt::Display};

use gm::Color;
use refs::{dump_ref_stats, MainLock, Own, Weak};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use ui_proc::view;
use utils::Every;

use crate::{
    view::{ViewData, ViewFrame, ViewInternalSetup},
    Button, Label, ToLabel, TouchStack, UIManager, View, ViewCallbacks, ViewSetup, ViewSubviews,
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
        let new = Self::new();
        let mut weak = new.weak();
        *DEBUG_VIEW.get_mut() = Some(new);
        let a = weak;
        weak.set_z_position(UIManager::DEBUG_Z_OFFSET);
        weak.__manually_set_superview(a);
        weak.init_views();
        weak.__internal_setup();
        weak.base_view().loaded.trigger(());
    }

    pub fn custom_button(&mut self, label: impl ToLabel, action: impl FnMut() + 'static) {
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

impl ViewSetup for DebugView {
    fn setup(mut self: Weak<Self>) {
        *CURRENT.get_mut() = self;

        self.set_hidden(false);
        self.set_color(Color::WHITE);

        self.__manually_set_superview(UIManager::root_view_weak());

        self.place().size(240, 220).l(10).b(10).all_ver();

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
