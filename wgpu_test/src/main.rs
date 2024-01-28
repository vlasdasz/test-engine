use anyhow::Result;
use test_engine::{assets::Assets, paths::git_root, wgpu_wrapper::app::App};

#[tokio::main]
async fn main() -> Result<()> {
    Assets::init(git_root().expect("git_root()"));
    App::start(1200, 1200).await
}
