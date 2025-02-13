use std::ops::{Deref, DerefMut};

use dispatch::{from_main, wait_for_next_frame};
use gm::{
    Color,
    flat::{Rect, Size},
};
use log::{trace, warn};
use refs::{MainLock, Own, Weak};
use render::{
    PathPipeline, UIImageRectPipepeline, UIRectPipepeline, rect_instance::RectInstance, rect_view::RectView,
};
use ui::{
    DrawingView, HasText, ImageView, Label, Setup, TextAlignment, UIManager, View, ViewAnimation, ViewData,
    ViewFrame, ViewLayout, ViewSubviews, ViewTest,
};
use wgpu::RenderPass;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, Section, Text, VerticalAlign};
use window::{Font, Window};

use crate::{App, ui::ui_test::state::clear_state};

static RECT_DRAWER: MainLock<UIRectPipepeline> = MainLock::new();
static IMAGE_RECT_DRAWER: MainLock<UIImageRectPipepeline> = MainLock::new();
static PATH: MainLock<PathPipeline> = MainLock::new();

pub struct UI;

impl UI {
    pub(crate) fn update() {
        Self::update_view(UIManager::root_view_weak().deref_mut());
        if let Some(debug_view) = UIManager::debug_view() {
            Self::update_view(debug_view);
        }
    }

    pub(crate) fn draw(pass: &mut RenderPass) {
        let mut sections: Vec<Section> = vec![];
        let debug_frames = UIManager::draw_debug_frames();
        Self::draw_view(
            pass,
            UIManager::root_view(),
            &mut sections,
            &mut 0.0,
            debug_frames,
        );
        if let Some(debug_view) = UIManager::debug_view() {
            Self::draw_view(pass, debug_view, &mut sections, &mut 0.0, debug_frames);
        }

        let window_size = UIManager::resolution();

        pass.set_viewport(0.0, 0.0, window_size.width, window_size.height, 0.0, 1.0);

        RECT_DRAWER.get_mut().draw(
            pass,
            RectView {
                resolution: UIManager::resolution(),
            },
        );

        IMAGE_RECT_DRAWER.get_mut().draw(
            pass,
            RectView {
                resolution: UIManager::resolution(),
            },
        );

        Font::helvetice()
            .brush
            .queue(Window::device(), Window::queue(), sections)
            .unwrap();
    }

    fn update_view(view: &mut dyn View) {
        if view.is_hidden() {
            return;
        }
        view.layout();
        view.commit_animations();
        view.calculate_absolute_frame();
        view.update();
        view.trigger_events();
        for mut view in view.subviews_mut() {
            Self::update_view(view.deref_mut());
        }
    }

    fn draw_view<'a>(
        pass: &mut RenderPass<'a>,
        view: &'a dyn View,
        sections: &mut Vec<Section<'a>>,
        text_offset: &mut f32,
        debug_frames: bool,
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

        view.before_render(pass);

        let frame = *view.absolute_frame();

        let root_size = UI::root_view_size();

        let clamped_frame = frame.clamp_to(root_size);

        let window_size = UIManager::resolution();

        if view.color().a > 0.0 {
            pass.set_viewport(0.0, 0.0, window_size.width, window_size.height, 0.0, 1.0);

            RECT_DRAWER.get_mut().add(RectInstance::new(
                frame,
                *view.color(),
                view.z_position() + *text_offset,
            ));
        }

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if image_view.image().is_ok() {
                let image = image_view.image();
                // let size: Size = image.size.into();
                // let frame = &size.fit_in_rect::<{ Axis::X }>(view.absolute_frame());
                // let frame = Self::rescale_frame(frame, 1.0, drawer.window_size);

                IMAGE_RECT_DRAWER.get_mut().add_with_image(
                    RectInstance {
                        position:   frame.origin,
                        size:       frame.size,
                        color:      Color::default(),
                        rotation:   0.0,
                        z_position: view.z_position() - UIManager::additional_z_offset(),
                    },
                    image,
                );
            } else {
                warn!("Image is not OK");
            }
        } else if let Some(label) = view.as_any().downcast_ref::<Label>()
            && !label.text.is_empty()
        {
            Self::draw_label(&frame, label, text_offset, sections);
        } else if let Some(drawing_view) = view.as_any().downcast_ref::<DrawingView>() {
            for path in drawing_view.paths().iter().rev() {
                PATH.get_mut().draw(
                    pass,
                    &clamped_frame,
                    path.buffer(),
                    path.bind(),
                    path.vertex_range(),
                    drawing_view.z_position() - UIManager::additional_z_offset(),
                );
            }
        }

        if debug_frames {
            pass.set_viewport(0.0, 0.0, window_size.width, window_size.height, 0.0, 1.0);

            for rect in frame.to_borders(2.0) {
                RECT_DRAWER
                    .get_mut()
                    .add(RectInstance::new(rect, Color::TURQUOISE, view.z_position() - 0.2));
            }
        }

        let mut text_offset = 0.0;

        let root_frame = UIManager::root_view().frame();

        for view in view.subviews().iter().rev() {
            if view.dont_hide() || view.absolute_frame().intersects(root_frame) {
                Self::draw_view(pass, view.deref(), sections, &mut text_offset, debug_frames);
            }
        }
    }

    fn draw_label<'a>(
        frame: &Rect,
        label: &'a Label,
        text_offset: &mut f32,
        sections: &mut Vec<Section<'a>>,
    ) {
        let center = frame.center();

        let margin = 16.0;

        let section = Section::default()
            .add_text(
                Text::new(&label.text)
                    .with_scale(label.text_size())
                    .with_color(label.text_color().as_slice())
                    .with_z(label.z_position() - UIManager::additional_z_offset() + *text_offset),
            )
            .with_bounds((
                frame.width() - if label.alignment.center() { 0.0 } else { margin },
                frame.height(),
            ))
            .with_layout(
                if label.multiline {
                    Layout::default_wrap()
                } else {
                    Layout::default_single_line()
                }
                .v_align(VerticalAlign::Center)
                .h_align(match label.alignment {
                    TextAlignment::Left => HorizontalAlign::Left,
                    TextAlignment::Center => HorizontalAlign::Center,
                    TextAlignment::Right => HorizontalAlign::Right,
                })
                .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
            )
            .with_screen_position((
                match label.alignment {
                    TextAlignment::Left => frame.x() + margin,
                    TextAlignment::Center => center.x,
                    TextAlignment::Right => frame.max_x() - margin,
                },
                center.y,
            ));

        *text_offset += UIManager::additional_z_offset();

        sections.push(section);
    }

    pub fn root_view_size() -> Size {
        UIManager::root_view().size()
    }
}

impl UI {
    pub async fn init_test_view<T: View + ViewTest + Default + 'static>() -> Weak<T> {
        Self::set_test_view(T::new(), 600, 600).await
    }

    pub async fn set_test_view<T: View + 'static>(view: Own<T>, width: u32, height: u32) -> Weak<T> {
        clear_state();

        App::set_window_size((width, height)).await;
        wait_for_next_frame().await;
        let view = from_main(move || {
            let weak = view.weak();
            let mut root = UIManager::root_view_weak();
            root.remove_all_subviews();
            let view = root.__add_subview_internal(view, true);
            view.place().back();
            trace!("{width} - {height}");
            weak
        })
        .await;
        wait_for_next_frame().await;
        view
    }
}
