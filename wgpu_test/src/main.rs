#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod wgpu_test_view;

use std::ops::{Deref, DerefMut};

use anyhow::Result;
use log::warn;
use test_engine::{
    assets::Assets,
    gm::{
        axis::Axis,
        flat::{IntSize, Rect, Size},
    },
    paths::git_root,
    ui::{refs::Own, Container, View, ViewAnimation, ViewData, ViewFrame, ViewLayout, ViewSubviews},
    ui_views::ImageView,
    wgpu_wrapper::{app::App, wgpu::RenderPass, wgpu_app::WGPUApp, wgpu_drawer::WGPUDrawer},
};

use crate::wgpu_test_view::WGPUTestView;

struct TEApp {
    pub(crate) root_view: Own<dyn View>,
}

impl TEApp {
    fn rescale_frame(rect: &Rect, display_scale: f32, window_size: Size) -> Rect {
        (
            rect.origin.x * display_scale,
            (window_size.height - rect.origin.y - rect.size.height) * display_scale,
            rect.size.width * display_scale,
            rect.size.height * display_scale,
        )
            .into()
    }

    fn update_view(&self, view: &mut dyn View) {
        if view.is_hidden() {
            return;
        }
        view.layout();
        view.commit_animations();
        view.calculate_absolute_frame();
        view.update();
        for view in view.subviews_mut() {
            self.update_view(view.deref_mut());
        }
    }

    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer, view: &dyn View) {
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

        let frame = Self::rescale_frame(view.absolute_frame(), 1.0, drawer.window_size);

        drawer.fill_rect(pass, &frame, view.color());

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if image_view.image.is_ok() {
                let image = image_view.image;
                let _frame = &image.size.fit_in_rect::<{ Axis::X }>(view.absolute_frame());
                // self.draw_image(
                //     &image,
                //     &UIManager::rescale_frame(frame),
                //     &image_view.tint_color,
                //     view.priority,
                //     false,
                // );
            }
        }

        for view in view.subviews() {
            if view.dont_hide() || view.absolute_frame().intersects(self.root_view.frame()) {
                self.draw(pass, drawer, view.deref())
            }
        }
    }
}

impl Default for TEApp {
    fn default() -> Self {
        let mut root_view = Container::make_root_view();
        let view = root_view.add_view::<WGPUTestView>();
        view.place().back();
        Self { root_view }
    }
}

impl App for TEApp {
    fn update(&mut self) {
        self.update_view(self.root_view.weak().deref_mut());
    }

    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer) {
        self.draw(pass, drawer, self.root_view.deref());
    }

    fn resize(&mut self, size: IntSize) {
        self.root_view.set_size(size);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Assets::init(git_root().expect("git_root()"));
    WGPUApp::start(TEApp::default(), 1200, 1200).await
}
