#![allow(incomplete_features)]
#![allow(clippy::float_cmp)]
#![allow(clippy::too_many_lines)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use std::env::var;

use anyhow::Result;
use log::info;
use test_engine::{
    ui::{Container, ViewSetup},
    App,
};

use crate::{
    base::test_base_ui,
    views::{
        basic::test_base_views, complex::test_complex_views, containers::test_containers,
        helpers::test_helper_views, input::test_input_views, layout::test_layout,
    },
};

mod base;
mod level;
mod views;

#[tokio::main]
async fn main() -> Result<()> {
    App::start_with_actor(Container::new(), async {
        test_engine::ui::UIManager::set_display_touches(true);

        let cycles: u32 = var("UI_TEST_CYCLES").unwrap_or("2".to_string()).parse().unwrap();

        for i in 1..=cycles {
            test().await?;
            info!("Cycle {i}: OK");
        }

        App::stop();

        Ok(())
    })
    .await
}

async fn test() -> Result<()> {
    test_base_ui().await?;
    test_complex_views().await?;
    test_base_views().await?;
    test_layout().await?;
    test_input_views().await?;
    test_containers().await?;
    test_helper_views().await?;

    Ok(())
}
