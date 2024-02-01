use anyhow::Result;
use test_engine::{
    assets::Assets,
    paths::git_root,
    ui::{layout::Placer, refs::Own, Container, View},
    wgpu_wrapper::{app::App, wgpu_app::WGPUApp},
};

struct TEApp {
    pub(crate) root_view: Own<dyn View>,
}

impl Default for TEApp {
    fn default() -> Self {
        let mut root_view = Own::<Container>::default();
        root_view.label = "Root view".to_string();
        let weak_root = root_view.weak_view();
        root_view.place = Placer::new(weak_root);

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
