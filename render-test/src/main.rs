#![allow(incomplete_features)]
#![feature(specialization)]

use test_engine::ui::Setup;
mod occlusion;
mod path;
mod pipelines;
pub(crate) mod render;

use anyhow::Result;
use test_engine::{AppRunner, ui::Container};

use crate::render::test_render;

#[tokio::main]
async fn main() -> Result<()> {
    AppRunner::start_with_actor(Container::new(), async {
        test_engine::ui::UIManager::set_display_touches(true);

        test_render().await?;

        AppRunner::stop();

        Ok(())
    })
    .await
}
