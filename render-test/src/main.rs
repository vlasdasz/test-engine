mod app;

use anyhow::Result;
use window::Window;

use crate::app::RenderApp;

#[tokio::main]
async fn main() -> Result<()> {
    dbg!("A");

    Window::start(RenderApp {}).await?;

    Ok(())
}
