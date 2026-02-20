#![allow(incomplete_features)]
#![allow(clippy::float_cmp)]
#![allow(clippy::too_many_lines)]
#![feature(stmt_expr_attributes)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use std::{collections::BTreeMap, env::var};

use anyhow::{Result, bail};
use clap::Parser;
use log::info;
use test_engine::{
    AppRunner,
    dispatch::from_main,
    ui::{Label, UIManager},
};

use crate::inspect::test_inspect;
use crate::{
    base::test_base_ui,
    views::{
        basic::test_base_views,
        complex::test_complex_views,
        containers::test_containers,
        helpers::test_helper_views,
        images::test_image_views,
        // input::test_input_views,
        layout::test_layout,
    },
};

mod base;
mod inspect;
mod level;
mod views;

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    test_name: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    AppRunner::start_with_actor(async {
        Label::set_default_text_size(32);
        UIManager::set_display_touches(true);

        from_main(|| {
            UIManager::override_scale(1.0);
        });

        let mut tests: BTreeMap<_, _> = test_game::UI_TESTS.lock().clone();

        tests.append(&mut test_engine::UI_TESTS.lock().clone());

        if let Some(test_name) = args.test_name {
            let test = match tests.get(&test_name) {
                Some(test) => test,
                None => {
                    println!("Test: {test_name} not found");
                    AppRunner::stop();
                    bail!("Test: {test_name} not found");
                }
            };
            test()?;
            AppRunner::stop();
            return Ok(());
        }

        for (_name, test) in tests.into_iter() {
            test()?;
        }

        let cycles: u32 = var("UI_TEST_CYCLES").unwrap_or("2".to_string()).parse().unwrap();

        for i in 1..=cycles {
            test().await?;
            info!("Cycle {i}: OK");
        }

        AppRunner::stop();

        Ok(())
    })?;

    Ok(())
}

async fn test() -> Result<()> {
    test_base_ui().await?;
    test_base_views().await?;
    test_inspect().await?;
    test_layout().await?;
    test_complex_views().await?;
    test_image_views().await?;
    test_containers().await?;
    // test_input_views().await?;
    test_helper_views().await?;

    Ok(())
}
