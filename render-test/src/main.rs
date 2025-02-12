#![allow(incomplete_features)]
#![feature(specialization)]

use test_engine::ui::Setup;
mod render;

use anyhow::Result;
use test_engine::{App, ui::Container};

use crate::render::test_render;

#[tokio::main]
async fn main() -> Result<()> {
    App::start_with_actor(Container::new(), async {
        test_engine::ui::UIManager::set_display_touches(true);

        test_render().await?;

        App::stop();

        Ok(())
    })
    .await
}
