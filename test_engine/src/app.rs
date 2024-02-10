use std::ops::{Deref, DerefMut};

use anyhow::Result;
use gm::{
    flat::{IntSize, Point, Rect},
    Color,
};
use log::warn;
use manage::data_manager::DataManager;
use refs::{Own, Weak};
use rtools::Random;
use ui::{
    check_touch,
    input::{TouchEvent, UIEvents},
    Container, Touch, TouchStack, UIManager, View, ViewAnimation, ViewData, ViewFrame, ViewLayout, ViewSetup,
    ViewSubviews,
};
use ui_views::{ImageView, Label};
use wgpu::RenderPass;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, Section, Text, VerticalAlign};
use wgpu_wrapper::{ElementState, Font, MouseButton, WGPUApp, WGPUDrawer};

use crate::{assets::Assets, git_root};

pub struct App {
    cursor_position:       Point,
    root_view:             Weak<dyn View>,
    pub(crate) first_view: Option<Own<dyn View>>,
}

impl App {
    pub async fn start(first_view: Own<dyn View>, width: u32, height: u32) -> Result<()> {
        Assets::init(git_root().expect("git_root()"));
        WGPUApp::start(Self::new(first_view), width, height).await
    }

    fn new(first_view: Own<dyn View>) -> Self {
        Self {
            cursor_position: Default::default(),
            root_view:       UIManager::root_view(),
            first_view:      first_view.into(),
        }
    }

    fn rescale_frame(rect: &Rect, display_scale: f32) -> Rect {
        rect * display_scale
    }

    fn update_view(&self, view: &mut dyn View) -> bool {
        if view.is_hidden() {
            return false;
        }
        view.layout();
        let mut animations = view.commit_animations();
        view.calculate_absolute_frame();
        view.update();
        for view in view.subviews_mut() {
            animations = animations || self.update_view(view.deref_mut());
        }
        animations
    }

    fn draw<'a>(
        &'a self,
        pass: &mut RenderPass<'a>,
        drawer: &'a WGPUDrawer,
        view: &'a dyn View,
        sections: &mut Vec<Section<'a>>,
    ) {
        if view.is_hidden() {
            return;
        }

        if view.absolute_frame().size.is_invalid() {
            warn!(
                "View has invalid frame: {}. Frame: {:?} ",
                view.label(),
                view.frame()
            );
            return;
        }

        let frame = Self::rescale_frame(view.absolute_frame(), 1.0);

        if !frame.origin.positive() {
            warn!("A");
            return;
        }

        drawer.fill_rect(pass, &frame, view.color());

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if image_view.image.is_ok() {
                let image = image_view.image;
                // let size: Size = image.size.into();
                // let frame = &size.fit_in_rect::<{ Axis::X }>(view.absolute_frame());
                // let frame = Self::rescale_frame(frame, 1.0, drawer.window_size);

                drawer.draw_image(pass, image.get_static(), &frame);
            }
        } else if let Some(label) = view.as_any().downcast_ref::<Label>()
            && !label.text.is_empty()
        {
            let center = frame.center();

            let section = Section::default()
                .add_text(
                    Text::new(&label.text)
                        .with_scale(label.text_size())
                        .with_color(Color::BLACK.as_slice()),
                )
                .with_bounds((frame.width(), frame.height()))
                .with_layout(
                    Layout::default()
                        .v_align(VerticalAlign::Center)
                        .h_align(HorizontalAlign::Center)
                        .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
                )
                .with_screen_position((center.x, center.y));

            sections.push(section);
        }

        for view in view.subviews() {
            if view.dont_hide() || view.absolute_frame().intersects(UIManager::root_view().frame()) {
                self.draw(pass, drawer, view.deref(), sections)
            }
        }
    }

    fn touch_event(&mut self, mut touch: Touch) -> bool {
        const LOG_TOUCHES: bool = false;
        const DISPLAY_TOUCHES: bool = false;

        if UIManager::touch_disabled() {
            return false;
        }

        UIEvents::get().on_touch.trigger(touch);

        if LOG_TOUCHES && !touch.is_moved() {
            warn!("{touch:?}");
        }

        if DISPLAY_TOUCHES && !touch.is_moved() {
            let mut view = Container::new();
            view.set_size((5, 5)).set_color(Color::random());
            view.set_center(touch.position);
            UIManager::root_view().add_subview(view);
        }

        let _level_touch = touch;
        // TODO: Revisit scale
        // if Platform::DESKTOP {
        //     touch.position = self.cursor_position / UIManager::ui_scale();
        // } else {
        //     touch.position /= UIManager::ui_scale();
        // }

        for view in TouchStack::touch_views() {
            if check_touch(view, &mut touch) {
                return true;
            }
        }

        // if let Some(level) = &mut self.level {
        //     level.set_cursor_position(level_touch.position);
        //     if touch.is_began() {
        //         level.add_touch(level_touch.position)
        //     }
        // }

        return DISPLAY_TOUCHES;
    }
}

impl wgpu_wrapper::App for App {
    fn window_ready(&mut self) {
        let view = UIManager::root_view().add_subview(self.first_view.take().unwrap());
        view.place().back();
        self.update();
    }

    fn update(&mut self) -> bool {
        dbg!(format!("update {}", u32::random()));
        self.update_view(UIManager::root_view().deref_mut())
    }

    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer) {
        let mut sections: Vec<Section> = vec![];
        self.draw(pass, drawer, self.root_view.deref(), &mut sections);

        Font::helvetice().brush.queue(&drawer.device, &drawer.queue, sections).unwrap()
    }

    fn resize(&mut self, size: IntSize) {
        UIManager::root_view().set_size(size);
        self.update();
    }

    fn mouse_moved(&mut self, position: Point) -> bool {
        self.cursor_position = position;
        self.touch_event(Touch {
            id: 1,
            position,
            event: TouchEvent::Moved,
            button: MouseButton::Left,
        })
    }

    fn mouse_event(&mut self, state: ElementState, button: MouseButton) -> bool {
        self.touch_event(Touch {
            id: 1,
            position: self.cursor_position,
            event: state.into(),
            button,
        })
    }
}
