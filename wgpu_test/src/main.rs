use std::ops::DerefMut;

use anyhow::Result;
use log::warn;
use test_engine::{
    assets::Assets,
    gl_wrapper::{path_data::DrawMode, GLWrapper},
    gm::{axis::Axis, Color},
    paths::git_root,
    ui::{
        layout::Placer, refs::Own, Container, UIManager, View, ViewAnimation, ViewData, ViewFrame,
        ViewLayout, ViewSubviews,
    },
    ui_views::ImageView,
    wgpu_wrapper::{app::App, wgpu_app::WGPUApp},
};

struct TEApp {
    pub(crate) root_view: Own<dyn View>,
}

impl TEApp {
    fn update_view(&self, view: &mut dyn View) {
        if view.is_hidden {
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

    fn draw(&self, view: &dyn View) {
        if view.is_hidden {
            return;
        }

        if view.frame().size.is_invalid() {
            warn!(
                "View has invalid frame: {}. Frame: {:?} ",
                view.label,
                view.frame()
            );
            return;
        }

        //self.fill(view.absolute_frame(), view.color(), view.priority);

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if image_view.image.is_ok() {
                let image = image_view.image;
                let frame = &image.size.fit_in_rect::<{ Axis::X }>(view.absolute_frame());
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
            if view.dont_hide || view.absolute_frame().intersects(self.root_view.frame()) {
                self.draw(view.deref())
            }
        }
    }
}

impl Default for TEApp {
    fn default() -> Self {
        let mut root_view = Own::<Container>::default();
        root_view.label = "Root view".to_string();
        let weak_root = root_view.weak_view();
        root_view.place = Placer::new(weak_root);

        // UIManager::root_view().add_subview(view).place.back();

        Self { root_view }
    }
}

impl App for TEApp {
    fn update(&mut self) {}

    fn render(&mut self) {}
}

#[tokio::main]
async fn main() -> Result<()> {
    Assets::init(git_root().expect("git_root()"));
    WGPUApp::start(TEApp::default(), 1200, 1200).await
}
