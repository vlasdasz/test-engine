use std::{
    any::type_name,
    ops::{Deref, DerefMut},
    sync::Mutex,
};

use dispatch::{from_main, wait_for_next_frame};
use gm::{color::TURQUOISE, flat::Rect};
use log::{debug, trace};
use refs::{MainLock, Own, Weak};
use render::{
    UIGradientPipeline, UIImageRectPipepeline, UIPathPipeline, UIRectPipepeline,
    data::{RectView, UIGradientInstance, UIImageInstance, UIRectInstance},
};
use ui::{
    DrawingView, HasText, ImageView, Label, Setup, TextAlignment, UIManager, View, ViewAnimation, ViewData,
    ViewFrame, ViewLayout, ViewSubviews, ViewTest,
};
use wgpu::RenderPass;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, Section, Text, VerticalAlign};
use window::{Font, Window};

use crate::{AppRunner, ui::ui_test::state::clear_state};

static RECT_DRAWER: MainLock<UIRectPipepeline> = MainLock::new();
static GRADIENT_DRAWER: MainLock<UIGradientPipeline> = MainLock::new();
static IMAGE_RECT_DRAWER: MainLock<UIImageRectPipepeline> = MainLock::new();
static UI_PATH_DRAWER: MainLock<UIPathPipeline> = MainLock::new();
pub static TEST_NAME: Mutex<String> = Mutex::new(String::new());

pub struct UI;

impl UI {
    pub(crate) fn update() {
        Self::update_view(UIManager::root_view().deref_mut());
        if let Some(debug_view) = UIManager::debug_view() {
            Self::update_view(debug_view);
        }
    }

    pub(crate) fn draw(pass: &mut RenderPass) {
        let mut sections: Vec<Section> = vec![];
        let debug_frames = UIManager::should_draw_debug_frames();
        let scale = UIManager::scale();
        Self::draw_view(
            pass,
            UIManager::root_view_static(),
            &mut sections,
            debug_frames,
            scale,
        );
        if let Some(debug_view) = UIManager::debug_view() {
            Self::draw_view(pass, debug_view, &mut sections, debug_frames, scale);
        }

        RECT_DRAWER.get_mut().draw(
            pass,
            RectView {
                resolution: UIManager::window_resolution(),
            },
        );

        IMAGE_RECT_DRAWER.get_mut().draw(
            pass,
            RectView {
                resolution: UIManager::window_resolution(),
            },
        );

        GRADIENT_DRAWER.get_mut().draw(
            pass,
            RectView {
                resolution: UIManager::window_resolution(),
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
        debug_frames: bool,
        scale: f32,
    ) {
        if view.is_hidden() {
            return;
        }

        let frame = *view.absolute_frame();

        if frame.size.has_no_area() {
            // warn!(
            //     "View has invalid frame: {}. Frame: {:?} ",
            //     view.label(),
            //     view.frame()
            // );
            return;
        }

        view.before_render(pass);

        if view.end_gradient_color().a > 0.0 {
            GRADIENT_DRAWER.get_mut().add(UIGradientInstance {
                position: frame.origin,
                size: frame.size,
                start_color: *view.color(),
                end_color: *view.end_gradient_color(),
                corner_radius: view.corner_radius(),
                z_position: view.z_position(),
                scale,
            });
        } else if view.color().a > 0.0 {
            RECT_DRAWER.get_mut().add(UIRectInstance::new(
                frame,
                *view.color(),
                view.corner_radius(),
                view.z_position(),
                scale,
            ));
        }

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if image_view.image().is_ok() {
                let image = image_view.image();

                IMAGE_RECT_DRAWER.get_mut().add_with_image(
                    UIImageInstance::new(
                        image_view.image_frame(),
                        view.corner_radius(),
                        view.z_position(),
                        image_view.flip_x,
                        image_view.flip_y,
                        scale,
                    ),
                    image,
                );
            }
        } else if let Some(label) = view.as_any().downcast_ref::<Label>()
            && !label.text.is_empty()
        {
            Self::draw_label(&frame, label, sections, scale);
        } else if let Some(drawing_view) = view.as_any().downcast_ref::<DrawingView>() {
            for path in drawing_view.paths().iter().rev() {
                UI_PATH_DRAWER.get_mut().draw(
                    pass,
                    path.buffer(),
                    path.uniform_bind(),
                    path.vertex_range(),
                    drawing_view.z_position() - UIManager::additional_z_offset(),
                );
            }
        }

        if debug_frames {
            for rect in frame.to_borders(2.0) {
                RECT_DRAWER.get_mut().add(UIRectInstance::new(
                    rect,
                    TURQUOISE,
                    0.0,
                    view.z_position() - 0.2,
                    scale,
                ));
            }
        }

        let root_frame = UIManager::root_view_static().frame();

        for view in view.subviews() {
            if view.dont_hide() || view.absolute_frame().intersects(root_frame) {
                Self::draw_view(pass, view.deref(), sections, debug_frames, scale);
            }
        }
    }

    fn draw_label<'a>(frame: &Rect, label: &'a Label, sections: &mut Vec<Section<'a>>, scale: f32) {
        let frame = frame * scale;

        let center = frame.center();

        let margin = 16.0;

        let section = Section::default()
            .add_text(
                Text::new(&label.text)
                    .with_scale(label.text_size() * scale)
                    .with_color(label.text_color().as_slice())
                    .with_z(label.z_position() - UIManager::additional_z_offset()),
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

        sections.push(section);
    }
}

impl UI {
    pub async fn reload_test_view<T: View + ViewTest + Default + 'static>() -> Weak<T> {
        Self::set_test_view(T::new(), 600, 600, false, get_test_name::<T>()).await
    }

    pub async fn init_test_view<T: View + ViewTest + Default + 'static>() -> Weak<T> {
        Self::set_test_view(T::new(), 600, 600, true, get_test_name::<T>()).await
    }

    pub async fn set_test_view<T: View + 'static>(
        view: Own<T>,
        width: u32,
        height: u32,
        test_start: bool,
        new_test_name: String,
    ) -> Weak<T> {
        let test_name = TEST_NAME.lock().unwrap().clone();

        if !test_name.is_empty() && test_start {
            debug!("{test_name}: OK");
        }

        TEST_NAME.lock().unwrap().clone_from(&new_test_name);

        debug!("{new_test_name}: Started");

        clear_state();

        AppRunner::set_window_size((width, height)).await;
        wait_for_next_frame().await;
        let view = from_main(move || {
            let weak = view.weak();
            let mut root = UIManager::root_view();
            root.clear_root();
            let view = root.add_subview_to_root(view);
            view.place().back();
            trace!("{width} - {height}");
            weak
        })
        .await;
        wait_for_next_frame().await;
        view
    }
}

fn get_test_name<T>() -> String {
    let input = type_name::<T>();

    let last_part = input.split("::").last().unwrap();

    last_part
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && c.is_uppercase() {
                format!(" {}", c.to_ascii_lowercase())
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}
